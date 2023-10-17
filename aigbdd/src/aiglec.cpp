//////////////////////////////////////////////////////////////////////////////////
//
// Company: Universidade Federal do Rio Grande do Sul
// Engineer: Rafael de Oliveira Calçada
//
// Create Date: 01.12.2019 14:24:35
// Description: Checks the logical equivalence between 2 AIGs
//
// Revision:
// Revision 0.01 - File created
//
//////////////////////////////////////////////////////////////////////////////////

#include <iostream>
#include <fstream>
#include <stack>
#include <string>
#include <unordered_map>
#include "structures.h"

/* GLOBALS
******************************************************************************/

// AIG data
vertex* vertices = NULL;
int* inputs = NULL;
int* outputs = NULL;
int M = 0;
int I = 0;
int L = 0;
int O = 0;
int A = 0;

// Auxiliary stack
stack<int, vector<int>>* stk;

// MK function. See ANDERSEN, H. R. An introduction to binary decision diagrams.
// In: Lecture Notes for Efficient Algorithms and Programs, 1999. p. 15
int mk(vector<triple>& T, unordered_map<triple_key,int>& H, int i, int l, int h)
{
	if(l == h) return l;    
	else
	{   
		triple t = {i, l, h};
		unordered_map<triple_key,int>::iterator it;
		it = H.find(keygen_h(t));
		if(it != H.end()) return it->second;
		else
		{
			int u = add(T, i, l, h);
			insert_h(H, i, l, h, u);
			return u;
		}        
	}
}

// APP function. See ANDERSEN, H. R. An introduction to binary decision diagrams.
// In: Lecture Notes for Efficient Algorithms and Programs, 1999. p. 19
// This is a specialization of APP that applies the AND between 2 BDDs
int app_and(vector<triple>& T, unordered_map<triple_key,int>& H, unordered_map<duo_key,int>& G, int u1, int u2)
{
	duo d = {u1, u2};
	unordered_map<duo_key,int>::iterator it;
	it = G.find(keygen_g(d));
	int u = -1;
	if(it != G.end()) return it->second;
	else if((u1 == 0 || u1 == 1) && (u2 == 0 || u2 == 1))
	{
		if(u1 == 1 && u2 == 1) u = 1;
		else u = 0;
	}
	else if(T[u1].var == T[u2].var)
	{
		u = mk(T, H, T[u1].var, app_and(T, H, G, T[u1].low, T[u2].low), app_and(T, H, G, T[u1].high, T[u2].high));
	}
	else if(T[u1].var < T[u2].var)
	{
		u = mk(T, H, T[u1].var, app_and(T, H, G, T[u1].low, u2), app_and(T, H, G, T[u1].high, u2));
	}
	else if(T[u1].var > T[u2].var)
	{
		u = mk(T, H, T[u2].var, app_and(T, H, G, u1, T[u2].low), app_and(T, H, G, u1, T[u2].high));
	}
	insert_g(G, u1, u2, u);
	return u;
}

// APP function. See ANDERSEN, H. R. An introduction to binary decision diagrams.
// In: Lecture Notes for Efficient Algorithms and Programs, 1999. p. 19
// This is a specialization of APP that applies the NAND between 2 BDDs
int app_nand(vector<triple>& T, unordered_map<triple_key,int>& H, unordered_map<duo_key,int>& G, int u1, int u2)
{
	duo d = {u1, u2};
	unordered_map<duo_key,int>::iterator it;
	it = G.find(keygen_g(d));
	int u = -1;
	if(it != G.end()) return it->second;
	else if((u1 == 0 || u1 == 1) && (u2 == 0 || u2 == 1))
	{
		if(u1 == 1 && u2 == 1) u = 0;
		else u = 1;
	}
	else if(T[u1].var == T[u2].var)
	{
		u = mk(T, H, T[u1].var, app_nand(T, H, G, T[u1].low, T[u2].low), app_nand(T, H, G, T[u1].high, T[u2].high));
	}
	else if(T[u1].var < T[u2].var)
	{
		u = mk(T, H, T[u1].var, app_nand(T, H, G, T[u1].low, u2), app_nand(T, H, G, T[u1].high, u2));
	}
	else if(T[u1].var > T[u2].var)
	{
		u = mk(T, H, T[u2].var, app_nand(T, H, G, u1, T[u2].low), app_nand(T, H, G, u1, T[u2].high));
	}
	insert_g(G, u1, u2, u);
	return u;
}

// APPLY function. See ANDERSEN, H. R. An introduction to binary decision diagrams.
// In: Lecture Notes for Efficient Algorithms and Programs, 1999. p. 19
// This is a specialization of APPLY that applies AND between 2 BDDs
int apply_and(vector<triple>& T, unordered_map<triple_key,int>& H, int u1, int u2)
{
	unordered_map<duo_key,int> G;
	init_g(G);
	return app_and(T, H, G, u1, u2);
}

// APPLY function. See ANDERSEN, H. R. An introduction to binary decision diagrams.
// In: Lecture Notes for Efficient Algorithms and Programs, 1999. p. 19
// This is a specialization of APPLY that applies AND between 2 BDDs
int apply_nand(vector<triple>& T, unordered_map<triple_key,int>& H, int u1, int u2)
{
	unordered_map<duo_key,int> G;
	init_g(G);
	return app_nand(T, H, G, u1, u2);
}

void create_graph(char* filename, int& M, int& I, int& L, int& O, int& A, vertex*& vertices, int*& outputs);
void print_aig(string filename, vertex*& vertices, int* outputs, int& M, int& I, int& O);
void print_bdd(string filename, vector<triple>& T, int bdd_root, int output_index);
string bdd_signature(vector<triple>& T, int bdd_root);

/* MAIN FUNCTION: TRANSFORMS AN AIG INTO A ROBDD
******************************************************************************/
int main(int argc, char* argv[])
{    

	bool genviz = false;
	vector<triple> T;
	unordered_map<triple_key,int> H;
	stk = new stack<int, std::vector<int>>;
	vector<string> aig1signatures;
	vector<string> aig2signatures;

	// read the path of the AIG file
	if(argc < 3)
	{
		cout << "\nUsage:\n" << argv[0] << " [aig-file-1] [aig-file-2] [-v]" << endl;
		cout << "\nThe -v is optional. If used, generates visualization files for the AIGs and their BDDs.\n" << endl;
		return EXIT_FAILURE;
	}
	if(argc == 4 && string(argv[3]).compare("-v") == 0) genviz = true;

	/* PROCESS THE 1ST AIG
	 ******************************************************************************/

	char* aigfile1 = argv[1];
	int substrlen = 0;
	string outprefix1 = "";
	if(string(argv[1]).find_last_of("/") != -1)
	{
		substrlen = string(argv[1]).find_last_of(".") - string(argv[1]).find_last_of("/");
		outprefix1 = string(argv[1]).substr(string(argv[1]).find_last_of("/")+1,substrlen-1);
	}
	else
	{
		substrlen = string(argv[1]).find_last_of(".");
		outprefix1 = string(argv[1]).substr(0,substrlen);
	}

	create_graph(aigfile1, M, I, L, O, A, vertices, outputs);

	if(genviz) print_aig(outprefix1, vertices, outputs, M, I, O);

	for(int i = 0; i < O; i++)
	{

		int vertex_index = (outputs[i] >> 1) - 1;
		vertex* v = &vertices[vertex_index];
		init_t(T, I+1);
		init_h(H);	
		// creates a ROBDD for each input
		for(int j = 0; j < I; j++)
		{
			vertex* w = &vertices[j];
			w->bdd = mk(T, H, j+1, 0, 1);
		}	

		while(v != NULL)
		{
			int left_index = v->left / 2 - 1;
			int right_index = v->right / 2 - 1;
			vertex* left = &vertices[left_index];
			vertex* right = &vertices[right_index];
			if(left != NULL && left->bdd == -1)
			{
				stk->push(vertex_index);
				v = left;
				vertex_index = left_index;
			}
			else if(right != NULL && right->bdd == -1)
			{
				stk->push(vertex_index);
				v = right;
				vertex_index = right_index;
			}
			else
			{   
				int bdd_left = left->bdd;
				int bdd_right = right->bdd;
				if(v->left & 1 == 1) bdd_left = apply_nand(T, H, bdd_left, bdd_left);
				if(v->right & 1 == 1) bdd_right = apply_nand(T, H, bdd_right, bdd_right);
				v->bdd = apply_and(T, H, bdd_right, bdd_left);
				if(stk->empty()) v = NULL;
				else
				{
					vertex_index = stk->top();
					stk->pop();
					v = &vertices[vertex_index];
				}
			}
		}

		vertex_index = (outputs[i] >> 1) - 1;
		v = &vertices[vertex_index];
		if(outputs[i] & 1 == 1) v->bdd = apply_nand(T, H, v->bdd, v->bdd);
		if(genviz) print_bdd(outprefix1, T, v->bdd, i);
		aig1signatures.push_back(bdd_signature(T,v->bdd));
		// erase the results for the next iteration
		for(int j = I; j < I+A; j++) 
		{
			vertex* w = &vertices[j];
			w->bdd = -1;
		}

	}

	/* PROCESS THE 2ND AIG
	 ******************************************************************************/

	free(vertices);
	free(outputs);

	char* aigfile2 = argv[2];
	substrlen = 0;
	string outprefix2 = "";
	if(string(argv[2]).find_last_of("/") != -1)
	{
		substrlen = string(argv[2]).find_last_of(".") - string(argv[2]).find_last_of("/");
		outprefix2 = string(argv[2]).substr(string(argv[2]).find_last_of("/")+1,substrlen-1);
	}
	else
	{
		substrlen = string(argv[2]).find_last_of(".");
		outprefix2 = string(argv[2]).substr(0,substrlen);
	}

	int savedI = I;
	int savedO = O;
	create_graph(aigfile2, M, I, L, O, A, vertices, outputs);

	if(savedI != I || savedO != O)
	{
		cout << "RESULT: The AIGs are no logical equivalent because they do not have the same number of inputs and outputs." << endl;
		return EXIT_SUCCESS;
	}

	if(genviz) print_aig(outprefix2, vertices, outputs, M, I, O);

	for(int i = 0; i < O; i++)
	{

		int vertex_index = (outputs[i] >> 1) - 1;
		vertex* v = &vertices[vertex_index];
		init_t(T, I+1);
		init_h(H);	
		// creates a ROBDD for each input
		for(int j = 0; j < I; j++)
		{
			vertex* w = &vertices[j];
			w->bdd = mk(T, H, j+1, 0, 1);
		}	

		while(v != NULL)
		{
			int left_index = v->left / 2 - 1;
			int right_index = v->right / 2 - 1;
			vertex* left = &vertices[left_index];
			vertex* right = &vertices[right_index];
			if(left != NULL && left->bdd == -1)
			{
				stk->push(vertex_index);
				v = left;
				vertex_index = left_index;
			}
			else if(right != NULL && right->bdd == -1)
			{
				stk->push(vertex_index);
				v = right;
				vertex_index = right_index;
			}
			else
			{   
				int bdd_left = left->bdd;
				int bdd_right = right->bdd;
				if(v->left & 1 == 1) bdd_left = apply_nand(T, H, bdd_left, bdd_left);
				if(v->right & 1 == 1) bdd_right = apply_nand(T, H, bdd_right, bdd_right);
				v->bdd = apply_and(T, H, bdd_right, bdd_left);
				if(stk->empty()) v = NULL;
				else
				{
					vertex_index = stk->top();
					stk->pop();
					v = &vertices[vertex_index];
				}
			}
		}

		vertex_index = (outputs[i] >> 1) - 1;
		v = &vertices[vertex_index];
		if(outputs[i] & 1 == 1) v->bdd = apply_nand(T, H, v->bdd, v->bdd);
		if(genviz) print_bdd(outprefix2, T, v->bdd, i);
		aig2signatures.push_back(bdd_signature(T,v->bdd));
		// erase the results for the next iteration
		for(int j = I; j < I+A; j++) 
		{
			vertex* w = &vertices[j];
			w->bdd = -1;
		}

	}

	bool equivalents = true;
	for(int i = 0; i < O; i++)
	{
		if(aig1signatures[i].compare(aig2signatures[i]) != 0)
		{
			equivalents = false;
			break;
		}
	}
	if(equivalents) cout << "RESULT: The AIGs are logical equivalents." << endl;
	else cout << "RESULT: The AIGs are NOT logical equivalents." << endl;

	return EXIT_SUCCESS;

}

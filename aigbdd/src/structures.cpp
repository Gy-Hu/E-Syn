//////////////////////////////////////////////////////////////////////////////////
//
// Company: Universidade Federal do Rio Grande do Sul
// Engineer: Rafael de Oliveira Calçada
//
// Create Date: 01.12.2019 14:24:35
// Description: Procedures that manipulate data structures
//
// Revision:
// Revision 0.01 - File created
//
//////////////////////////////////////////////////////////////////////////////////

#include "structures.h"
#include <iostream>
#include <fstream>
#include <string>
#include <stack>
#include <cstdlib>

// Pairing function that creates a bijection between a triple
// and an integer number (the key)
triple_key keygen_h(triple& t)
{
    int i = t.var;
    int v0 = t.low;
    int v1 = t.high;
    int pair = ( ( (v0 + v1) * (v0 + v1 + 1) ) / 2 ) + v0;
    int key = ( ( (i + pair) * (i + pair + 1) ) / 2 ) + i;
    return key;
}

// Pairing function that creates a bijection between a duo
// and an integer number (the key)
duo_key keygen_g(duo& d)
{
    int u1 = d.u1;
    int u2 = d.u2;
    int key = ( ( (u1 + u2) * (u1 + u2 + 1) ) / 2 ) + u1;
    return key;
}

void init_t(vector<triple>& T, int max_index)
{
    T.clear();
    triple zero = {max_index, -1, -1};
    triple one = {max_index+1, -1, -1};
    T.push_back(zero);
    T.push_back(one);
}

int add(vector<triple>& T, int i, int l, int h)
{
    int u = T.size();
    triple t = {i, l, h};
    T.push_back(t);
    return u;
}

void init_h(unordered_map<triple_key,int>& H)
{
    H.clear();
}

void init_g(unordered_map<duo_key,int>& G)
{
    G.clear();
}

void insert_h(unordered_map<triple_key, int>& H, int i, int l, int h, int u)
{
    triple t = {i, l, h};
    unordered_map<triple_key,int>::iterator it;
    it = H.find(keygen_h(t));
    if(it != H.end()) it->second = u;
    else H.insert(pair<triple_key,int>(keygen_h(t),u));
}

void insert_g(unordered_map<duo_key, int>& G, int u1, int u2, int u)
{
    duo d = {u1, u2};
    unordered_map<duo_key,int>::iterator it;
    it = G.find(keygen_g(d));
    if(it != G.end()) it->second = u;
    else G.insert(pair<duo_key,int>(keygen_g(d),u));
}

void print_aig(string filename, vertex*& vertices, int* outputs, int& M, int& I, int& O)
{

    ofstream aigviz;
    string filepath(filename);
    filepath += "_aig";
    aigviz.open(filepath + ".dot", fstream::out | fstream::trunc);

    if(!aigviz.is_open())
    {
        cout << "ERROR. Cannot open or create the output file." << endl;
        exit(EXIT_FAILURE);
    }

    unordered_map<int, int> outs;
    for(int i = 0; i < O; i++) outs.insert(pair<int,int>((outputs[i] >> 1) - 1, outputs[i]));

    aigviz << "strict digraph {" << endl;

    for(int i = 0; i < M; i++)
	{        
        vertex* v = &vertices[i];
        int vertex_index = i;
		int left_index = v->left / 2 - 1;
		int right_index = v->right / 2 - 1;
		if(left_index != -1)
		{
            aigviz << "  " << vertex_index << " -> " << left_index;
            if(v->left & 1 == 1) aigviz << " [style=dashed dir=back];" << endl;
            else aigviz << " [dir=back];" << endl;
		}
		if(right_index != -1)
		{
			aigviz << "  " << vertex_index << " -> " << right_index;
            if(v->right & 1 == 1) aigviz << " [style=dashed dir=back];" << endl;
            else aigviz << " [dir=back];" << endl;
		}
        unordered_map<int,int>::iterator it;
        it = outs.find(vertex_index);
        if(it != outs.end()) aigviz << "  " << vertex_index << " [label=\"" << it->second << "\" style=filled fillcolor=khaki];" << endl;
	    else if(vertex_index >= I) aigviz << "  " << vertex_index << " [label=\"" << (vertex_index+1)*2 << "\"];" << endl;
        else aigviz << "  subgraph cluster0 { style=invis; " << vertex_index << " [shape=square label=\"" << (vertex_index+1)*2 << "\"]; }" << endl;
	}

    aigviz << "}" << endl;
    aigviz.close();

    string command = "dot -Tpng " + filepath + ".dot > " + filepath + ".png";
    int retcode = system(command.c_str());
    if(retcode != 0) cout << "Error: the AIG visualization could not be generated." << endl;
    else
    {
        string del_command = "rm " + filepath + ".dot";
        int delretcode = system(del_command.c_str());
        if(delretcode != 0) cout << "Error: could not delete the AIG .dot file." << endl;
    }

}

void print_bdd(string filename, vector<triple>& T, int bdd_root, int output_index)
{

    ofstream bddviz;
    string filepath(filename);
    filepath += "_bdd_" + to_string(output_index);
    bddviz.open(filepath + ".dot", fstream::out | fstream::trunc);

    if(!bddviz.is_open())
    {
        cout << "ERROR. Cannot open or create the output file." << endl;
        exit(EXIT_FAILURE);
    }

    bddviz << "strict digraph {" << endl;    

    bddviz << "  subgraph cluster0 { style=invis; 0 [shape=square label=\"0\"]; }" << endl;
    bddviz << "  subgraph cluster0 { style=invis; 1 [shape=square label=\"1\"]; }" << endl;

    stack<int, vector<int>> stk;
    stk.push(bdd_root);

    while(!stk.empty())
    {
        int u = stk.top();
        stk.pop();
        if(T[u].low > 1) stk.push(T[u].low);
        if(T[u].high > 1) stk.push(T[u].high);
        if(u > 1) bddviz << "  " << u << " [label=\"" << T[u].var * 2 << "\"];" << endl;
        else if(u == 1) bddviz << "  " << u << " [label=\"1\" style=filled fillcolor=khaki];" << endl;
        else if(u == 0) bddviz << "  " << u << " [label=\"0\" style=filled fillcolor=khaki];" << endl;
        if(T[u].low != -1) bddviz << "  " << u << " -> " << T[u].low << " [style=dashed];" << endl;
        if(T[u].high != -1) bddviz << "  " << u << " -> " << T[u].high << ";" << endl;
    }

    bddviz << "}" << endl;
    bddviz.close();

    string command = "dot -Tpng " + filepath + ".dot > " + filepath + ".png";
    int retcode = system(command.c_str());
    if(retcode != 0) cout << "Error: the BDD visualization could not be generated." << endl;
    else
    {
        string del_command = "rm " + filepath + ".dot";
        int delretcode = system(del_command.c_str());
        if(delretcode != 0) cout << "Error: could not delete the BDD .dot file." << endl;
    }

}

string bdd_signature(vector<triple>& T, int bdd_root)
{

    string signature = "";
    stack<int, vector<int>> stk;
    stk.push(bdd_root);

    while(!stk.empty())
    {
        int u = stk.top();
        stk.pop();
        signature += "n" + to_string(T[u].var);
        signature += "f1" + to_string(T[T[u].low].var);
        signature += "f2" + to_string(T[T[u].high].var);
        if(T[u].low > 1) stk.push(T[u].low);
        if(T[u].high > 1) stk.push(T[u].high);
    }

    return signature;

}
//////////////////////////////////////////////////////////////////////////////////
//
// Company: Universidade Federal do Rio Grande do Sul
// Engineer: Rafael de Oliveira Calçada
//
// Create Date: 19.09.2019 16:12:57
// Description: Read an AIG file (binary or ASCII)
//
// Revision:
// Revision 0.01 - File created
//
//////////////////////////////////////////////////////////////////////////////////

#include <iostream>
#include <fstream>
#include <cstring>
#include "structures.h"
using namespace std;

// get a char from a file in the AIGER binary format
unsigned char getnoneofch(ifstream& input_file)
{
	int ch = input_file.get();
	if(ch != EOF) return ch;
	cerr << "*** decode: unexpected EOF" << endl;
	exit(-1);
}

// decodes a delta encoding from a file in the AIGER binary format
unsigned int decode(ifstream& input_file)
{
	unsigned x = 0, i = 0;
	unsigned char ch;
	while ((ch = getnoneofch(input_file)) & 0x80)
	{
		x |= (ch & 0x7f) << (7 * i++);
	}
	return x | (ch << (7 * i));
}

// process the file in the ASCII format
void process_ascii_format(ifstream& input_file, int& M, int& I, int& L, int& O, int& A, vertex*& vertices, int*& outputs)
{
	char* token;
	char buffer[256];
	// check for latches
	if(L != 0)
	{
		cerr << "This graph contains latches. The current version of this implementation do not support them." << endl;
		exit(-1);
	}

	// integrity check #1
	if(M != I + L + A)
	{
		cerr << "This graph is invalid. M != I + L + A." << endl;
		exit(-1);
	}

	// Memory allocation
	outputs = new int[O];
	vertices = new vertex[M];

	// initialization of output list
	for(int i = 0; i < O; i++) outputs[i] = -1;

	// creates the input vertices
	for(int i = 0; i < I; i++)
	{
		input_file.getline(buffer, sizeof(buffer));
		if(strlen(buffer) < 1)
		{
			cerr << "The input file reached the end before expected." << endl;
			exit(-1);
		}
		token = strtok(buffer, " ");
		int label = atoi(token);

		// integrity check #2
		if(label < 0)
		{
			cerr << "The graph contains an invalid (negative) input index: " << label << "." << endl;
			exit(-1);
		}

		// integrity check #4
		if(label != ((i+1)*2))
		{
			cerr << "The AIG format states that the label of an input must be twice its index, but the input with index " << i+1 << " has the label " << label << "." << endl;
			exit(-1);
		}

		// if reached here, everything is OK, so set the value of the incoming edges
		// to -1 (indicating no incoming edges) and fanout to 0
		vertices[i].left = -1;
		vertices[i].right = -1;
		vertices[i].bdd = -1;
	}

	// save the label of the output vertices
	for(int i = 0; i < O; i++)
	{
		input_file.getline(buffer, sizeof(buffer));
		if(strlen(buffer) < 1)
		{
			cerr << "The input file reached the end before expected." << endl;
			exit(-1);
		}
		token = strtok(buffer, " ");
		int label = atoi(token);

		// integrity check #5
		if(label < 0)
		{
			cerr << "The graph contains an invalid (negative) output index: " << label << "." << endl;
			exit(-1);
		}

		// integrity check #6
		for(int j = 0; j < O; j++)
			if(outputs[j] == label)
			{
				cerr << "The graph contains an output declared twice: " << label << "." << endl;
				exit(-1);
			}

		// if reached here, everything is OK, so adds the label in the outputs list
		outputs[i] = label;
	}

	// creates the vertices and its edges
	for(int i = 0; i < A; i++)
	{
		input_file.getline(buffer, sizeof(buffer));
		if(strlen(buffer) < 1)
		{
			cerr << "The input file reached the end before expected." << endl;
			exit(-1);
		}
		token = strtok(buffer, " ");
		int label = atoi(token);

		// integrity check #7
		if(label < 0)
		{
			cerr << "The graph contains an invalid (negative) vertex index: " << label << "." << endl;
			exit(-1);
		}

		// integrity check #8
		if(label != ((i+I+1)*2))
		{
			cerr << "The AIG format states that the label of a vertex must be twice its index, but the vertex with index " << i+1 << " has the label " << label << "." << endl;
			exit(-1);
		}
		token = strtok(NULL, " ");
		int rhs0 = atoi(token);
		token = strtok(NULL, " ");
		int rhs1 = atoi(token);

		// integrity check #9
		if(rhs0 < rhs1)
		{
			cerr << "The AIG format states that the label of the first input of a vertex must be greater than the second." << endl;
			cerr << "Found rhs0=" << rhs0 << " and rhs1=" << rhs1 << " for the label " << label << "." << endl;
			exit(-1);			
		}

		// integrity check #10
		if(rhs0 < 0 || rhs1 < 0)
		{
			cerr << "The vertex has an invalid value for its inputs." << endl;
			cerr << "Found rhs0=" << rhs0 << " and rhs1=" << rhs1 << " for the label " << label << "." << endl;
			exit(-1);			
		}

		// integrity check #11
		if(label <= rhs0 || label <= rhs1)
		{
			cerr << "The AIG format states that the label must be greater than the value of its inputs." << endl;
			cerr << "Found rhs0=" << rhs0 << " and rhs1=" << rhs1 << " for the label " << label << "." << endl;
			exit(-1);			
		}

		// if reached here, everything is OK, so adds the vertex into the list, creates its edges, and updates the fanout of the child vertices
		vertices[i+I].bdd = -1;
		vertices[i+I].left = rhs0;
		vertices[i+I].right = rhs1;

	}

}

// process the file in the binary format
void process_binary_format(ifstream& input_file, int& M, int& I, int& L, int& O, int& A, vertex*& vertices, int*& outputs)
{
	char* token;
	char buffer[256];

	// check for latches
	if(L != 0)
	{
		cerr << "This graph contains latches. The current version of this implementation do not support them." << endl;
		exit(-1);
	}

	// integrity check #1
	if(M != I + L + A)
	{
		cerr << "This graph is invalid. M != I + L + A." << endl;
		exit(-1);
	}

	// OK. Now we're ready for memory allocation
	outputs = new int[O];
	vertices = new vertex[M];

	// initialization of output list
	for(int i = 0; i < O; i++) outputs[i] = -1;

	// creates the input vertices
	for(int i = 0; i < I; i++)
	{
		vertices[i].left = -1;
		vertices[i].right = -1;
		vertices[i].bdd = -1;
	}

	// save the label of the output vertices
	for(int i = 0; i < O; i++)
	{
		input_file.getline(buffer, sizeof(buffer));
		if(strlen(buffer) < 1)
		{
			cerr << "The input file reached the end before expected." << endl;
			exit(-1);
		}
		token = strtok(buffer, " ");
		int label = atoi(token);

		// integrity check #5
		if(label < 0)
		{
			cerr << "The graph contains an invalid (negative) output index: " << label << "." << endl;
			exit(-1);
		}

		// integrity check #6
		for(int j = 0; j < O; j++)
			if(outputs[j] == label)
			{
				cerr << "The graph contains an output declared twice: " << label << "." << endl;
				exit(-1);
			}

		// if reached here, everything is OK, so adds the label in the outputs list
		outputs[i] = label;
	}

	// creates the vertices and its edges
	for(int i = 0; i < A; i++)
	{
		
		unsigned int delta0 = decode(input_file);
		unsigned int delta1 = decode(input_file);

		int label = (I+i+1)*2;
		int rhs0 = label - delta0;
		int rhs1 = rhs0 - delta1;

		// integrity check #7
		if(label < 0)
		{
			cerr << "The graph contains an invalid (negative) vertex index: " << label << "." << endl;
			exit(-1);
		}

		// integrity check #8
		if(label != ((i+I+1)*2))
		{
			cerr << "The AIG format states that the label of a vertex must be twice its index, but the vertex with index " << i+1 << " has the label " << label << "." << endl;
			exit(-1);
		}

		// integrity check #9
		if(rhs0 < rhs1)
		{
			cerr << "The AIG format states that the label of the first input of a vertex must be greater than the second." << endl;
			cerr << "Found rhs0=" << rhs0 << " and rhs1=" << rhs1 << " for the label " << label << "." << endl;
			exit(-1);			
		}

		// integrity check #10
		if(rhs0 < 0 || rhs1 < 0)
		{
			cerr << "The vertex has an invalid value for its inputs." << endl;
			cerr << "Found rhs0=" << rhs0 << " and rhs1=" << rhs1 << " for the label " << label << "." << endl;
			exit(-1);			
		}

		// integrity check #11
		if(label <= rhs0 || label <= rhs1)
		{
			cerr << "The AIG format states that the label must be greater than the value of its inputs." << endl;
			cerr << "Found rhs0=" << rhs0 << " and rhs1=" << rhs1 << " for the label " << label << "." << endl;
			exit(-1);			
		}

		// if reached here, everything is OK, so adds the vertex into in the list, creates its edges, and updates the fanout of the child vertices
		vertices[i+I].bdd = -1;		
		vertices[i+I].left = rhs0;
		vertices[i+I].right = rhs1;

	}

}

// creates the graph used by the main function
void create_graph(char* filename, int& M, int& I, int& L, int& O, int& A, vertex*& vertices, int*& outputs)
{

	// opens the input file
	ifstream input_file;
	input_file.open(filename, ios::binary | ios::in);

	if(!input_file.is_open())
	{
		cerr << "Failed to open the input file." << endl;
		exit(-1);
	}

	// process the 1st line
	char buffer[256];
	buffer[0] = '\0';
	input_file.getline(buffer, sizeof(buffer));

	// split into tokens, saving the values in variables
	char* token = strtok(buffer," ");
	token = strtok(NULL, " ");
	M = atoi(token);
	token = strtok(NULL, " ");
	I = atoi(token);
	token = strtok(NULL, " ");
	L = atoi(token);
	token = strtok(NULL, " ");
	O = atoi(token);
	token = strtok(NULL, " ");
	A = atoi(token);

	// file format check
	if(strlen(buffer) > 2 && buffer[0] == 'a' && buffer[1] == 'a' && buffer[2] == 'g')
	{
		cout << endl <<  "Processing AIG in the ASCII format..." << endl;
		cout << "M I L O A = " << M << " " << I << " " << L
			 << " " << O << " " << A << endl << endl;
		process_ascii_format(input_file, M, I, L, O, A, vertices, outputs);
	}
	else if(strlen(buffer) > 2 && buffer[0] == 'a' && buffer[1] == 'i' && buffer[2] == 'g')
	{
		cout << endl << "Processing AIG in the binary format..." << endl;
		cout << "M I L O A = " << M << " " << I << " " << L
			 << " " << O << " " << A << endl << endl;
		process_binary_format(input_file, M, I, L, O, A, vertices, outputs);
	}
	else {
		cerr << "Failed to process the input file. Wrong, invalid or unknown format." << endl;
		exit(-1);
	}

}

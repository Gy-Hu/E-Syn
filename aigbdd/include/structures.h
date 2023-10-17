//////////////////////////////////////////////////////////////////////////////////
//
// Company: Universidade Federal do Rio Grande do Sul
// Engineer: Rafael de Oliveira Calçada
//
// Create Date: 01.12.2019 14:24:35
// Description: Data structures definitions
//
// Revision:
// Revision 0.01 - File created
//
//////////////////////////////////////////////////////////////////////////////////

#include <unordered_map>
#include <vector>
#include <stack>
using namespace std;

typedef struct v {
	int left;
	int right;
	int bdd;
} vertex;

typedef struct triple_struct {
	int var;
	int low;
	int high;
} triple;

typedef struct duo_struct {
	int u1;
	int u2;
} duo;

typedef int triple_key;
typedef int duo_key;

triple_key keygen_h(triple& t);
duo_key keygen_g(duo& d);
int add(vector<triple>& T, int i, int l, int h);
void init_t(vector<triple>& T, int max_index);
void init_h(unordered_map<triple_key,int>& H);
void init_g(unordered_map<duo_key,int>& G);
void insert_h(unordered_map<triple_key,int>& H, int i, int l, int h, int u);
void insert_g(unordered_map<duo_key,int>& G, int u1, int u2, int u);
void print_aig(string filename, vertex*& vertices, int& M, int& I, int& O);
void print_bdd(string filename, vector<triple>& T, int bdd_root, int output_index);
string bdd_signature(vector<triple>& T, int bdd_root);
#include <iostream>
#include <fstream>
#include <map>
#include <string>
#include <regex>
using namespace std;
int main(int argc, char* argv[]) {
    if(argc != 3){
        cerr << "Usage: " << argv[0] << " <input file> <output file>" << endl;
        return 1;
    }
    string inputFileName = argv[1];
    string outputFileName = argv[2];
    ifstream inputFile(inputFileName);
    if (!inputFile.is_open()) {
        cerr << "Unable to open input file" << endl;
        return 1;
    }
    map<string, string> variableExpressions;
    map<string, string> out_Expressions;
    string line;
    regex varFormat("^[a-zA-Z_]"); // Regex for the desired variable format
    while (getline(inputFile, line)) {
        
        size_t equalPos = line.find('=');
        if (equalPos != string::npos) {
            string variable = line.substr(0, equalPos);
            string expression = line.substr(equalPos + 1);
            expression.pop_back();
            if (variable.find("new_n") == 0) {
                variableExpressions[variable] = expression;
            }
            if (regex_match(variable, varFormat) && variable.find("new_n") == string::npos) { // Check for the desired format and exclude "new_n"
                out_Expressions[variable] = expression;
            }
        }
    }
    for (const auto& pair : variableExpressions) {
        cout << pair.first << "=" << pair.second << endl;
    }
    for (const auto& pair : out_Expressions) {
        cout <<pair.first << "=" << pair.second << endl;
    }
    map<string, string> newExpressions; // Map to hold modified output expressions
    for (const auto& pair : out_Expressions) {
        string newExpression = pair.second;
        bool replaced = true;
        while (replaced) {
            string oldExpression = newExpression;
            size_t pos = 0;
            while (pos < newExpression.length()) {
                size_t startPos = newExpression.find("new_n", pos);
                if (startPos == string::npos) {
                    break;  // No more "new_n" found, exit loop
                }
                size_t endPos = newExpression.find('_', startPos);
                if (endPos == string::npos) {
                    break;  // Invalid format, exit loop
                }
                size_t secondEndPos = newExpression.find('_', endPos + 1);
                if (secondEndPos == string::npos) {
                    break;  // Invalid format, exit loop
                }
                string variable = newExpression.substr(startPos, secondEndPos - startPos + 1) + (" ");
                if (variableExpressions.find(variable) != variableExpressions.end()) {
                    newExpression.replace(startPos, secondEndPos - startPos + 1, "(" + variableExpressions[variable] + ")");
                } else {
                    pos = secondEndPos + 1;  // Move to the next character
                }
            }
            if (newExpression == oldExpression) {
                replaced = false;
            }
        }
        newExpressions[pair.first] = newExpression; // Store the modified expression
    }
    ofstream outputFile(outputFileName);
    ifstream inputFile1(inputFileName);
    if (!inputFile1.is_open()) {
        cerr << "Unable to open input file" << endl;
        return 1;
    }
    
    for (int i = 0; i < 3; ++i) {
        string line;
        getline(inputFile1, line);
        outputFile << line << endl;
    }
   // Write output expressions for each po
    for (const auto& pair : newExpressions) {
        outputFile << pair.first << " = " << pair.second << ";" << endl;
    }
    outputFile.close();
    inputFile1.close();
    return 0;
}
#include <iostream>
#include <fstream>
#include <map>
#include <string>
#include <regex>
#include <set>


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
    set<string> outOrderVariables; // Set to hold variables from OUTORDER line

    // First, parse the OUTORDER line to get all the variables
    bool insideOutOrderBlock = false;
    int length_of_no_changed_line = 0;
    while (std::getline(inputFile, line)) {
        length_of_no_changed_line++;
        
        size_t outOrderPos = line.find("OUTORDER");
        if (outOrderPos != std::string::npos) {
            insideOutOrderBlock = true;
            line = line.substr(outOrderPos + 11); // Remove "OUTORDER" from the line
        }
        if (insideOutOrderBlock) {
            std::stringstream ss(line); // Create a stringstream from the line
            std::string variable;
            while (ss >> variable) { // Read each variable from the stringstream
                //remove `;` from the variable
                if (variable.back() == ';') {
                    variable.pop_back();
                }
                outOrderVariables.insert(variable); // Insert the variable into the set
            }
        }
        if (insideOutOrderBlock && line.back() == ';') { 
            // If we're inside the OUTORDER block and the line ends with a semicolon, 
            // we've reached the end of the OUTORDER block
            insideOutOrderBlock = false;
            break;
        }
    }

    // print all the Order variables
    // for (const auto& variable : outOrderVariables) {
    //     cout << variable << endl;
    // }
    //print how many variables are in the set
    //cout << "Number of variables in the set: " << outOrderVariables.size() << endl;

    // Then, use this set to parse the rest of the file
    while (getline(inputFile, line)) {
        size_t equalPos = line.find('=');
        if (equalPos != string::npos) {
            string variable = line.substr(0, equalPos);
            string expression = line.substr(equalPos + 1);
            expression.pop_back();
            
            // Trim trailing spaces from variable
            string tmp_var = variable;
            tmp_var.erase(variable.find_last_not_of(" \n\r\t")+1);
            //cout<< "variable: " << variable << endl;
            if (variable.find("new_n") == 0) {
                variableExpressions[variable] = expression;
            }
            if (outOrderVariables.find(tmp_var) != outOrderVariables.end()) {  // Check if variable is in the OUTORDER set
                out_Expressions[variable] = expression;
            }
        }
    }

    //print all in out_Expressions
    

    // for (const auto& pair : variableExpressions) {
    //     cout << pair.first << "=" << pair.second << endl;
    // }

    // for (const auto& pair : out_Expressions) {
    //     cout <<pair.first << "=" << pair.second << endl;
    // }

    // for (const auto& pair : out_Expressions) {
    //     cout <<pair.first << "=" << pair.second << endl;
    // }
    //map<string, string> newExpressions; // Map to hold modified output expressions
    vector<pair<string, string>> newExpressions;
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
                    string tmp_var = variableExpressions[variable];
                    //remove the first space in tmp_var
                    //tmp_var.erase(tmp_var.find_last_not_of(" \n\r\t")+1);
                    newExpression.replace(startPos, secondEndPos - startPos + 1, "(" + variableExpressions[variable] + ")");
                    //newExpression.replace(startPos, secondEndPos - startPos + 1, "(" + tmp_var + ")");
                } else {
                    pos = secondEndPos + 1;  // Move to the next character
                }
            }
            if (newExpression == oldExpression) {
                replaced = false;
            }
        }
        newExpressions.push_back(make_pair(pair.first, newExpression));
        //newExpressions[pair.first] = newExpression; // Store the modified expression
    }
    ofstream outputFile(outputFileName);
    ifstream inputFile1(inputFileName);
    if (!inputFile1.is_open()) {
        cerr << "Unable to open input file" << endl;
        return 1;
    }
    
    getline(inputFile1, line);
    outputFile << line << endl;

    for (int i = 1; i < length_of_no_changed_line; ++i) {
        string line;
        getline(inputFile1, line);
        //cout endl if meet `;`
        if (line.back() == ';') {
            outputFile << line << endl;
        } else {
            outputFile << line ;
        }
    }

    // for all the pairs in newExpressions, if pair.second contains `!`, replace it as `! `
    // for (auto& pair : newExpressions) {
    //     string tmp = pair.second;
    //     size_t pos = 0;
    //     while (pos < tmp.length()) {
    //         size_t startPos = tmp.find("!", pos);
    //         if (startPos == string::npos) {
    //             break;  // No more "!" found, exit loop
    //         }
    //         tmp.replace(startPos, 1, "! ");
    //         pos = startPos + 2;
    //     }
    //     pos = 0;
    //     while (pos < tmp.length()) {
    //         size_t startPos = tmp.find("( ", pos);
    //         if (startPos == string::npos) {
    //             break;  // No more "( " found, exit loop
    //         }
    //         tmp.replace(startPos, 2, "(");
    //         pos = startPos + 1;
    //     }
    //     newExpressions[pair.first] = tmp;
    // }

    for (auto& pair : newExpressions) {
        string tmp = pair.second;
        size_t pos = 0;
        while (pos < tmp.length()) {
            size_t startPos = tmp.find("!", pos);
            if (startPos == string::npos) {
                break;  // No more "!" found, exit loop
            }
            tmp.replace(startPos, 1, "! ");
            pos = startPos + 2;
        }
        pos = 0;
        while (pos < tmp.length()) {
            size_t startPos = tmp.find("( ", pos);
            if (startPos == string::npos) {
                break;  // No more "( " found, exit loop
            }
            tmp.replace(startPos, 2, "(");
            pos = startPos + 1;
        }
        pair.second = tmp;
    }

   // Write output expressions for each po
    for (const auto& pair : newExpressions) {
        outputFile << pair.first << " = (" << pair.second << ");" << endl;
    }

    // Write output expressions for each po
    // for (const auto& pair : newExpressions) {
    //     outputFile << pair.first << " = (" << pair.second << ");" << endl;
    // }

    outputFile.close();
    inputFile1.close();
    return 0;
}
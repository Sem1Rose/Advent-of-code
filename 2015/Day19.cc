#include <iostream>
#include <algorithm>
#include <map>
#include <unordered_set>
#include <vector>
#include <fstream>
#include <stdio.h>
#include <stdlib.h>
#include <bits/stdc++.h>

using namespace std;

int part_1(ifstream &input)
{
    map<string, vector<string>> replaces;
    string molecule;
    unordered_set<string> molecules;

    string line;
    while (getline(input, line))
    {
        if (line.empty())
        {
            getline(input, molecule);

            break;
        }

        char *key = (char *)malloc(sizeof(char) * 3), *replace = (char *)malloc(sizeof(char) * 12);

        sscanf(line.c_str(), "%s => %s", key, replace);

        if (replaces.find(key) == replaces.end())
            replaces.insert({key, {replace}});
        else
            replaces[key].push_back(replace);
    }

    // cout << molecule << endl;
    // for (auto it = replaces.cbegin(); it != replaces.cend(); ++it)
    // {
    //     cout << it->first << " = " << "[";
    //     for (auto &&i : it->second)
    //         cout << i << ", ";
    //     cout << "]" << endl;
    // }

    int i = 0;
    while (i < molecule.size())
    {
        string key{molecule[i]};

        if (replaces.find(key) != replaces.end())
        {
            for (auto &&replace : replaces[key])
            {
                string new_molecule = molecule;
                new_molecule.erase(new_molecule.begin() + i);
                int j = 0;
                for (auto &&c : replace)
                    new_molecule.insert(new_molecule.begin() + i + j++, c);

                molecules.insert(new_molecule);
            }
        }

        if (i < molecule.size() - 1)
        {
            key.push_back(molecule[i + 1]);
            if (replaces.find(key) != replaces.end())
            {
                for (auto &&replace : replaces[key])
                {
                    string new_molecule = molecule;
                    new_molecule.erase(new_molecule.begin() + i);
                    new_molecule.erase(new_molecule.begin() + i);
                    int j = 0;
                    for (auto &&c : replace)
                        new_molecule.insert(new_molecule.begin() + i + j++, c);

                    molecules.insert(new_molecule);
                }
            }
        }

        ++i;
    }

    cout << molecules.size() << endl;

    return 0;
}

// this is going to be a massacre
// rip my dear 12400 you will be missed
vector<int> iterate(int step, string mol, map<string, vector<string>> *replaces)
{
    if (mol == "e")
        return {step};

    vector<int> steps;

    for (auto &&k : *replaces)
    {
        int start = 0;
        // cout << k.first << endl;
        auto result = mol.find(k.first, start);
        while (result != string::npos)
        {
            string new_mol = mol;
            for (int i = 0; i < k.first.size(); i++)
                new_mol.erase(new_mol.begin() + result);

            // cout << "    " << new_mol << endl;

            for (auto &&v : k.second)
            {
                string newer_mol = new_mol;
                int j = 0;
                for (auto &&c : v)
                    newer_mol.insert(newer_mol.begin() + result + j++, c);

                // cout << "    " << v << "\t" << newer_mol << endl;
                for (auto &&r : iterate(step + 1, newer_mol, replaces))
                    steps.push_back(r);
            }
            start = result + 1;
            result = mol.find(k.first, start);
        }
    }

    return steps;
}

int part_2(ifstream &input)
{
    map<string, vector<string>> replaces;
    string molecule;
    unordered_set<string> molecules;

    string line;
    while (getline(input, line))
    {
        if (line.empty())
        {
            getline(input, molecule);

            break;
        }

        char *key = (char *)malloc(sizeof(char) * 3), *replace = (char *)malloc(sizeof(char) * 12);

        sscanf(line.c_str(), "%s => %s", key, replace);

        if (replaces.find(replace) == replaces.end())
            replaces.insert({replace, {key}});
        else
            replaces[replace].push_back(key);
    }

    // cout << molecule << endl;
    // for (auto it = replaces.cbegin(); it != replaces.cend(); ++it)
    // {
    //     cout << it->first << " = " << it->first.size() << " [";
    //     for (auto &&i : it->second)
    //         cout << i << ", ";
    //     cout << "]" << endl;
    // }

    int min = INT_MAX;
    for (auto &&i : iterate(0, molecule, &replaces))
        if (i < min)
            min = i;

    cout << min << endl;
    return 0;
}

int main()
{
    ifstream input("input.txt");
    if (!input.is_open())
    {
        cerr << "Error opening file" << endl;
        return 1;
    }

    return part_2(input);
}
#include <bits/stdc++.h>
#include <stdio.h>
#include <stdlib.h>

#include <algorithm>
#include <fstream>
#include <iostream>
#include <map>
#include <unordered_set>
#include <vector>

#include "./libs/ThreadPool.h"

using namespace std;

int part_1(ifstream &input)
{
    map<string, vector<string>> replaces;
    string molecule;
    unordered_set<string> molecules;

    string line;
    while (getline(input, line)) {
        if (line.empty()) {
            getline(input, molecule);

            break;
        }

        char *key     = (char *)malloc(sizeof(char) * 3),
             *replace = (char *)malloc(sizeof(char) * 12);

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
    while (i < molecule.size()) {
        string key{molecule[i]};

        if (replaces.find(key) != replaces.end()) {
            for (auto &&replace : replaces[key]) {
                string new_molecule = molecule;
                new_molecule.erase(new_molecule.begin() + i);
                int j = 0;
                for (auto &&c : replace)
                    new_molecule.insert(new_molecule.begin() + i + j++, c);

                molecules.insert(new_molecule);
            }
        }

        if (i < molecule.size() - 1) {
            key.push_back(molecule[i + 1]);
            if (replaces.find(key) != replaces.end()) {
                for (auto &&replace : replaces[key]) {
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
vector<int> iterate(ThreadPool *pool, int threading_level, int step, string mol,
                    map<string, vector<string>> *replaces)
{
    if (mol == "e")
        return {step};

    vector<int> steps;

    if (threading_level > 0) {
        vector<future<vector<int>>> results;
        for (auto &&k : *replaces) {
            results.emplace_back(pool->enqueue([&]() {
                vector<int> s;
                int start   = 0;
                auto result = mol.find(k.first, start);
                while (result != string::npos) {
                    string new_mol = mol;
                    for (int i = 0; i < k.first.size(); i++)
                        new_mol.erase(new_mol.begin() + result);

                    for (auto &&v : k.second) {
                        string newer_mol = new_mol;
                        int j            = 0;
                        for (auto &&c : v)
                            newer_mol.insert(newer_mol.begin() + result + j++,
                                             c);

                        for (auto &&r : iterate(pool, threading_level - 1,
                                                step + 1, newer_mol, replaces))
                            // steps.push_back(r);
                            s.push_back(r);
                    }

                    start  = result + 1;
                    result = mol.find(k.first, start);
                }

                return s;
            }));
        }

        for (auto &&result : results)
            for (auto &&i : result.get())
                steps.push_back(i);
    } else {
        for (auto &&k : *replaces) {
            int start   = 0;
            auto result = mol.find(k.first, start);
            while (result != string::npos) {
                string new_mol = mol;
                for (int i = 0; i < k.first.size(); i++)
                    new_mol.erase(new_mol.begin() + result);

                for (auto &&v : k.second) {
                    string newer_mol = new_mol;
                    int j            = 0;
                    for (auto &&c : v)
                        newer_mol.insert(newer_mol.begin() + result + j++, c);

                    for (auto &&r : iterate(pool, threading_level - 1, step + 1,
                                            newer_mol, replaces))
                        steps.push_back(r);
                }

                start  = result + 1;
                result = mol.find(k.first, start);
            }
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
    while (getline(input, line)) {
        if (line.empty()) {
            getline(input, molecule);

            break;
        }

        char *key     = (char *)malloc(sizeof(char) * 3),
             *replace = (char *)malloc(sizeof(char) * 12);

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

    ThreadPool pool(24);

    int min = INT_MAX;
    for (auto &&i : iterate(&pool, 1, 0, molecule, &replaces))
        if (i < min)
            min = i;

    cout << min << endl;

    // std::vector<std::future<int>> results;

    // for (int i = 0; i < 8; ++i) {
    //     results.emplace_back(pool.enqueue([i] {
    //         while (true)
    //             ;
    //         return0
    //     }));
    // }

    // for (auto &&result : results)
    //     std::cout << result.get() << ' ';
    // std::cout << std::endl;

    return 0;
}

int main()
{
    ifstream input("input.txt");
    if (!input.is_open()) {
        cerr << "Error opening file" << endl;
        return 1;
    }

    return part_2(input);
}
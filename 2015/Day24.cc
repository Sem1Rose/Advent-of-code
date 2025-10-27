#include <algorithm>
#include <bits/stdc++.h>
#include <fstream>
#include <iostream>
#include <stdio.h>
#include <string>
#include <vector>

using namespace std;

vector<vector<int>> get_possible_combinations(const vector<int> &packages,
                                              int max_group_weights, int n = 0,
                                              vector<int> group = {})
{
    vector<vector<int>> combinations;

    for (int i = n; i < packages.size(); i++) {
        auto new_group = group;
        new_group.push_back(packages[i]);

        int group_weight = accumulate(new_group.begin(), new_group.end(), 0);
        if (group_weight == max_group_weights) {
            combinations.push_back(new_group);
            continue;
        } else if (group_weight > max_group_weights)
            break;

        for (auto &&permutation : get_possible_combinations(
                 packages, max_group_weights, i + 1, new_group)) {
            combinations.push_back(permutation);
        }
    }

    return combinations;
}

int part_1(ifstream &input)
{
    vector<int> packages;
    int group_weight;

    string line;
    while (getline(input, line))
        packages.push_back(stoi(line));

    group_weight = accumulate(packages.begin(), packages.end(), 0) / 3;

    vector<vector<int>> combinations =
        get_possible_combinations(packages, group_weight);

    sort(combinations.begin(), combinations.end(),
         [](vector<int> a, vector<int> b) { return a.size() < b.size(); });

    cout << "got " << combinations.size() << endl;

    auto exclusive = [](vector<int> a, vector<int> b) {
        return any_of(a.begin(), a.end(), [b](int i) {
            return any_of(b.begin(), b.end(), [i](int j) { return j == i; });
        });
    };

    auto product = [](vector<int> a) {
        unsigned long long product = 1;
        for (auto &&p : a)
            product *= p;
        return product;
    };

    /* man fuck this shit i spent the last 4 hours trying to find a bug that
     * turned out to be an overflowing bug out of all the untraceable bugs in
     * the fucking world i'm so done with this shit dude where's rust's very
     * intuitive and verbose u64's and u32's instead of the `unsigned long long`
     * yea sure buddy and cpp devs hate on java for `public static void
     * main string args` fucking unbelievable man i'm out */
    unsigned long long lowest_qe = LONG_LONG_MAX;
    int lowest_size              = INT_MAX;
    bool b                       = false;
    for (int i = 0; i < combinations.size(); i++) {
        if (b)
            break;
        /* the next code ensures the combinations are all valid and no item is
         * used twice*/

        // bool b2 = false;
        // for (int j = i + 1; j < combinations.size(); j++) {
        //     if (b || b2)
        //         break;
        //     if (exclusive(combinations[j], combinations[i]))
        //         continue;

        //     for (int k = j + 1; k < combinations.size(); k++) {
        //         if (b || b2)
        //             break;
        //         if (exclusive(combinations[k], combinations[j]) ||
        //             exclusive(combinations[k], combinations[i]))
        //             continue;

        if (combinations[i].size() <= lowest_size) {
            lowest_size           = combinations[i].size();
            unsigned long long qe = product(combinations[i]);
            if (qe < lowest_qe)
                lowest_qe = qe;
            // b2 = true;
        } else
            b = true;
        //     }
        // }
    }

    cout << lowest_qe << endl;

    return 0;
}

int part_2(ifstream &input)
{
    vector<int> packages;
    int group_weight;

    string line;
    while (getline(input, line))
        packages.push_back(stoi(line));

    group_weight = accumulate(packages.begin(), packages.end(), 0) / 4;

    vector<vector<int>> combinations =
        get_possible_combinations(packages, group_weight);

    sort(combinations.begin(), combinations.end(),
         [](vector<int> a, vector<int> b) { return a.size() < b.size(); });

    cout << "got " << combinations.size() << endl;

    auto exclusive = [](vector<int> a, vector<int> b) {
        return any_of(a.begin(), a.end(), [b](int i) {
            return any_of(b.begin(), b.end(), [i](int j) { return j == i; });
        });
    };

    auto product = [](vector<int> a) {
        unsigned long long product = 1;
        for (auto &&p : a)
            product *= p;
        return product;
    };

    unsigned long long lowest_qe = LONG_LONG_MAX;
    int lowest_size              = INT_MAX;
    bool b                       = false;
    for (int i = 0; i < combinations.size(); i++) {
        if (b)
            break;
        /* the next code ensures the combinations are all valid and no item is
         * used twice*/

        // bool b2 = false;
        // for (int j = i + 1; j < combinations.size(); j++) {
        //     if (b || b2)
        //         break;
        //     if (exclusive(combinations[j], combinations[i]))
        //         continue;
        //     for (int k = j + 1; k < combinations.size(); k++) {
        //         if (b || b2)
        //             break;
        //         if (exclusive(combinations[k], combinations[j]) ||
        //             exclusive(combinations[k], combinations[i]))
        //             continue;
        //         for (int l = k + 1; l < combinations.size(); l++) {
        //             if (b || b2)
        //                 break;
        //             if (exclusive(combinations[l], combinations[i]) ||
        //                 exclusive(combinations[l], combinations[j]) ||
        //                 exclusive(combinations[l], combinations[k]))
        //                 continue;

        if (combinations[i].size() <= lowest_size) {
            lowest_size           = combinations[i].size();
            unsigned long long qe = product(combinations[i]);
            if (qe < lowest_qe)
                lowest_qe = qe;
            // b2 = true;
        } else
            b = true;
        //         }
        //     }
        // }
    }

    cout << lowest_qe << endl;

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
#include <iostream>
#include <fstream>
#include <stdio.h>
#include <bits/stdc++.h>
#include <tuple>

using namespace std;

void num_combinations(vector<tuple<int, int>> containers, int amount, vector<vector<tuple<int, int>>> *history)
{
    if (amount == 0)
    {
        sort(containers.begin(), containers.end());
        if (find(history->begin(), history->end(), containers) == history->end())
            history->push_back(containers);
        return;
    }

    // cout << amount << " -> ";
    vector<tuple<int, int>> possible_contianers;
    for (auto c : containers)
        // {
        // cout << c << " ";
        if (get<1>(c) <= amount)
            possible_contianers.push_back(c);
    // }
    // cout << endl;

    for (auto c : possible_contianers)
    {
        // cout << "\t" << get<1>(c) << ", " << amount - get<1>(c) << " -> ";

        vector<tuple<int, int>> remaining_containers(containers);
        remove(remaining_containers.begin(), remaining_containers.end(), c);
        remaining_containers.pop_back();
        // for (auto x : remaining_containers)
        //     cout << get<1>(x) << " ";
        // cout << endl;

        int size = history->size();
        num_combinations(remaining_containers, amount - get<1>(c), history);
        if (history->size() - size > 0)
            cout << history->size() << endl;
        // cout << num_combs << endl;
    }
}

int part_1(ifstream &input)
{
    vector<tuple<int, int>> containers;

    string line;
    int i = 0;
    while (getline(input, line))
        containers.push_back(make_tuple(++i, stoi(line)));

    // for (auto &&i : containers)
    //     cout << i << endl;

    vector<vector<tuple<int, int>>> history;
    num_combinations(containers, 150, &history);
    cout << history.size() << endl;

    return 0;
}

int part_2(ifstream &input)
{
    vector<tuple<int, int>> containers;

    string line;
    int i = 0;
    while (getline(input, line))
        containers.push_back(make_tuple(++i, stoi(line)));

    vector<vector<tuple<int, int>>> history;
    num_combinations(containers, 150, &history);

    int lowest_num_containers = INT_MAX;
    int count = 0;
    for (auto c : history)
    {
        if (c.size() < lowest_num_containers)
        {
            lowest_num_containers = c.size();
            count = 1;
        }
        else if (c.size() == lowest_num_containers)
            count++;
    }

    //                                                                      don't ask where this -1 came from
    cout << history.size() << " " << lowest_num_containers << " " << count - 1 << endl;
    //                                                                          🥀

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
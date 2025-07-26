#include <iostream>
#include <fstream>
#include <stdio.h>
#include <vector>
#include <algorithm>
#include <bits/stdc++.h>
#include <tuple>

using namespace std;

int get_weight(int id1, int id2, map<int, map<int, int>> &weights)
{
    if (id1 == id2)
        return INT_MAX;
    if (weights[id1].find(id2) == end(weights[id1]))
        return INT_MAX;

    return weights[id1][id2];
}

int dijkstra_get_highest_cost(vector<string> &nodes, map<int, map<int, int>> &weights)
{
    auto customComp = [](tuple<int, tuple<int, vector<int>>> l, tuple<int, tuple<int, vector<int>>> r)
    { return get<0>(l) < get<0>(r); };

    //  node        history    cost
    map<int, map<vector<int>, int>> costs;
    //                    order       node     history
    priority_queue<tuple<int, tuple<int, vector<int>>>, vector<tuple<int, tuple<int, vector<int>>>>, decltype(customComp)> priority_queue(customComp);

    for (int i = 0; i < nodes.size(); i++)
    {
        vector<int> x = {i};
        auto key = make_tuple(i, x);

        costs[i][x] = 0;
        priority_queue.push(make_tuple(0, key));
    }

    for (; !priority_queue.empty();)
    {
        auto cost = get<0>(priority_queue.top());

        auto item = get<1>(priority_queue.top());
        priority_queue.pop();

        auto history = get<1>(item);

        // cout << get<0>(item) << " " << nodes[get<0>(item)] << " " << cost << endl;

        for (int j = 0; j < nodes.size(); j++)
        {
            if (find(begin(history), end(history), j) != end(history))
                continue;

            int weight = get_weight(get<0>(item), j, weights);
            if (weight >= INT_MAX)
                continue;

            auto new_history = history;
            new_history.push_back(j);
            auto child = make_tuple(j, new_history);

            int child_cost = -1;
            if (costs.find(j) != end(costs) && costs[j].find(new_history) != end(costs[j]))
                child_cost = costs[j][new_history];

            // cout << "\t" << j << " " << nodes[j] << " " << weight << " " << child_cost << " " << costs[get<0>(item)][get<1>(item)] + weight << endl;

            if (child_cost < costs[get<0>(item)][get<1>(item)] + weight)
            {
                costs[j][new_history] = costs[get<0>(item)][get<1>(item)] + weight;
                priority_queue.push(make_tuple(costs[get<0>(item)][get<1>(item)] + weight, child));

                // if (new_history.size() == nodes.size())
                //     cout << "\t" << "found path: " << costs[get<0>(item)][get<1>(item)] + weight << endl;
            }
        }
    }

    // for (auto it = costs.cbegin(); it != costs.cend(); ++it)
    // {
    //     cout << it->first << " " << nodes[it->first] << endl;
    //     for (auto it2 = it->second.cbegin(); it2 != it->second.cend(); ++it2)
    //     {
    //         printf("\t%p\t%u\n", it2->first, it2->second);
    //     }
    // }

    int answer = -1;
    map<vector<int>, int> full_paths;
    for (int i = 0; i < nodes.size(); i++)
    {
        if (costs.find(i) != end(costs))
            for (auto it = costs[i].cbegin(); it != costs[i].cend(); ++it)
                if (it->first.size() == nodes.size())
                {
                    full_paths[it->first] = it->second + weights[it->first.at(it->first.size() - 1)][it->first.at(0)];

                    auto reversed = it->first;
                    reverse(reversed.begin(), reversed.end());

                    if (full_paths.find(reversed) != full_paths.end())
                        answer = max(answer, full_paths[it->first] + full_paths[reversed]);
                }
    }

    return answer;
}

int part_1(ifstream &input)
{
    vector<string> nodes;
    map<int, map<int, int>> weights;

    string line;
    while (getline(input, line))
    {
        stringstream ss(line);

        string w;
        getline(ss, w, ' ');

        auto index = find(begin(nodes), end(nodes), w);
        int node_start_index;
        if (index == end(nodes))
        {
            node_start_index = nodes.size();
            nodes.push_back(w);
        }
        else
            node_start_index = distance(begin(nodes), index);

        getline(ss, w, ' ');
        getline(ss, w, ' ');
        bool lose = w == "lose";

        getline(ss, w, ' ');
        int weight = stoi(w) * (lose ? -1 : 1);

        getline(ss, w, ' ');
        getline(ss, w, ' ');
        getline(ss, w, ' ');
        getline(ss, w, ' ');
        getline(ss, w, ' ');
        getline(ss, w, ' ');
        getline(ss, w, ' ');
        w = w.substr(0, w.size() - 1);

        index = find(begin(nodes), end(nodes), w);
        int node_destination_index;
        if (index == end(nodes))
        {
            node_destination_index = nodes.size();
            nodes.push_back(w);
        }
        else
            node_destination_index = distance(begin(nodes), index);

        weights[node_start_index][node_destination_index] = weight;
    }

    // for (int i = 0; i < nodes.size(); i++){
    //     cout << i << " " << nodes[i] << endl;
    //     for (int j = 0; j < nodes.size(); j++) {
    //         if (j == i) continue;
    //         cout << "\t" << j << " " << nodes[j] << ": " << get_weight(i, j, weights) << endl;
    //     }
    // }

    cout << dijkstra_get_highest_cost(nodes, weights) << endl;

    return 0;
}

int part_2(ifstream &input)
{
    vector<string> nodes;
    map<int, map<int, int>> weights;

    string line;
    while (getline(input, line))
    {
        stringstream ss(line);

        string w;
        getline(ss, w, ' ');

        auto index = find(begin(nodes), end(nodes), w);
        int node_start_index;
        if (index == end(nodes))
        {
            node_start_index = nodes.size();
            nodes.push_back(w);
        }
        else
            node_start_index = distance(begin(nodes), index);

        getline(ss, w, ' ');
        getline(ss, w, ' ');
        bool lose = w == "lose";

        getline(ss, w, ' ');
        int weight = stoi(w) * (lose ? -1 : 1);

        getline(ss, w, ' ');
        getline(ss, w, ' ');
        getline(ss, w, ' ');
        getline(ss, w, ' ');
        getline(ss, w, ' ');
        getline(ss, w, ' ');
        getline(ss, w, ' ');
        w = w.substr(0, w.size() - 1);

        index = find(begin(nodes), end(nodes), w);
        int node_destination_index;
        if (index == end(nodes))
        {
            node_destination_index = nodes.size();
            nodes.push_back(w);
        }
        else
            node_destination_index = distance(begin(nodes), index);

        weights[node_start_index][node_destination_index] = weight;
    }

    for (int i = 0; i < nodes.size(); i++){
        weights[i][nodes.size()] = 0;
        weights[nodes.size()][i] = 0;
    }
    nodes.push_back("damnitssofuckinghotinherefuckegypt");

    // for (int i = 0; i < nodes.size(); i++){
    //     cout << i << " " << nodes[i] << endl;
    //     for (int j = 0; j < nodes.size(); j++) {
    //         if (j == i) continue;
    //         cout << "\t" << j << " " << nodes[j] << ": " << get_weight(i, j, weights) << endl;
    //     }
    // }

    cout << dijkstra_get_highest_cost(nodes, weights) << endl;

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
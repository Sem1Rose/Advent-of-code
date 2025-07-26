#include <iostream>
#include <fstream>
#include <stdio.h>
#include <vector>
#include <algorithm>
#include <bits/stdc++.h>
#include <tuple>

using namespace std;

unsigned int get_weight(int id1, int id2, map<unsigned int, map<unsigned int, unsigned int>> &weights)
{
    if (id1 == id2)
        return INT_MAX;
    if (weights[min(id1, id2)].find(max(id1, id2)) == end(weights[min(id1, id2)]))
        return INT_MAX;

    return weights[min(id1, id2)][max(id1, id2)];
}

int dijkstra_get_lowest_cost(vector<string> &nodes, map<unsigned int, map<unsigned int, unsigned int>> &weights)
{
    auto customComp = [](tuple<int, tuple<int, unsigned int>> l, tuple<int, tuple<int, unsigned int>> r)
    { return get<0>(l) > get<0>(r); };

    //          node            mask            cost
    map<unsigned int, map<unsigned int, unsigned int>> costs;
    //                      order               node            mask
    priority_queue<tuple<unsigned int, tuple<unsigned int, unsigned int>>, vector<tuple<unsigned int, tuple<unsigned int, unsigned int>>>, decltype(customComp)> priority_queue(customComp);

    for (int i = 0; i < nodes.size(); i++)
    {
        auto key = make_tuple(i, pow(2, i));

        costs[i][pow(2, i)] = 0;
        priority_queue.push(make_tuple(0, key));
    }

    for (; !priority_queue.empty();)
    {
        auto cost = get<0>(priority_queue.top());

        auto item = get<1>(priority_queue.top());
        priority_queue.pop();

        for (int j = 0; j < nodes.size(); j++)
        {
            if ((unsigned int)pow(2, j) & get<1>(item)) continue;

            int weight = get_weight(get<0>(item), j, weights);
            if (weight >= INT_MAX)
                continue;

            unsigned int mask = get<1>(item) | (unsigned int)pow(2, j);
            auto child = make_tuple(j, mask);

            int current_cost = INT_MAX;
            if (costs.find(j) != end(costs) && costs[j].find(mask) != end(costs[j]))
                    current_cost = costs[j][mask];

            if (current_cost > costs[get<0>(item)][get<1>(item)] + weight)
            {
                costs[j][mask] = costs[get<0>(item)][get<1>(item)] + weight;
                priority_queue.push(make_tuple(costs[get<0>(item)][get<1>(item)] + weight, child));
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

    unsigned int mask = (unsigned int)pow(2, nodes.size()) - 1;
    unsigned int answer = INT_MAX;
    for (int i = 0; i < nodes.size(); i++)
    {
        unsigned int cost = INT_MAX;
        if (costs.find(i) != end(costs))
            if (costs[i].find(mask) != end(costs[i]))
                cost = costs[i][mask];

        answer = min(answer, cost);
    }

    return answer;
}

int part_1(ifstream &input)
{
    vector<string> nodes;
    map<unsigned int, map<unsigned int, unsigned int>> weights;

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

        index = find(begin(nodes), end(nodes), w);
        int node_destination_index;
        if (index == end(nodes))
        {
            node_destination_index = nodes.size();
            nodes.push_back(w);
        }
        else
            node_destination_index = distance(begin(nodes), index);

        getline(ss, w, ' ');
        getline(ss, w, ' ');

        unsigned int weight = stoi(w);

        int i = min(node_start_index, node_destination_index), j = max(node_start_index, node_destination_index);
        weights[i][j] = weight;
    }

    // for (auto it = weights.cbegin(); it != weights.cend(); ++it)
    // {
    //     cout << it->first << " " << nodes[it->first] << endl;

    //     for (auto it2 = it->second.cbegin(); it2 != it->second.cend(); ++it2)
    //     {
    //         cout << "\t" << it2->first << " " << nodes[it2->first] << ": " << it2->second << endl;
    //     }
    // }

    // for (int i = 0; i < nodes.size(); i++){
    //     cout << i << " " << nodes[i] << endl;
    //     for (int j = 0; j < nodes.size(); j++) {
    //         if (j == i) continue;
    //         cout << "\t" << j << " " << nodes[j] << ": " << get_weight(i, j, weights) << endl;
    //     }
    // }

    for (int i = 0; i < nodes.size(); i++)
    {
        cout << i << " " << nodes[i] << endl;
    }

    cout << dijkstra_get_lowest_cost(nodes, weights) << endl;

    return 0;
}

int dijkstra_get_highest_cost(vector<string> &nodes, map<unsigned int, map<unsigned int, unsigned int>> &weights)
{
    auto customComp = [](tuple<int, tuple<int, unsigned int>> l, tuple<int, tuple<int, unsigned int>> r)
    { return get<0>(l) > get<0>(r); };

    //  node           mask     cost
    map<int, map<unsigned int, int>> costs;
    //                    order       node     mask
    priority_queue<tuple<int, tuple<int, unsigned int>>, vector<tuple<int, tuple<int, unsigned int>>>, decltype(customComp)> priority_queue(customComp);

    for (int i = 0; i < nodes.size(); i++)
    {
        auto key = make_tuple(i, pow(2, i));

        costs[i][pow(2, i)] = 0;
        priority_queue.push(make_tuple(0, key));
    }

    for (; !priority_queue.empty();)
    {
        auto cost = get<0>(priority_queue.top());

        auto item = get<1>(priority_queue.top());
        priority_queue.pop();

        cout << get<0>(item) << " " << nodes[get<0>(item)] << " " << cost << endl;

        for (int j = 0; j < nodes.size(); j++)
        {
            if ((unsigned int)pow(2, j) & get<1>(item)) continue;

            int weight = get_weight(get<0>(item), j, weights);
            if (weight >= INT_MAX)
                continue;

            unsigned int mask = get<1>(item) | (unsigned int)pow(2, j);
            auto child = make_tuple(j, mask);

            int child_cost = -1;
            if (costs.find(j) != end(costs) && costs[j].find(mask) != end(costs[j]))
                    child_cost = costs[j][mask];

            cout << "\t" << j << " " << nodes[j] << " " << weight << " " << child_cost << " " << costs[get<0>(item)][get<1>(item)] + weight << endl;

            if (child_cost < costs[get<0>(item)][get<1>(item)] + weight)
            {
                costs[j][mask] = costs[get<0>(item)][get<1>(item)] + weight;
                priority_queue.push(make_tuple(costs[get<0>(item)][get<1>(item)] + weight, child));
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

    unsigned int mask = (unsigned int)pow(2, nodes.size()) - 1;
    int answer = -1;
    for (int i = 0; i < nodes.size(); i++)
    {
        int cost = -1;
        if (costs.find(i) != end(costs))
            if (costs[i].find(mask) != end(costs[i]))
                cost = costs[i][mask];

        answer = max(answer, cost);
    }

    return answer;
}

int part_2(ifstream &input)
{
    vector<string> nodes;
    map<unsigned int, map<unsigned int, unsigned int>> weights;

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

        index = find(begin(nodes), end(nodes), w);
        int node_destination_index;
        if (index == end(nodes))
        {
            node_destination_index = nodes.size();
            nodes.push_back(w);
        }
        else
            node_destination_index = distance(begin(nodes), index);

        getline(ss, w, ' ');
        getline(ss, w, ' ');

        unsigned int weight = stoi(w);

        int i = min(node_start_index, node_destination_index), j = max(node_start_index, node_destination_index);
        weights[i][j] = weight;
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
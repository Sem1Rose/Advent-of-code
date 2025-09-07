#include <iostream>
#include <fstream>
#include <stdio.h>
#include <bits/stdc++.h>

using namespace std;

typedef array<int, 10> aunt;
/*
children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1
*/

int part_1(ifstream &input)
{
    // just search "children: 3" in the puzzle input lmao

    // anyhow, this is part two modified to work with part one
    aunt search_items{3, 7, 2, 3, 0, 0, 5, 3, 2, 1};
    map<string, int> item_indices{
        {"children", 0},
        {"cats", 1},
        {"samoyeds", 2},
        {"pomeranians", 3},
        {"akitas", 4},
        {"vizslas", 5},
        {"goldfish", 6},
        {"trees", 7},
        {"cars", 8},
        {"perfumes", 9},
    };
    vector<aunt> aunts;

    string line;
    while (getline(input, line))
    {
        aunt a{-1, -1, -1, -1, -1, -1, -1, -1, -1, -1};

        string key;
        string value;
        stringstream ss(line);
        getline(ss, key, ' ');
        getline(ss, key, ' ');

        while (getline(ss, key, ' '))
        {
            getline(ss, value, ' ');

            value.erase(std::remove(value.begin(), value.end(), ','), value.end());
            a[item_indices[key.substr(0, key.size() - 1)]] = stoi(value);
        }

        aunts.push_back(a);
    }

    auto filtered_aunts(aunts);
    for (int i = 0; i < 10; i++)
    {
        for (int j = 0; j < filtered_aunts.size(); j++)
        {
            auto a = filtered_aunts[j];
            if (a[i] == -1)
                continue;

            if (a[i] != search_items[i])
            {
                auto x = remove(filtered_aunts.begin(), filtered_aunts.end(), a);
                filtered_aunts.erase(x, filtered_aunts.end());
                j--;
            }
        }
    }

    for (auto &&a : filtered_aunts)
        cout << find(aunts.begin(), aunts.end(), a) - aunts.begin() + 1 << endl;

    return 0;
}

int part_2(ifstream &input)
{
    aunt search_items{3, 7, 2, 3, 0, 0, 5, 3, 2, 1};
    aunt search_comp{0, 1, 0, -1, 0, 0, -1, 1, 0, 0};
    map<string, int> item_indices{
        {"children", 0},
        {"cats", 1},
        {"samoyeds", 2},
        {"pomeranians", 3},
        {"akitas", 4},
        {"vizslas", 5},
        {"goldfish", 6},
        {"trees", 7},
        {"cars", 8},
        {"perfumes", 9},
    };
    vector<aunt> aunts;

    string line;
    while (getline(input, line))
    {
        aunt a{-1, -1, -1, -1, -1, -1, -1, -1, -1, -1};

        string key;
        string value;
        stringstream ss(line);
        getline(ss, key, ' ');
        getline(ss, key, ' ');

        while (getline(ss, key, ' '))
        {
            getline(ss, value, ' ');

            value.erase(std::remove(value.begin(), value.end(), ','), value.end());
            a[item_indices[key.substr(0, key.size() - 1)]] = stoi(value);
        }

        aunts.push_back(a);
    }

    auto filtered_aunts(aunts);
    for (int i = 0; i < 10; i++)
    {
        for (int j = 0; j < filtered_aunts.size(); j++)
        {
            auto a = filtered_aunts[j];
            if (a[i] == -1)
                continue;

            switch (search_comp[i])
            {
            case 0:
                // printf("(%d == %d)?\t", a[i], search_items[i]);
                if (a[i] != search_items[i])
                {
                    auto x = remove(filtered_aunts.begin(), filtered_aunts.end(), a);
                    filtered_aunts.erase(x, filtered_aunts.end());
                    // printf("%d %d %d (%d != %d)\n", i, j, filtered_aunts.size(), a[i], search_items[i]);
                    j--;
                }
                // else
                //     cout << endl;
                break;
            case 1:
                // printf("(%d > %d)?\t", a[i], search_items[i]);
                if (a[i] <= search_items[i])
                {
                    auto x = remove(filtered_aunts.begin(), filtered_aunts.end(), a);
                    filtered_aunts.erase(x, filtered_aunts.end());
                    // printf("%d %d %d (%d <= %d)\n", i, j, filtered_aunts.size(), a[i], search_items[i]);
                    j--;
                }
                // else
                //     cout << endl;
                break;
            case -1:
                // printf("(%d < %d)?\t", a[i], search_items[i]);
                if (a[i] >= search_items[i])
                {
                    auto x = remove(filtered_aunts.begin(), filtered_aunts.end(), a);
                    filtered_aunts.erase(x, filtered_aunts.end());
                    // printf("%d %d %d (%d >= %d)\n", i, j, filtered_aunts.size(), a[i], search_items[i]);
                    j--;
                }
                // else
                //     cout << endl;
                break;
            default:
                continue;
            }
        }
    }

    for (auto &&a : filtered_aunts)
        cout << find(aunts.begin(), aunts.end(), a) - aunts.begin() + 1 << endl;

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
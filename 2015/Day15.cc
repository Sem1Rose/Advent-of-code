#include <iostream>
#include <fstream>
#include <stdio.h>
#include <vector>
#include <algorithm>
#include <numeric>
#include <bits/stdc++.h>

using namespace std;

typedef array<int, 5> cookie;
vector<int> get_prices(int index, int *previous, vector<cookie> cookies)
{
    // cout << string(index, '\t') << "index: " << index << endl;
    int new_prev[index + 1];
    int sum = 0;
    for (int i = 0; i < index; i++)
    {
        sum += previous[i];
        new_prev[i] = previous[i];
        // cout << string(index, '\t') << "\t" << previous->at(i);
    }

    // cout << "\t" << "sum : " << sum << endl;
    vector<int> prices;
    for (int i = 0; i + sum <= 100; i++)
    {
        new_prev[index] = i;
        if (index == cookies.size() - 1)
        {
            if (i + sum < 100)
                continue;

            // cout << string(index, '\t') << "\t" << i << " " << i + sum << endl;

            int price = 1;
            for (int j = 0; j < 4; j++)
            {
                int property = 0;
                for (int k = 0; k < cookies.size(); k++)
                {
                    // cout << string(index, '\t') << "\t\tcookie" << k << " " << new_prev[k] << " * " << cookies[k][j] << endl;
                    property += new_prev[k] * cookies[k][j];
                }
                // cout << string(index, '\t') << "\tproperty " << j << " " << property << endl;

                price *= max(property, 0);
            }
            // cout << string(index, '\t') << "price: " << price << endl;

            prices.push_back(price);
        }
        else
        {
            // cout << string(index, '\t') << i << endl;
            vector<int> new_prices = get_prices(index + 1, new_prev, cookies);

            prices.insert(prices.end(), new_prices.begin(), new_prices.end());
        }
    }

    return prices;
}

int part_1(ifstream &input)
{
    vector<cookie> cookies;

    string line;
    while (getline(input, line))
    {
        string name;
        int capacity = 0, durability = 0, flavor = 0, texture = 0, calories = 0;

        stringstream ss(line);
        getline(ss, name, ' ');

        string rest;
        getline(ss, rest);

        sscanf(rest.c_str(), "capacity %d, durability %d, flavor %d, texture %d, calories %d", &capacity, &durability, &flavor, &texture, &calories);

        cookie c{capacity, durability, flavor, texture, calories};
        cookies.push_back(c);
    }

    // for (auto &c : cookies)
    //     printf("capacity %d, durability %d, flavor %d, texture %d, calories %d\n", c[0], c[1], c[2], c[3], c[4]);

    vector<int> prices = get_prices(0, {}, cookies);
    // 103C3 -> (100 + 4 - 1) Choose (4-1)
    // for (int i = 0; i <= 100; i++)
    // {
    //     for (int j = 0; i + j <= 100; j++)
    //     {
    //         for (int k = 0; i + j + k <= 100; k++)
    //         {
    //             for (int l = 0; i + j + k + l <= 100; l++)
    //             {
    //                 if (i + j + k + l < 100)
    //                     // if (i + j < 100)
    //                     continue;

    //                 // int capacity = max(i * cookies[0][0] + j * cookies[1][0], 0),
    //                 int capacity = max(i * cookies[0][0] + j * cookies[1][0] + k * cookies[2][0] + l * cookies[3][0], 0),
    //                     // durability = max(i * cookies[0][1] + j * cookies[1][1], 0),
    //                     durability = max(i * cookies[0][1] + j * cookies[1][1] + k * cookies[2][1] + l * cookies[3][1], 0),
    //                     // flavor = max(i * cookies[0][2] + j * cookies[1][2], 0),
    //                     flavor = max(i * cookies[0][2] + j * cookies[1][2] + k * cookies[2][2] + l * cookies[3][2], 0),
    //                     // texture = max(i * cookies[0][3] + j * cookies[1][3], 0);
    //                     texture = max(i * cookies[0][3] + j * cookies[1][3] + k * cookies[2][3] + l * cookies[3][3], 0);

    //                 // printf("%d + %d = %d - %d - %d - %d =\t %d\n", i, j, capacity, durability, flavor, texture, capacity * durability * flavor * texture);

    //                 prices.push_back(capacity * durability * flavor * texture);
    //             }
    //         }
    //     }
    // }

    cout << *max_element(prices.begin(), prices.end()) << endl;

    return 0;
}

vector<int> get_prices2(int index, int *previous, vector<cookie> cookies)
{
    int new_prev[index + 1];
    int sum = 0;
    for (int i = 0; i < index; i++)
    {
        sum += previous[i];
        new_prev[i] = previous[i];
    }

    vector<int> prices;
    for (int i = 0; i + sum <= 100; i++)
    {
        new_prev[index] = i;
        if (index == cookies.size() - 1)
        {
            if (i + sum < 100)
                continue;

            int price = 1;
            for (int j = 0; j < 4; j++)
            {
                int property = 0;
                for (int k = 0; k < cookies.size(); k++)
                    property += new_prev[k] * cookies[k][j];
                price *= max(property, 0);
            }

            int calories = 0;
            for (int k = 0; k < cookies.size(); k++)
                calories += new_prev[k] * cookies[k][4];

            if (calories == 500)
                prices.push_back(price);
        }
        else
        {
            vector<int> new_prices = get_prices2(index + 1, new_prev, cookies);

            prices.insert(prices.end(), new_prices.begin(), new_prices.end());
        }
    }

    return prices;
}

int part_2(ifstream &input)
{
    vector<cookie> cookies;

    string line;
    while (getline(input, line))
    {
        string name;
        int capacity = 0, durability = 0, flavor = 0, texture = 0, calories = 0;

        stringstream ss(line);
        getline(ss, name, ' ');

        string rest;
        getline(ss, rest);

        sscanf(rest.c_str(), "capacity %d, durability %d, flavor %d, texture %d, calories %d", &capacity, &durability, &flavor, &texture, &calories);

        cookie c{capacity, durability, flavor, texture, calories};
        cookies.push_back(c);
    }

    vector<int> prices = get_prices2(0, {}, cookies);

    cout << *max_element(prices.begin(), prices.end()) << endl;

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

    return part_1(input);
}
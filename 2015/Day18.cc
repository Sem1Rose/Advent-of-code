#include <iostream>
#include <fstream>
#include <stdio.h>
#include <bits/stdc++.h>
#include <tuple>

using namespace std;

int part_1(ifstream &input)
{
    vector<vector<bool>> map;

    string line;
    int len;
    while (getline(input, line))
    {
        len = line.length();
        vector<bool> arr;
        for (auto i = line.begin(); i != line.end(); i++)
            arr.push_back(*i == '#');

        map.push_back(arr);
    }

    // for (int i = 0; i < len; i++)
    // {
    //     for (int j = 0; j < len; j++)
    //         cout << (map[i][j] ? '#' : '.');
    //     cout << endl;
    // }

    int dirs[4][2][2] = {
        {
            {0, 1},
            {1, 0},
        },
        {
            {1, 0},
            {0, -1},
        },
        {
            {0, -1},
            {-1, 0},
        },
        {
            {-1, 0},
            {0, 1},
        },
    };

    for (int cycle = 0; cycle < 100; cycle++)
    {
        vector<vector<bool>> new_map(map);
        for (int i = 0; i < len; i++)
        {
            for (int j = 0; j < len; j++)
            {
                bool status = map[i][j];

                int num_neighbors = 0;
                for (auto &dir : dirs)
                {
                    for (int repeat = 0; repeat < 2; repeat++)
                    {
                        int x = i + dir[0][0] + dir[1][0] * repeat, y = j + dir[0][1] + dir[1][1] * repeat;
                        if (x < 0 || x >= len || y < 0 || y >= len)
                            continue;

                        num_neighbors += map[x][y];
                    }
                }

                if (status)
                {
                    if (num_neighbors != 2 && num_neighbors != 3)
                        status = false;
                }
                else if (num_neighbors == 3)
                    status = true;

                new_map[i][j] = status;
            }
        }
        map = new_map;
    }

    int count = 0;
    for (int i = 0; i < len; i++)
        for (int j = 0; j < len; j++)
            count += map[i][j];

    cout << count << endl;

    return 0;
}

int part_2(ifstream &input)
{
    vector<vector<bool>> map;

    string line;
    int len;
    while (getline(input, line))
    {
        len = line.length();
        vector<bool> arr;
        for (auto i = line.begin(); i != line.end(); i++)
            arr.push_back(*i == '#');

        map.push_back(arr);
    }

    // for (int i = 0; i < len; i++)
    // {
    //     for (int j = 0; j < len; j++)
    //         cout << (map[i][j] ? '#' : '.');
    //     cout << endl;
    // }

    int dirs[4][2][2] = {
        {
            {0, 1},
            {1, 0},
        },
        {
            {1, 0},
            {0, -1},
        },
        {
            {0, -1},
            {-1, 0},
        },
        {
            {-1, 0},
            {0, 1},
        },
    };

    for (int cycle = 0; cycle < 100; cycle++)
    {
        vector<vector<bool>> new_map(map);
        for (int i = 0; i < len; i++)
        {
            for (int j = 0; j < len; j++)
            {
                if ((i == 0 || i == len - 1) && (j == 0 || j == len - 1))
                {
                    new_map[i][j] = true;
                    continue;
                }
                bool status = map[i][j];

                int num_neighbors = 0;
                for (auto &dir : dirs)
                {
                    for (int repeat = 0; repeat < 2; repeat++)
                    {
                        int x = i + dir[0][0] + dir[1][0] * repeat, y = j + dir[0][1] + dir[1][1] * repeat;
                        if (x < 0 || x >= len || y < 0 || y >= len)
                            continue;

                        num_neighbors += ((x == 0 || x == len - 1) && (y == 0 || y == len - 1)) ? true : map[x][y];
                    }
                }

                if (status)
                {
                    if (num_neighbors != 2 && num_neighbors != 3)
                        status = false;
                }
                else if (num_neighbors == 3)
                    status = true;

                new_map[i][j] = status;
            }
        }
        map = new_map;
    }

    int count = 0;
    for (int i = 0; i < len; i++)
        for (int j = 0; j < len; j++)
            count += map[i][j];

    cout << count << endl;

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
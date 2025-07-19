#include <iostream>
#include <fstream>
#include <stdio.h>
#include <vector>
#include <algorithm>

using namespace std;

int part_1(ifstream &input)
{
    string directions;
    getline(input, directions);

    vector<int> hashed_positions = {0};
    int x = 0, y = 0;
    for (char c : directions)
    {
        if (c == '^')
        {
            y++;
        }
        else if (c == 'v')
        {
            y--;
        }
        else if (c == '>')
        {
            x++;
        }
        else if (c == '<')
        {
            x--;
        } else {
            continue;
        }

        int hashed_position = y * 1000 + x;
        if(find(hashed_positions.begin(), hashed_positions.end(), hashed_position) != hashed_positions.end()) {
            continue;
        }
        hashed_positions.push_back(hashed_position);

        // printf("%d, %d, %d\n", x, y, hashed_position);
    }

    cout << hashed_positions.size() << endl;

    return 0;
}

int part_2(ifstream &input)
{
    string directions;
    getline(input, directions);

    vector<int> hashed_positions = {0};
    int positions[2][2] = {{0, 0}, {0, 0}};
    bool turn = false;
    for (char c : directions)
    {
        int index = turn ? 1 : 0;
        turn = !turn;

        if (c == '^')
        {
            positions[index][1]++;
        }
        else if (c == 'v')
        {
            positions[index][1]--;
        }
        else if (c == '>')
        {
            positions[index][0]++;
        }
        else if (c == '<')
        {
            positions[index][0]--;
        }
        else
        {
            continue;
        }

        int hashed_position = positions[index][1] * 1000 + positions[index][0];
        if (find(hashed_positions.begin(), hashed_positions.end(), hashed_position) != hashed_positions.end())
        {
            continue;
        }
        hashed_positions.push_back(hashed_position);

        // printf("%d, %d, %d\n", x, y, hashed_position);
    }

    cout << hashed_positions.size() << endl;

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
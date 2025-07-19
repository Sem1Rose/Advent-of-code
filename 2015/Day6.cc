#include <iostream>
#include <fstream>
#include <stdio.h>
#include <bits/stdc++.h>

using namespace std;

int part_1(ifstream &input)
{
    bool lights[1000][1000] = {false};
    string line;
    while (getline(input, line))
    {
        string operation;
        int x0;
        int y0;
        int x1;
        int y1;

        stringstream s(line);

        getline(s, operation, ' ');
        if (operation == "turn")
        {
            string on_off;
            getline(s, on_off, ' ');

            operation.append(on_off);
        }

        string remaining;
        getline(s, remaining);
        sscanf(remaining.c_str(), "%d,%d through %d,%d", &x0, &y0, &x1, &y1);

        // printf("%s %d,%d %d,%d\n", operation.c_str(), x0, y0, x1, y1);

        for (int i = x0; i <= x1; i++)
        {
            for (int j = y0; j <= y1; j++)
            {
                lights[i][j] = operation == "turnon"    ? true
                               : operation == "turnoff" ? false
                                                        : !lights[i][j];
            }
        }
    }

    int num = 0;
    for (int i = 0; i < 1000; i++)
    {
        for (int j = 0; j < 1000; j++)
        {
            if (lights[i][j])
                num++;
        }
    }

    cout << num << endl;

    return 0;
}

int part_2(ifstream &input)
{
    int lights[1000][1000] = {0};
    string line;
    while (getline(input, line))
    {
        string operation;
        int x0;
        int y0;
        int x1;
        int y1;

        stringstream s(line);

        getline(s, operation, ' ');
        if (operation == "turn")
        {
            string on_off;
            getline(s, on_off, ' ');

            operation.append(on_off);
        }

        string remaining;
        getline(s, remaining);
        sscanf(remaining.c_str(), "%d,%d through %d,%d", &x0, &y0, &x1, &y1);

        // printf("%s %d,%d %d,%d\n", operation.c_str(), x0, y0, x1, y1);

        for (int i = x0; i <= x1; i++)
        {
            for (int j = y0; j <= y1; j++)
            {
                lights[i][j] += operation == "turnon"                        ? 1
                                : operation == "turnoff" && lights[i][j] > 0 ? -1
                                : operation == "toggle"                      ? 2
                                                                             : 0;
            }
        }
    }

    unsigned long long sum = 0;
    for (int i = 0; i < 1000; i++)
    {
        for (int j = 0; j < 1000; j++)
        {
            sum += lights[i][j];
        }
    }

    cout << sum << endl;

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
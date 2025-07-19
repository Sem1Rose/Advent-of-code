#include <iostream>
#include <fstream>
#include <stdio.h>

using namespace std;

int part_1(ifstream &input)
{
    int floor = 0;
    string instructions;
    getline(input, instructions);

    for (char c : instructions)
    {
        if (c == '(')
            floor++;
        else if (c == ')')
            floor--;
        else
        {
            cerr << "Invalid character in input: " << c << endl;
            return 1;
        }
    }

    cout << floor << endl;

    return 0;
}

int part_2(ifstream &input)
{
    int floor = 0;
    string instructions;
    getline(input, instructions);

    int index = 1;
    for (char c : instructions)
    {
        if (c == '(')
            floor++;
        else if (c == ')')
            floor--;
        else
        {
            cerr << "Invalid character in input: " << c << endl;
            return 1;
        }

        if (floor == -1)
        {
            cout << index << endl;
            return 0;
        }

        index++;
    }

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

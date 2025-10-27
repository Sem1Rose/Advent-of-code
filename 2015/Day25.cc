#include <fstream>
#include <iostream>
#include <stdio.h>

using namespace std;

int part_1(ifstream &input)
{
    int row = 2981, column = 3075;
    long long id = ((row - 1) * row) / 2; // hell yea the lucky function
    for (int i = 0; i < column - 1; i++)
        id += row + i;
    id += column;

    long long prev = 20151125;
    for (int i = 1; i < id; i++) {
        prev = (prev * 252533) % 33554393;
        // cout << i + 1 << " " << prev << endl;
    }

    cout << prev << endl;

    return 0;
}

int part_2(ifstream &input) { return 0; }

int main()
{
    ifstream input("input.txt");
    if (!input.is_open()) {
        cerr << "Error opening file" << endl;
        return 1;
    }

    return part_1(input);
}
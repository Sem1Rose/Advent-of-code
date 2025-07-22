#include <iostream>
#include <fstream>
#include <stdio.h>

using namespace std;

int part_1(ifstream &input)
{
    unsigned int total_size = 0;
    unsigned int total_memory_size = 0;

    string line;
    while (getline(input, line))
    {
        total_size += line.size();

        for (int i = 1; i < line.size() - 1; i++, total_memory_size++)
        {
            char c = line.at(i);
            if (c == '\\')
            {
                c = line.at(++i);

                if (c == 'x')
                    i += 2;
            }
        }
    }

    cout << total_size - total_memory_size << endl;

    return 0;
}

int part_2(ifstream &input)
{
    unsigned int total_encoded_size = 0;
    unsigned int total_size = 0;

    string line;
    while (getline(input, line)) {
        total_size += line.size();
        total_encoded_size += 2;

        for (int i = 0; i < line.size(); i++, total_encoded_size++) {
            char c = line.at(i);

            if (c == '\\' || c == '"')
                total_encoded_size ++;
        }
    }

    cout << total_encoded_size - total_size << endl;

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
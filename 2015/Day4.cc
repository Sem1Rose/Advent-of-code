#include "md5.h"
#include <iostream>
#include <fstream>
#include <stdio.h>

using namespace std;
using md5::MD5;

int part_1(ifstream &input)
{
    string secret;
    getline(input, secret);

    for (int i = 1;; i++)
    {
        char key[100];
        sprintf(key, "%s%d", secret.c_str(), i);

        string hash = MD5::Hash(key);
        if (hash.rfind("00000", 0) == 0) {
            cout << hash << endl << i << endl;
            return 0;
        }
    }

    return 1;
}

int part_2(ifstream &input)
{
    string secret;
    getline(input, secret);

    for (int i = 1;; i++)
    {
        char key[100];
        sprintf(key, "%s%d", secret.c_str(), i);

        string hash = MD5::Hash(key);
        if (hash.rfind("000000", 0) == 0)
        {
            cout << hash << endl
                 << i << endl;
            return 0;
        }
    }

    return 1;
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
#include <iostream>
#include <fstream>
#include <stdio.h>

using namespace std;

string look_and_say(string input) {
    string str;
    for (int i = 0; i < input.length(); i++)
    {
        char c = input.at(i);
        int count = 1;
        // cout << i << " " << c << " " << "1" << endl;
        for (int j = i + 1; j < input.length(); j++) {
            char new_c = input.at(j);
            if (new_c == c) {
                i++;
                count++;
            } else
                break;
            // cout << j << " " << new_c << " " << count << endl;
        }

        str += to_string(count);
        str += c;

        // cout << str << endl;
    }

    return str;
}

int part_1(ifstream &input)
{
    string data = "3113322113";
    for (int i = 0; i < 40; i++)
        data = look_and_say(data);

    cout << data.length() << endl;

    return 0;
}

int part_2(ifstream &input)
{
    string data = "3113322113";
    for (int i = 0; i < 50; i++)
        data = look_and_say(data);

    cout << data.length() << endl;

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
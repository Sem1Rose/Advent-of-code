#include <iostream>
#include <fstream>
#include <stdio.h>
#include <bits/stdc++.h>

using namespace std;

string inc_string(string input)
{
    reverse(input.begin(), input.end());

    string output;
    bool inc = true;
    for (char c : input)
    {
        if (inc && c == 'z')
        {
            output += "a";
            continue;
        }

        output += char(c + (inc ? 1 : 0));
        inc = false;
    }

    reverse(output.begin(), output.end());
    return output;
}

string inc_string2(string input)
{
    string bad_chars = "iol";

    string preproccessed;
    bool reset = false;
    for (char c : input)
    {
        if (!reset && bad_chars.find(c) != string::npos)
        {
            preproccessed += char(c + 1);
            reset = true;
        } else if (reset) {
            preproccessed += 'a';
        } else {
            preproccessed += c;
        }
    }

    reverse(preproccessed.begin(), preproccessed.end());

    string output;
    bool inc = true;
    for (char c : preproccessed)
    {
        if (inc && c == 'z')
        {
            output += "a";
            continue;
        }

        output += char(c + (inc ? 1 : 0));
        inc = false;
    }

    reverse(output.begin(), output.end());
    return output;
}

bool check_password(string password) {
    string bad_chars = "iol";

    char prev_dublicates_char = 0;
    int num_inc_straights = 0, num_dublicates = 0;
    bool three_in_a_row = false;

    char prev_char = 0;
    for (auto &&c : password)
    {
        if(bad_chars.find(c) != string::npos)
            return false;

        if (prev_char == 0) {
            prev_char = c;
            continue;
        }

        if (c == prev_char && c != prev_dublicates_char){
            num_dublicates++;
            prev_dublicates_char = c;
        }

        if (c - prev_char == 1){
            num_inc_straights++;
            if (num_inc_straights >= 2)
                three_in_a_row = true;
        }
        else
            num_inc_straights = 0;

        prev_char = c;
    }

    if (num_dublicates >= 2 && three_in_a_row)
        return true;

    return false;
}

int part_1(ifstream &input)
{
    string password = "cqjxjnds";
    password = inc_string2(password);
    while (!check_password(password))
        password = inc_string2(password);

    cout << password << endl;

    return 0;
}

int part_2(ifstream &input)
{
    string password = "ghijklmn";
    password = inc_string2(password);
    while (!check_password(password))
        password = inc_string2(password);

    password = inc_string2(password);
    while (!check_password(password))
        password = inc_string2(password);

    cout << password << endl;

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
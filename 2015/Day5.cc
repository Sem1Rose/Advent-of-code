#include <iostream>
#include <fstream>
#include <stdio.h>
#include <algorithm>

using namespace std;

int part_1(ifstream &input)
{
    char vowels[6] = "aeoiu";
    char bad_strings[4][3] = {"ab", "cd", "pq", "xy"};

    int num_nice_strings = 0;
    string str;
    while (getline(input, str))
    {
        bool bad_string = false;
        bool twice_in_a_row = false;
        int num_vowls = 0;
        char previous_char = 0;
        for (char c : str)
        {
            if (previous_char == 0)
            {
                previous_char = c;
                continue;
            }

            char x[3] = {previous_char, c};
            if (find(begin(bad_strings), end(bad_strings), x) != end(bad_strings))
            {
                bad_string = true;
                break;
            }

            if (previous_char == c)
                twice_in_a_row = true;

            if (find(begin(vowels), end(vowels), c) != end(vowels))
                num_vowls++;

            previous_char = c;
        }

        if (!bad_string && num_vowls >= 3 && twice_in_a_row)
        {
            num_nice_strings++;
        }
    }

    cout << num_nice_strings << endl;

    return 0;
}

int part_2(ifstream &input)
{
    int num_nice_strings = 0;
    string str;
    while (getline(input, str))
    {
        int idx = 1;
        bool twice = false;
        bool char_repeat = false;
        char prev_prev_char = 0;
        char prev_char = 0;
        for (char c : str)
        {
            if (prev_char == 0)
            {
                prev_char = c;
                continue;
            }
            if (prev_prev_char == 0)
            {
                prev_prev_char = prev_char;
                prev_char = c;
                continue;
            }

            char x[3] = {prev_char, c};
            int result_index = str.find(x);
            // cout << x << " " << result_index << " " << idx << endl;
            if (result_index != idx && idx - result_index > 1)
                twice = true;

            if (c == prev_prev_char)
                char_repeat = true;

            prev_prev_char = prev_char;
            prev_char = c;

            idx++;
        }

        // cout << char_repeat << " " << twice << endl;
        if (char_repeat && twice)
        {
            num_nice_strings++;
        }
    }

    cout << num_nice_strings << endl;

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
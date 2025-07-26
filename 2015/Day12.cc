#include <iostream>
#include <fstream>
#include <stdio.h>
#include <bits/stdc++.h>

using namespace std;

int part_1(ifstream &input)
{
    string data;
    getline(input, data);

    string clean;

    auto iter = data.begin();
    while (iter != data.end())
    {
        if (!isdigit(*iter) && *iter != '-')
        {
            clean += ' ';
            while (++iter != data.end() && !isdigit(*iter) && *iter != '-');
            continue;
        }

        clean += *iter++;
    }

    int sum = 0;

    stringstream s(clean);
    string num;
    while (getline(s, num, ' ')) {
        if (num != "" && num != " "){
            sum += stoi(num);
        }
    }


    cout << sum << endl;

    return 0;
}

int part_2(ifstream &input)
{
    string data;
    getline(input, data);

    string preprocessed;
    reverse(data.begin(), data.end());
    auto iter = data.begin();
    vector<int> dists_to_braces;
    vector<int> real_dists_to_braces;
    while (iter != data.end())
    {
        preprocessed.push_back(*iter);
        if (*iter == '}'){
            real_dists_to_braces.push_back(0);
            dists_to_braces.push_back(0);
        }
        else if (*iter == '{'){
            real_dists_to_braces.pop_back();
            dists_to_braces.pop_back();
        }
        else if (*iter == '"' && distance(iter, data.end()) > 5)
        {
            if(*++++iter == 'e') {
                if (*++++++iter == ':') {
                    int dist = dists_to_braces[dists_to_braces.size() - 1];
                    dists_to_braces.pop_back();
                    for (int i = 0; i < dist; i++){
                        preprocessed.pop_back();

                        for (int i = 0; i < dists_to_braces.size(); i++)
                            dists_to_braces[i]--;
                    }

                    iter -= 5;
                    int real_dist = real_dists_to_braces[real_dists_to_braces.size() - 1];
                    real_dists_to_braces.pop_back();
                    for (int i = 0; i < real_dist; i++)
                    {
                        iter--;

                        for (int i = 0; i < real_dists_to_braces.size(); i++)
                            real_dists_to_braces[i]--;
                    }

                    int depth = 0;
                    while (true){
                        if (*iter == '}')
                            depth++;
                        else if (*iter == '{')
                            depth--;

                        if (depth == 0)
                            break;

                        iter++;
                        for (int i = 0; i < real_dists_to_braces.size(); i++)
                            real_dists_to_braces[i]++;
                    }
                }else
                    iter -= 5;
            }
            else
                iter-=2;
        }

        for (int i = 0; i < dists_to_braces.size(); i++){
            dists_to_braces[i]++;
            real_dists_to_braces[i]++;
        }
        iter++;
    }

    reverse(preprocessed.begin(), preprocessed.end());

    string clean;
    iter = preprocessed.begin();
    while (iter != preprocessed.end())
    {
        if (!isdigit(*iter) && *iter != '-')
        {
            clean += ' ';
            while (++iter != preprocessed.end() && !isdigit(*iter) && *iter != '-');
            continue;
        }

        clean += *iter++;
    }

    int sum = 0;

    stringstream s(clean);
    string num;
    while (getline(s, num, ' '))
    {
        if (num != "" && num != " ")
        {
            sum += stoi(num);
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

    return part_1(input);
}
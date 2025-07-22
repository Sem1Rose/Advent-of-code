#include <iostream>
#include <fstream>
#include <stdio.h>
#include <bits/stdc++.h>

using namespace std;

bool parse_op(map<string, unsigned short int> &wires, string operation, bool ass=false) {
    unsigned short int valuein1, valuein2;

    stringstream s(operation);

    string p;
    getline(s, p, ' ');
    if (p == "NOT")
    {
        getline(s, p, ' ');

        try
        {
            valuein1 = stoi(p);
        }
        catch (...)
        {
            if (wires.find(p) == end(wires))
            {
                cerr << "Damn " << operation << " " << p << endl;
                // exit(1);
                return false;
            }

            valuein1 = wires[p];
        }

        getline(s, p, ' ');
        getline(s, p, ' ');

        if (ass && p == "b")
            return false;

        unsigned short int value = ~valuein1;
        wires[p] = value;
    }
    else
    {
        try
        {
            valuein1 = stoi(p);
        }
        catch (...)
        {
            if (wires.find(p) == end(wires))
            {
                cerr << "Dang " << operation << " " << p << endl;
                // exit(1);
                return false;
            }

            valuein1 = wires[p];
        }

        getline(s, p, ' ');

        if (p == "->")
        {
            getline(s, p, ' ');

            if (ass && p == "b")
                return false;
            wires[p] = valuein1;
        }
        else
        {
            string operation = p;
            getline(s, p, ' ');

            try
            {
                valuein2 = stoi(p);
            }
            catch (...)
            {
                if (wires.find(p) == end(wires))
                {
                    cerr << "Jeez " << operation << " " << p << endl;
                    // exit(1);
                    return false;
                }

                valuein2 = wires[p];
            }

            getline(s, p, ' ');
            getline(s, p, ' ');

            unsigned short int value;
            if (operation == "OR")
            {
                value = valuein1 | valuein2;
            }
            else if (operation == "AND")
            {
                value = valuein1 & valuein2;
            }
            else if (operation == "LSHIFT")
            {
                value = valuein1 << valuein2;
            }
            else if (operation == "RSHIFT")
            {
                value = valuein1 >> valuein2;
            }
            else
            {
                cerr << "Sheet " << operation << " " << operation << endl;
                exit(1);
            }

            if (ass && p == "b")
                return false;
            wires[p] = value;
        }
    }

    return true;
}

int part_1(ifstream &input)
{
    map<string, unsigned short int> wires;
    vector<string> skipped_ops;

    string operation;
    while (getline(input, operation))
    {
        if (!parse_op(wires, operation)) {
            if (find(begin(skipped_ops), end(skipped_ops), operation) == end(skipped_ops)) {
                skipped_ops.push_back(operation);
            }
        }
        else
        {
            vector<string> new_skipped_ops;
            for (int i = 0; i < skipped_ops.size(); i++)
            {
                string op = skipped_ops[i];
                if (!parse_op(wires, op))
                {
                    new_skipped_ops.push_back(op);
                }
            }

            skipped_ops = new_skipped_ops;
        }
    }

    while (skipped_ops.size() > 0) {
        vector<string> new_skipped_ops;
        for (int i = 0; i < skipped_ops.size(); i++)
        {
            string op = skipped_ops[i];
            if (!parse_op(wires, op))
            {
                new_skipped_ops.push_back(op);
                cerr << "FUCK " << op << endl;
            }
        }

        skipped_ops = new_skipped_ops;
    }

    for (auto it = wires.cbegin(); it != wires.cend(); ++it) {
        cout << it->first << " = " << it->second << endl;
    }
    cout << wires["a"] << endl;

    return 0;
}

int part_2(ifstream &input)
{
    vector<string> operations;
    map<string, unsigned short int> wires;
    vector<string> skipped_ops;

    string operation;
    while (getline(input, operation))
    {
        operations.push_back(operation);
        if (!parse_op(wires, operation))
        {
            if (find(begin(skipped_ops), end(skipped_ops), operation) == end(skipped_ops))
            {
                skipped_ops.push_back(operation);
            }
        }
        else
        {
            vector<string> new_skipped_ops;
            for (int i = 0; i < skipped_ops.size(); i++)
            {
                string op = skipped_ops[i];
                if (!parse_op(wires, op))
                {
                    new_skipped_ops.push_back(op);
                }
            }

            skipped_ops = new_skipped_ops;
        }
    }

    while (skipped_ops.size() > 0)
    {
        vector<string> new_skipped_ops;
        for (int i = 0; i < skipped_ops.size(); i++)
        {
            string op = skipped_ops[i];
            if (!parse_op(wires, op))
            {
                new_skipped_ops.push_back(op);
                cerr << "FUCK " << op << endl;
            }
        }

        skipped_ops = new_skipped_ops;
    }

    unsigned short int a = wires["a"];
    wires.clear();
    skipped_ops.clear();

    wires["b"] = a;

    for (string operation: operations)
    {
        if (!parse_op(wires, operation, true))
        {
            if (find(begin(skipped_ops), end(skipped_ops), operation) == end(skipped_ops))
            {
                skipped_ops.push_back(operation);
            }
        }
        else
        {
            vector<string> new_skipped_ops;
            for (int i = 0; i < skipped_ops.size(); i++)
            {
                string op = skipped_ops[i];
                if (!parse_op(wires, op, true))
                {
                    new_skipped_ops.push_back(op);
                }
            }

            skipped_ops = new_skipped_ops;
        }
    }

    while (skipped_ops.size() > 0)
    {
        vector<string> new_skipped_ops;
        for (int i = 0; i < skipped_ops.size(); i++)
        {
            string op = skipped_ops[i];
            if (!parse_op(wires, op, true))
            {
                new_skipped_ops.push_back(op);
            }
        }

        skipped_ops = new_skipped_ops;

        // lmao goofy ass solution
        cout << wires["a"] << endl;
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
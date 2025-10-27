#include <bits/stdc++.h>
#include <fstream>
#include <iostream>
#include <stdio.h>
#include <string>

using namespace std;
class TuringMachine
{
  public:
    string *instructions;
    int instructions_length;
    int ip = -1;
    unsigned long long a, b = 0;

    TuringMachine(string *instructions, int length, int a)
        : instructions(instructions), instructions_length(length), a(a)
    {
    }

    void step()
    {
        if (++ip >= instructions_length) {
            cout << b << endl;
            exit(0);
        }

        string line = instructions[ip];

        stringstream stream(line);
        string instruction;
        getline(stream, instruction, ' ');

        string arguments;
        getline(stream, arguments);

        cout << ip << " " << instruction << " " << a << " " << b << endl;
        if (instruction == "hlf")
            hlf(arguments);
        else if (instruction == "tpl")
            tpl(arguments);
        else if (instruction == "inc")
            inc(arguments);
        else if (instruction == "jmp")
            jmp(arguments);
        else if (instruction == "jie")
            jie(arguments);
        else if (instruction == "jio")
            jio(arguments);
        else {
            cerr << "unknown instruction: " << instruction << endl;
            exit(1);
        }
    }

    void hlf(string arguments)
    {
        if (arguments[0] == 'a')
            a = a >> 1;
        else if (arguments[0] == 'b')
            b = b >> 1;
        else {
            cerr << "unknown register: " << arguments[0] << endl;
            exit(1);
        }
    }
    void tpl(string arguments)
    {
        if (arguments[0] == 'a')
            a *= 3;
        else if (arguments[0] == 'b')
            b *= 3;
        else {
            cerr << "unknown register: " << arguments[0] << endl;
            exit(1);
        }
    }
    void inc(string arguments)
    {
        if (arguments[0] == 'a')
            a++;
        else if (arguments[0] == 'b')
            b++;
        else {
            cerr << "unknown register: " << arguments[0] << endl;
            exit(1);
        }
    }
    void jmp(string arguments)
    {
        string offset_arg;
        if (arguments[0] == '+') {
            int i = 0;
            while (arguments[++i])
                offset_arg += arguments[i];

            int offset = stoi(offset_arg);
            ip += offset - 1;
        } else if (arguments[0] == '-') {
            int i = 0;
            while (arguments[++i])
                offset_arg += arguments[i];

            int offset = stoi(offset_arg);
            ip -= offset + 1;
        } else {
            cerr << "unknown offset: " << arguments << endl;
            exit(1);
        }
    }
    void jie(string arguments)
    {
        bool reg;
        if (arguments[0] == 'a')
            reg = 0;
        else if (arguments[0] == 'b')
            reg = 1;
        else {
            cerr << "unknown register: " << arguments[0] << endl;
            exit(1);
        }

        if (reg && (b & 1))
            return;
        else if (!reg && (a & 1))
            return;

        int i = 3;
        string offset_arg;
        if (arguments[i] == '+') {
            while (arguments[++i])
                offset_arg += arguments[i];

            int offset = stoi(offset_arg);
            ip += offset - 1;
        } else if (arguments[i] == '-') {
            while (arguments[++i])
                offset_arg += arguments[i];

            int offset = stoi(offset_arg);
            ip -= offset + 1;
        } else {
            cerr << "unknown offset: " << arguments << endl;
            exit(1);
        }
    }
    void jio(string arguments)
    {
        bool reg;
        if (arguments[0] == 'a')
            reg = 0;
        else if (arguments[0] == 'b')
            reg = 1;
        else {
            cerr << "unknown register: " << arguments[0] << endl;
            exit(1);
        }

        if (reg && b != 1)
            return;
        else if (!reg && a != 1)
            return;

        int i = 3;
        string offset_arg;
        if (arguments[i] == '+') {
            while (arguments[++i])
                offset_arg += arguments[i];

            int offset = stoi(offset_arg);
            ip += offset - 1;
        } else if (arguments[i] == '-') {
            while (arguments[++i])
                offset_arg += arguments[i];

            int offset = stoi(offset_arg);
            ip -= offset + 1;
        } else {
            cerr << "unknown offset: " << arguments << endl;
            exit(1);
        }
    }
};

int part_1(ifstream &input)
{
    string *instructions = new string[100];

    int count = 0;
    string line;
    while (getline(input, line))
        instructions[count++] = line;

    TuringMachine conpooter = TuringMachine(instructions, count, 0);
    while (true)
        conpooter.step();

    return 0;
}

int part_2(ifstream &input)
{
    string *instructions = new string[100];

    int count = 0;
    string line;
    while (getline(input, line))
        instructions[count++] = line;

    TuringMachine conpooter = TuringMachine(instructions, count, 1);
    while (true)
        conpooter.step();

    return 0;
}

int main()
{
    ifstream input("input.txt");
    if (!input.is_open()) {
        cerr << "Error opening file" << endl;
        return 1;
    }

    return part_2(input);
}
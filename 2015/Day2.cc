#include <iostream>
#include <fstream>
#include <stdio.h>

using namespace std;

int part_1(ifstream &input)
{
    int total_surface_area = 0;
    string dimension;
    while (getline(input, dimension)) {
        int l, w, h, surface_area, smallest_area = __INT_MAX__;
        sscanf(dimension.c_str(), "%dx%dx%d", &l, &w, &h);

        if (l * w < smallest_area)
            smallest_area = l * w;
        if (l * h < smallest_area)
            smallest_area = l * h;
        if (h * w < smallest_area)
            smallest_area = h * w;

        surface_area = 2 * l * w + 2 * (l + w) * h;

        total_surface_area += surface_area + smallest_area;
    }

    cout << total_surface_area << endl;

    return 0;
}

int part_2(ifstream &input)
{
    int total_ribbon_length = 0;
    string dimension;
    while (getline(input, dimension))
    {
        int l, w, h, ribbon_length, smallest_perimeter = __INT_MAX__;
        sscanf(dimension.c_str(), "%dx%dx%d", &l, &w, &h);

        if (l + w < smallest_perimeter)
            smallest_perimeter = l + w;
        if (l + h < smallest_perimeter)
            smallest_perimeter = l + h;
        if (h + w < smallest_perimeter)
            smallest_perimeter = h + w;

        total_ribbon_length += smallest_perimeter * 2 + l * w * h;
    }

    cout << total_ribbon_length << endl;

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

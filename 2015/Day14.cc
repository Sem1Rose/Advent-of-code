#include <iostream>
#include <fstream>
#include <stdio.h>
#include <vector>
#include <algorithm>
#include <bits/stdc++.h>

using namespace std;

struct Reindeer
{
    string name;
    int speed;
    int fly_duration;
    int rest_duration;
};

int part_1(ifstream &input)
{
    vector<Reindeer> reindeers;

    string line;
    while (getline(input, line))
    {
        string name;
        int speed = 0, fly_duration = 0, rest_duration = 0;

        stringstream ss(line);
        getline(ss, name, ' ');

        string rest;
        getline(ss, rest);

        sscanf(rest.c_str(), "can fly %d km/s for %d seconds, but then must rest for %d seconds.", &speed, &fly_duration, &rest_duration);

        Reindeer r = {name, speed, fly_duration, rest_duration};
        reindeers.push_back(r);
    }

    // for (auto &&reindeer : reindeers)
    //     printf("%s, %dkm/s, %d, %d\n", reindeer.name.c_str(), reindeer.speed, reindeer.fly_duration, reindeer.rest_duration);

    const int race_seconds = 2503;

    int dists_travelled[reindeers.size()];
    for (int i = 0; i < reindeers.size(); i++)
    {
        Reindeer reindeer = reindeers[i];

        int cycle = reindeer.fly_duration + reindeer.rest_duration;
        int kmpcycle = reindeer.speed * reindeer.fly_duration;

        int num_cycles = race_seconds / cycle;
        int remaining = race_seconds % cycle;

        int dist_travelled = 0;
        if (remaining >= reindeer.fly_duration)
        {
            num_cycles++;
            dist_travelled = num_cycles * kmpcycle;
        }
        else
        {
            dist_travelled = num_cycles * kmpcycle + remaining * reindeer.speed;
        }

        dists_travelled[i] = dist_travelled;
    }

    cout << *max_element(dists_travelled, dists_travelled + reindeers.size()) << endl;

    return 0;
}

int part_2(ifstream &input)
{
    vector<Reindeer> reindeers;

    string line;
    while (getline(input, line))
    {
        string name;
        int speed = 0, fly_duration = 0, rest_duration = 0;

        stringstream ss(line);
        getline(ss, name, ' ');

        string rest;
        getline(ss, rest);

        sscanf(rest.c_str(), "can fly %d km/s for %d seconds, but then must rest for %d seconds.", &speed, &fly_duration, &rest_duration);

        Reindeer r = {name, speed, fly_duration, rest_duration};
        reindeers.push_back(r);
    }
    const int race_seconds = 2503;

    vector<int> stars(reindeers.size(), 0);
    vector<int> dists_travelled(reindeers.size());
    for (int second = 1; second <= race_seconds; second++)
    {
        for (int i = 0; i < reindeers.size(); i++)
        {
            Reindeer reindeer = reindeers[i];

            int cycle = reindeer.fly_duration + reindeer.rest_duration;
            int kmpcycle = reindeer.speed * reindeer.fly_duration;

            int num_cycles = second / cycle;
            int remaining = second % cycle;

            int dist_travelled = 0;
            if (remaining >= reindeer.fly_duration)
            {
                num_cycles++;
                dist_travelled = num_cycles * kmpcycle;
            }
            else
            {
                dist_travelled = num_cycles * kmpcycle + remaining * reindeer.speed;
            }

            dists_travelled[i] = dist_travelled;
        }

        int max_dist = *max_element(dists_travelled.begin(), dists_travelled.end());
        for (int i = 0; i < reindeers.size(); i++)
            if (dists_travelled[i] == max_dist)
                stars[i]++;
    }
    cout << *max_element(stars.begin(), stars.end()) << endl;

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
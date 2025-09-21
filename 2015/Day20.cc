#include <fstream>
#include <iostream>
#include <map>
#include <math.h>
#include <stdio.h>
#include <vector>

using namespace std;

// hell yea boiiiiiiiiiiiiiiii
vector<int> get_factors(int n)
{
    vector<int> factors{1, n};
    for (int i = (n & 1 ? 3 : 2); i <= sqrt(n); i += (n & 1 ? 2 : 1))
        if (!(n % i)) {
            factors.push_back(i);
            factors.push_back(n / i);
        }

    return factors;
}

int part_1(ifstream &input)
{
    // for (int i = 2; i < 100; i++) {
    //     cout << i << ": ";
    //     for (auto &&j : get_factors(i))
    //         cout << j << " ";
    //     cout << endl;
    // }

    auto sum = [](const auto &container) {
        auto s = 0;
        for (const auto &i : container) {
            s += (int)i;
        }
        return s;
    };

    //           magic num 2.0 skipping a ton of pointless calculations
    for (int i = 500000;; i++) {
        // cout << i << ": " << sum(get_factors(i)) << endl;
        if (sum(get_factors(i)) >= 2900000) {
            cout << i << endl;
            break;
        }
    }
    return 0;
}

int part_2(ifstream &input)
{
    for (int i = 500000;; i++) {
        auto sum2 = [&i](const auto &container) {
            auto s = 0;
            for (const auto &x : container) {
                s += (i > (int)x * 50 ? 0 : (int)x);
            }
            return s;
        };

        vector<int> factors = get_factors(i);

        if (sum2(factors) >= 29000000 / 11) {
            cout << i << endl;
            break;
        }
    }

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
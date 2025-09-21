#include <bits/stdc++.h>
#include <fstream>
#include <iostream>
#include <stdio.h>
#include <vector>

using namespace std;

struct ShopItem {
    int cost, damage, armor;
};
struct Player {
    int hp, damage, armor;
};
struct Game {
    Player player, boss;
    int choices[4];
    int cost;
};

// bool play_game(Game *game)
// {
//     int player_damage = max(1, game->player.damage - game->boss.armor);
//     int boss_damage   = max(1, game->boss.damage - game->player.armor);
//     return (ceil(game->boss.hp / (float)player_damage) >=
//             ceil(game->player.hp / (float)boss_damage));
//     // cout << ceil(game->boss.hp / (float)player_damage) << " "
//     //      << ceil(game->player.hp / (float)boss_damage) << endl;

//     // cout << player_damage << " " << boss_damage << endl;
//     // bool turn = true;
//     // while (true) {
//     //     if (turn)
//     //         game->boss.hp -= player_damage;
//     //     else
//     //         game->player.hp -= boss_damage;

//     //     cout << game->boss.hp << " x " << game->player.hp << endl;

//     //     turn = !turn;

//     //     if (game->boss.hp <= 0 || game->player.hp <= 0)
//     //         break;
//     // }

//     // return game->player.hp > 0;
// }

int part_1(ifstream &input)
{
    vector<ShopItem> weapons;
    vector<ShopItem> armor;
    vector<ShopItem> rings;

    string line;
    getline(input, line);
    while (getline(input, line)) {
        if (line.empty())
            break;
        ShopItem weapon{0, 0, 0};

        stringstream stream(line);
        string ss;
        getline(stream, ss, ' ');

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        weapon.cost = stoi(ss);

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        weapon.damage = stoi(ss);

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        weapon.armor = stoi(ss);

        weapons.push_back(weapon);
    }

    getline(input, line);
    while (getline(input, line)) {
        if (line.empty())
            break;
        ShopItem armr{0, 0, 0};

        stringstream stream(line);
        string ss;
        getline(stream, ss, ' ');

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        armr.cost = stoi(ss);

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        armr.damage = stoi(ss);

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        armr.armor = stoi(ss);

        armor.push_back(armr);
    }

    getline(input, line);
    while (getline(input, line)) {
        if (line.empty())
            break;
        ShopItem ring{0, 0, 0};

        stringstream stream(line);
        string ss;
        getline(stream, ss, ' ');
        getline(stream, ss, ' ');

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        ring.cost = stoi(ss);

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        ring.damage = stoi(ss);

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        ring.armor = stoi(ss);

        rings.push_back(ring);
    }

    // for (auto &&i : weapons)
    //     printf("%d %d %d\n", i.cost, i.damage, i.armor);
    // cout << endl;
    // for (auto &&i : armor)
    //     printf("%d %d %d\n", i.cost, i.damage, i.armor);
    // cout << endl;
    // for (auto &&i : rings)
    //     printf("%d %d %d\n", i.cost, i.damage, i.armor);

    auto play_game = [](Game *game) {
        int player_damage = max(1, game->player.damage - game->boss.armor);
        int boss_damage   = max(1, game->boss.damage - game->player.armor);
        cout << player_damage << " x " << boss_damage << " | "
             << ceil(game->boss.hp / (float)player_damage) << " x "
             << ceil(game->player.hp / (float)boss_damage) << endl;
        return (ceil(game->boss.hp / (float)player_damage) <=
                ceil(game->player.hp / (float)boss_damage));
    };
    auto recalculate = [&weapons, &armor, &rings](Game *game) {
        game->cost          = 0;
        game->player.armor  = 0;
        game->player.damage = 0;
        if (game->choices[0] != -1) {
            game->cost += weapons[game->choices[0]].cost;
            game->player.armor += weapons[game->choices[0]].armor;
            game->player.damage += weapons[game->choices[0]].damage;
        }
        if (game->choices[1] != -1) {
            game->cost += armor[game->choices[1]].cost;
            game->player.armor += armor[game->choices[1]].armor;
            game->player.damage += armor[game->choices[1]].damage;
        }
        if (game->choices[2] != -1) {
            game->cost += rings[game->choices[2]].cost;
            game->player.armor += rings[game->choices[2]].armor;
            game->player.damage += rings[game->choices[2]].damage;
        }
        if (game->choices[3] != -1) {
            game->cost += rings[game->choices[3]].cost;
            game->player.armor += rings[game->choices[3]].armor;
            game->player.damage += rings[game->choices[3]].damage;
        }
    };

    Player boss{103, 9, 2};

    Game game{
        Player{
            100,
            weapons[0].damage,
            0,
        },
        boss,
        {0, -1, -1, -1},
        weapons[0].cost,
    };

    // while (!play_game(&game)) {
    //     int lowest_price = INT_MAX;
    //     int choice       = -1;
    //     int choice_id    = -1;
    //     if (game.choices[0] + 1 < weapons.size())
    //         if (weapons[game.choices[0] + 1].cost < lowest_price) {
    //             lowest_price = weapons[game.choices[0] + 1].cost;
    //             choice       = 0;
    //             choice_id    = game.choices[0] + 1;
    //         }

    //     if (game.choices[1] + 1 < armor.size())
    //         if (armor[game.choices[1] + 1].cost < lowest_price) {
    //             lowest_price = armor[game.choices[1] + 1].cost;
    //             choice       = 1;
    //             choice_id    = game.choices[1] + 1;
    //         }

    //     int rings_cost = 0;
    //     if (game.choices[2] != -1)
    //         rings_cost = rings[game.choices[2]].cost;
    //     for (int i = 0; i < rings.size(); i++) {
    //         if (i == game.choices[3])
    //             continue;

    //         if (rings[i].cost > rings_cost && rings[i].cost < lowest_price) {
    //             lowest_price = rings[i].cost;
    //             choice       = 2;
    //             choice_id    = i;
    //         }
    //     }

    //     if (choice != 2) {
    //         rings_cost = 0;
    //         if (game.choices[3] != -1)
    //             rings_cost = rings[game.choices[3]].cost;
    //         for (int i = 0; i < rings.size(); i++) {
    //             if (i == game.choices[2])
    //                 continue;

    //             if (rings[i].cost > rings_cost &&
    //                 rings[i].cost < lowest_price) {
    //                 lowest_price = rings[i].cost;
    //                 choice       = 3;
    //                 choice_id    = i;
    //             }
    //         }
    //     }

    //     if (choice == -1)
    //         break;

    //     game.choices[choice] = choice_id;
    //     recalculate(&game);

    //     printf("%d, %d, %d, %d\n", game.choices[0], game.choices[1],
    //            game.choices[2], game.choices[3]);
    // }

    // cout << game.cost << endl << endl;

    // bool locks[4]{false, false, false, false};

    // while (true) {
    //     int highest_price_cut = -1;
    //     int choice            = -1;
    //     int choice_id         = -1;
    //     if (!locks[0] && game.choices[0] > 0)
    //         if (weapons[game.choices[0]].cost -
    //                 weapons[game.choices[0] - 1].cost >
    //             highest_price_cut) {
    //             highest_price_cut = weapons[game.choices[0]].cost -
    //                                 weapons[game.choices[0] - 1].cost;
    //             choice    = 0;
    //             choice_id = game.choices[0] - 1;
    //         }

    //     if (!locks[1] && game.choices[1] > 0) {
    //         if (armor[game.choices[1]].cost - armor[game.choices[1] - 1].cost
    //         >
    //             highest_price_cut) {
    //             highest_price_cut = armor[game.choices[1]].cost -
    //                                 armor[game.choices[1] - 1].cost;
    //             choice    = 1;
    //             choice_id = game.choices[1] - 1;
    //         }
    //     } else if (!locks[1]) {
    //         if (armor[game.choices[1]].cost > highest_price_cut) {
    //             highest_price_cut = armor[game.choices[1]].cost;
    //             choice            = 1;
    //             choice_id         = -1;
    //         }
    //     }

    //     if (!locks[2]) {
    //         int rings_cost = 0;
    //         if (game.choices[2] != -1)
    //             rings_cost = rings[game.choices[2]].cost;
    //         for (int i = 0; i < rings.size(); i++) {
    //             if (i == game.choices[3])
    //                 continue;

    //             if (rings[i].cost < rings_cost &&
    //                 rings[game.choices[2]].cost - rings[i].cost >
    //                     highest_price_cut) {
    //                 highest_price_cut =
    //                     rings[game.choices[2]].cost - rings[i].cost;
    //                 choice    = 2;
    //                 choice_id = i;
    //             }
    //         }
    //     }

    //     if (!locks[3]) {
    //         int rings_cost = 0;
    //         if (game.choices[3] != -1)
    //             rings_cost = rings[game.choices[3]].cost;
    //         for (int i = 0; i < rings.size(); i++) {
    //             if (i == game.choices[2])
    //                 continue;

    //             if (rings[i].cost < rings_cost &&
    //                 rings[game.choices[3]].cost - rings[i].cost >
    //                     highest_price_cut) {
    //                 highest_price_cut =
    //                     rings[game.choices[3]].cost - rings[i].cost;
    //                 choice    = 3;
    //                 choice_id = i;
    //             }
    //         }
    //     }

    //     if (choice == -1)
    //         break;

    //     int prev_id          = game.choices[choice];
    //     game.choices[choice] = choice_id;

    //     printf("%d, %d, %d, %d\n", game.choices[0], game.choices[1],
    //            game.choices[2], game.choices[3]);

    //     recalculate(&game);

    //     if (!play_game(&game)) {
    //         cout << "rolling back" << endl;
    //         game.choices[choice] = prev_id;
    //         recalculate(&game);
    //         locks[choice] = true;
    //     } else {
    //         cout << game.cost << endl;
    //         for (int i = 0; i < 4; i++)
    //             locks[i] = false;
    //     }
    //     printf("%d, %d, %d, %d\n", locks[0], locks[1], locks[2], locks[3]);

    //     bool brk = true;
    //     for (int i = 0; i < 4; i++)
    //         if (!locks[i])
    //             brk = false;

    //     if (brk)
    //         break;
    // }

    /* WHEN NOHING WORKS
        JUST USE THE GOOD OL' BRUTE FORCE */

    // c++ is so ass for making this a thing!!!!
    int armor_size = armor.size(), weapons_size = weapons.size(),
        rings_size  = rings.size();
    int lowest_cost = INT_MAX;
    for (int wpn = 0; wpn < weapons_size; wpn++) {
        for (int armr = -1; armr < armor_size; armr++) {
            for (int rng1 = -1; rng1 < rings_size; rng1++) {
                for (int rng2 = -1; rng2 < rings_size; rng2++) {
                    if (rng1 == rng2 && rng1 != -1)
                        continue;

                    auto prev_choices = game.choices;

                    game.choices[0] = wpn;
                    game.choices[1] = armr;
                    game.choices[2] = rng1;
                    game.choices[3] = rng2;

                    printf("%d, %d, %d, %d\n", game.choices[0], game.choices[1],
                           game.choices[2], game.choices[3]);

                    recalculate(&game);
                    cout << lowest_cost << " " << game.cost << endl;
                    if (game.cost >= lowest_cost || !play_game(&game)) {
                        game.choices[0] = prev_choices[0];
                        game.choices[1] = prev_choices[1];
                        game.choices[2] = prev_choices[2];
                        game.choices[3] = prev_choices[3];
                    } else {
                        lowest_cost = game.cost;
                    }
                }
            }
        }
    }

    cout << lowest_cost << endl;

    return 0;
}

int part_2(ifstream &input)
{

    vector<ShopItem> weapons;
    vector<ShopItem> armor;
    vector<ShopItem> rings;

    string line;
    getline(input, line);
    while (getline(input, line)) {
        if (line.empty())
            break;
        ShopItem weapon{0, 0, 0};

        stringstream stream(line);
        string ss;
        getline(stream, ss, ' ');

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        weapon.cost = stoi(ss);

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        weapon.damage = stoi(ss);

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        weapon.armor = stoi(ss);

        weapons.push_back(weapon);
    }

    getline(input, line);
    while (getline(input, line)) {
        if (line.empty())
            break;
        ShopItem armr{0, 0, 0};

        stringstream stream(line);
        string ss;
        getline(stream, ss, ' ');

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        armr.cost = stoi(ss);

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        armr.damage = stoi(ss);

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        armr.armor = stoi(ss);

        armor.push_back(armr);
    }

    getline(input, line);
    while (getline(input, line)) {
        if (line.empty())
            break;
        ShopItem ring{0, 0, 0};

        stringstream stream(line);
        string ss;
        getline(stream, ss, ' ');
        getline(stream, ss, ' ');

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        ring.cost = stoi(ss);

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        ring.damage = stoi(ss);

        getline(stream, ss, ' ');
        while (ss == "")
            getline(stream, ss, ' ');

        ring.armor = stoi(ss);

        rings.push_back(ring);
    }

    auto play_game = [](Game *game) {
        int player_damage = max(1, game->player.damage - game->boss.armor);
        int boss_damage   = max(1, game->boss.damage - game->player.armor);
        cout << player_damage << " x " << boss_damage << " | "
             << ceil(game->boss.hp / (float)player_damage) << " x "
             << ceil(game->player.hp / (float)boss_damage) << endl;
        return (ceil(game->boss.hp / (float)player_damage) <=
                ceil(game->player.hp / (float)boss_damage));
    };
    auto recalculate = [&weapons, &armor, &rings](Game *game) {
        game->cost          = 0;
        game->player.armor  = 0;
        game->player.damage = 0;
        if (game->choices[0] != -1) {
            game->cost += weapons[game->choices[0]].cost;
            game->player.armor += weapons[game->choices[0]].armor;
            game->player.damage += weapons[game->choices[0]].damage;
        }
        if (game->choices[1] != -1) {
            game->cost += armor[game->choices[1]].cost;
            game->player.armor += armor[game->choices[1]].armor;
            game->player.damage += armor[game->choices[1]].damage;
        }
        if (game->choices[2] != -1) {
            game->cost += rings[game->choices[2]].cost;
            game->player.armor += rings[game->choices[2]].armor;
            game->player.damage += rings[game->choices[2]].damage;
        }
        if (game->choices[3] != -1) {
            game->cost += rings[game->choices[3]].cost;
            game->player.armor += rings[game->choices[3]].armor;
            game->player.damage += rings[game->choices[3]].damage;
        }
    };

    Player boss{103, 9, 2};

    Game game{
        Player{
            100,
            weapons[0].damage,
            0,
        },
        boss,
        {0, -1, -1, -1},
        weapons[0].cost,
    };

    int armor_size = armor.size(), weapons_size = weapons.size(),
        rings_size   = rings.size();
    int highest_cost = 0;
    for (int wpn = 0; wpn < weapons_size; wpn++) {
        for (int armr = -1; armr < armor_size; armr++) {
            for (int rng1 = -1; rng1 < rings_size; rng1++) {
                for (int rng2 = -1; rng2 < rings_size; rng2++) {
                    if (rng1 == rng2 && rng1 != -1)
                        continue;

                    auto prev_choices = game.choices;

                    game.choices[0] = wpn;
                    game.choices[1] = armr;
                    game.choices[2] = rng1;
                    game.choices[3] = rng2;

                    printf("%d, %d, %d, %d\n", game.choices[0], game.choices[1],
                           game.choices[2], game.choices[3]);

                    recalculate(&game);
                    cout << highest_cost << " " << game.cost << endl;
                    if (game.cost <= highest_cost || play_game(&game)) {
                        game.choices[0] = prev_choices[0];
                        game.choices[1] = prev_choices[1];
                        game.choices[2] = prev_choices[2];
                        game.choices[3] = prev_choices[3];
                    } else {
                        highest_cost = game.cost;
                    }
                }
            }
        }
    }

    cout << highest_cost << endl;

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
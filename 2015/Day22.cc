#include <bits/stdc++.h>
#include <fstream>
#include <iostream>
#include <stdio.h>
#include <vector>

using namespace std;

const int num_attacks = 5;
struct Attack {
    int id, cost, damage, heal, armor, mana, turns;
} attacks[num_attacks]{{0, 53, 4, 0, 0, 0, 0},
                       {1, 73, 2, 2, 0, 0, 0},
                       {2, 113, 0, 0, 7, 0, 6},
                       {3, 173, 3, 0, 0, 0, 6},
                       {4, 229, 0, 0, 0, 101, 5}};

struct Game {
    struct Player {
        int hp, mana, armor;
        vector<Attack> effects;
    } player;
    struct Boss {
        int hp, damage;
    } boss;

    int mana_spent;
};

void play_game(Game *game, int *lowest_mana_spent)
{
    // cout << "=====================" << endl;
    // printf("start game:\th:%d\tm:%d\ta:%d\tboss:%d\n", game->player.hp,
    //        game->player.mana, game->player.armor, game->boss.hp);

    auto apply_effects = [lowest_mana_spent](Game &g) {
        g.player.armor = 0;
        for (int i = 0; i < g.player.effects.size(); i++) {
            auto effect = &g.player.effects[i];
            if (effect->damage)
                g.boss.hp -= effect->damage;
            if (effect->heal)
                g.player.hp += effect->heal;
            if (effect->armor)
                g.player.armor = effect->armor;
            if (effect->mana)
                g.player.mana += effect->mana;

            if (--effect->turns <= 0)
                g.player.effects.erase(next(g.player.effects.begin(), i--));

            // printf("effect:\t\th:%d\tm:%d\ta:%d\tboss:%d\n",
            // game.player.hp,
            //        game.player.mana, game.player.armor, game.boss.hp);
        }

        if (g.boss.hp <= 0) {
            if (g.mana_spent < *lowest_mana_spent)
                *lowest_mana_spent = g.mana_spent;
            return true;
        }
        return false;
    };

    if (game->player.hp <= 0)
        return;
    if (apply_effects(*game))
        return;
    // if (game->boss.hp <= 0)
    //     return game->mana_spent;

    int i = 0;
    for (; i < num_attacks; i++) {
        //  You must have enough mana to cast a spell
        if (game->player.mana < attacks[i].cost)
            break;

        //  You cannot cast a spell that would start an effect which is
        // already active
        bool skip = false;
        for (auto &&effect : game->player.effects) {
            if (effect.id == i) {
                skip = true;
                break;
            }
        }
        if (skip)
            continue;

        Game new_game = *game;
        new_game.player.mana -= attacks[i].cost;
        new_game.mana_spent += attacks[i].cost;
        if (new_game.mana_spent >= *lowest_mana_spent)
            continue;

        // printf("%dchose attack:\th:%d\tp:%d\te:%d\td:%d\n", i,
        // attacks[i].heal,
        //        attacks[i].cost, attacks[i].turns > 0, attacks[i].damage);
        if (attacks[i].turns > 0) {
            Attack effect = attacks[i];
            new_game.player.effects.push_back(effect);
        } else {
            new_game.player.hp += attacks[i].heal;
            new_game.boss.hp -= attacks[i].damage;
        }

        // printf("player turn:\th:%d\tm:%d\ta:%d\tboss:%d\n",
        // new_game.player.hp,
        //        new_game.player.mana, new_game.player.armor,
        //        new_game.boss.hp);

        if (apply_effects(new_game))
            return;
        // apply_effects(&new_game);
        // if (new_game.boss.hp <= 0) {
        //     if (new_game.mana_spent < *lowest_mana_spent)
        //         *lowest_mana_spent = new_game.mana_spent;
        //     cout << "game ended with " << new_game.mana_spent << endl;
        //     continue;
        // }

        // the boss' attacks always deal at least 1 damage
        new_game.player.hp -=
            max(1, new_game.boss.damage - new_game.player.armor);
        // printf("boss attack:\td:%d\n",
        //        new_game.boss.damage - new_game.player.armor);

        // int new_game_mana_spent = play_game(&new_game, lowest_mana_spent);
        play_game(&new_game, lowest_mana_spent);
        // if (new_game_mana_spent < *lowest_mana_spent)
        //     *lowest_mana_spent = new_game_mana_spent;
        // cout << "2game ended with " << new_game_mana_spent << endl;
    }
    //  If you cannot afford to cast any spell, you lose
    // if (i == 0)
    //     return INT_MAX;
}

int part_1(ifstream &input)
{
    Game game{
        {50, 500, 0, {}},
        {51, 9},
        0,
    };

    int lowest_mana = INT_MAX;
    play_game(&game, &lowest_mana);
    cout << lowest_mana << endl;
    return 0;
}

// wow! either this is actually hard or i'm just high
void play_game2(Game *game, int *lowest_mana_spent)
{
    auto apply_effects = [lowest_mana_spent](Game &g) {
        g.player.armor = 0;
        for (int i = 0; i < g.player.effects.size(); i++) {
            auto effect = &g.player.effects[i];
            if (effect->damage)
                g.boss.hp -= effect->damage;
            if (effect->heal)
                g.player.hp += effect->heal;
            if (effect->armor)
                g.player.armor = effect->armor;
            if (effect->mana)
                g.player.mana += effect->mana;

            if (--effect->turns <= 0)
                g.player.effects.erase(next(g.player.effects.begin(), i--));
        }

        if (g.boss.hp <= 0) {
            if (g.mana_spent < *lowest_mana_spent)
                *lowest_mana_spent = g.mana_spent;
            return true;
        }
        return false;
    };

    if (--(game->player.hp) <= 0)
        return;
    if (apply_effects(*game))
        return;

    for (int i = 0; i < num_attacks; i++) {
        //  You must have enough mana to cast a spell
        if (game->player.mana < attacks[i].cost)
            break;

        //  You cannot cast a spell that would start an effect which is already
        //  active
        bool skip = false;
        for (auto &&effect : game->player.effects) {
            if (effect.id == i) {
                skip = true;
                break;
            }
        }
        if (skip)
            continue;

        Game new_game = *game;
        new_game.player.mana -= attacks[i].cost;
        new_game.mana_spent += attacks[i].cost;
        if (new_game.mana_spent >= *lowest_mana_spent)
            continue;

        if (attacks[i].turns > 0) {
            Attack effect = attacks[i];
            new_game.player.effects.push_back(effect);
        } else {
            new_game.player.hp += attacks[i].heal;
            new_game.boss.hp -= attacks[i].damage;
        }

        if (apply_effects(new_game))
            continue;

        // the boss' attacks always deal at least 1 damage
        new_game.player.hp -=
            max(1, new_game.boss.damage - new_game.player.armor);

        play_game2(&new_game, lowest_mana_spent);
    }
}

int part_2(ifstream &input)
{
    Game game{
        {50, 500, 0, {}},
        {51, 9},
        0,
    };

    int lowest_mana = INT_MAX;
    play_game2(&game, &lowest_mana);
    cout << lowest_mana << endl;
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
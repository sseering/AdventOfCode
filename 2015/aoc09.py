#!/usr/bin/env python3

# --- Day 9: All in a Single Night ---
#
# Every year, Santa manages to deliver all of his presents in a single night.
#
# This year, however, he has some new locations to visit; his elves have provided him the distances between every pair of locations. He can start and end at any two (different) locations he wants, but he must visit each location exactly once. What is the shortest distance he can travel to achieve this?
#
# For example, given the following distances:
#
# London to Dublin = 464
# London to Belfast = 518
# Dublin to Belfast = 141
#
# The possible routes are therefore:
#
# Dublin -> London -> Belfast = 982
# London -> Dublin -> Belfast = 605
# London -> Belfast -> Dublin = 659
# Dublin -> Belfast -> London = 659
# Belfast -> Dublin -> London = 605
# Belfast -> London -> Dublin = 982
#
# The shortest of these is London -> Dublin -> Belfast = 605, and so the answer is 605 in this example.
#
# What is the distance of the shortest route?
#
# To begin, get your puzzle input.
#
# The first half of this puzzle is complete! It provides one gold star: *
# --- Part Two ---
#
# The next year, just to show off, Santa decides to take the route with the longest distance instead.
#
# He can still start and end at any two (different) locations he wants, and he still must visit each location exactly once.
#
# For example, given the distances above, the longest route would be 982 via (for example) Dublin -> London -> Belfast.
#
# What is the distance of the longest route?
#
# Although it hasn't changed, you can still get your puzzle input.

import collections
import itertools

INPUT = """Tristram to AlphaCentauri = 34
Tristram to Snowdin = 100
Tristram to Tambi = 63
Tristram to Faerun = 108
Tristram to Norrath = 111
Tristram to Straylight = 89
Tristram to Arbre = 132
AlphaCentauri to Snowdin = 4
AlphaCentauri to Tambi = 79
AlphaCentauri to Faerun = 44
AlphaCentauri to Norrath = 147
AlphaCentauri to Straylight = 133
AlphaCentauri to Arbre = 74
Snowdin to Tambi = 105
Snowdin to Faerun = 95
Snowdin to Norrath = 48
Snowdin to Straylight = 88
Snowdin to Arbre = 7
Tambi to Faerun = 68
Tambi to Norrath = 134
Tambi to Straylight = 107
Tambi to Arbre = 40
Faerun to Norrath = 11
Faerun to Straylight = 66
Faerun to Arbre = 144
Norrath to Straylight = 115
Norrath to Arbre = 135
Straylight to Arbre = 127
"""

# INPUT = """London to Dublin = 464
# London to Belfast = 518
# Dublin to Belfast = 141
# """


def part12() -> None:
    all_places = set()
    all_trips = dict()

    for line in INPUT.splitlines():
        (loc_a, to_str, loc_b, eq_str, dist_str) = line.split()

        if to_str != "to" or eq_str != "=":
            raise Exception()

        start = max(loc_a, loc_b)
        end = min(loc_a, loc_b)
        dist = int(dist_str)

        all_trips[(start, end)] = dist
        all_places.add(start)
        all_places.add(end)

    min_len = None
    max_len = 0
    for perm in itertools.permutations(all_places):
        len_ = 0
        for (loc_a, loc_b) in zip(perm, perm[1:]):
            start = max(loc_a, loc_b)
            end = min(loc_a, loc_b)
            len_ += all_trips[(start, end)]

        max_len = max(max_len, len_)
        if min_len is None or min_len > len_:
            min_len = len_

    print(f'solution for part 1 is {min_len}')
    print(f'solution for part 1 is {max_len}')


def main() -> None:
    part12()
    print('done')


if __name__ == '__main__':
    main()

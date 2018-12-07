#!/usr/bin/env python3

# --- Day 13: Packet Scanners ---
#
# You need to cross a vast firewall. The firewall consists of several layers, each with a security scanner that moves back and forth across the layer. To succeed, you must not be detected by a scanner.
#
# By studying the firewall briefly, you are able to record (in your puzzle input) the depth of each layer and the range of the scanning area for the scanner within it, written as depth: range. Each layer has a thickness of exactly 1. A layer at depth 0 begins immediately inside the firewall; a layer at depth 1 would start immediately after that.
#
# For example, suppose you've recorded the following:
#
# 0: 3
# 1: 2
# 4: 4
# 6: 4
#
# This means that there is a layer immediately inside the firewall (with range 3), a second layer immediately after that (with range 2), a third layer which begins at depth 4 (with range 4), and a fourth layer which begins at depth 6 (also with range 4). Visually, it might look like this:
#
#  0   1   2   3   4   5   6
# [ ] [ ] ... ... [ ] ... [ ]
# [ ] [ ]         [ ]     [ ]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
# Within each layer, a security scanner moves back and forth within its range. Each security scanner starts at the top and moves down until it reaches the bottom, then moves up until it reaches the top, and repeats. A security scanner takes one picosecond to move one step. Drawing scanners as S, the first few picoseconds look like this:
#
#
# Picosecond 0:
#  0   1   2   3   4   5   6
# [S] [S] ... ... [S] ... [S]
# [ ] [ ]         [ ]     [ ]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
# Picosecond 1:
#  0   1   2   3   4   5   6
# [ ] [ ] ... ... [ ] ... [ ]
# [S] [S]         [S]     [S]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
# Picosecond 2:
#  0   1   2   3   4   5   6
# [ ] [S] ... ... [ ] ... [ ]
# [ ] [ ]         [ ]     [ ]
# [S]             [S]     [S]
#                 [ ]     [ ]
#
# Picosecond 3:
#  0   1   2   3   4   5   6
# [ ] [ ] ... ... [ ] ... [ ]
# [S] [S]         [ ]     [ ]
# [ ]             [ ]     [ ]
#                 [S]     [S]
#
# Your plan is to hitch a ride on a packet about to move through the firewall. The packet will travel along the top of each layer, and it moves at one layer per picosecond. Each picosecond, the packet moves one layer forward (its first move takes it into layer 0), and then the scanners move one step. If there is a scanner at the top of the layer as your packet enters it, you are caught. (If a scanner moves into the top of its layer while you are there, you are not caught: it doesn't have time to notice you before you leave.) If you were to do this in the configuration above, marking your current position with parentheses, your passage through the firewall would look like this:
#
# Initial state:
#  0   1   2   3   4   5   6
# [S] [S] ... ... [S] ... [S]
# [ ] [ ]         [ ]     [ ]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
# Picosecond 0:
#  0   1   2   3   4   5   6
# (S) [S] ... ... [S] ... [S]
# [ ] [ ]         [ ]     [ ]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
#  0   1   2   3   4   5   6
# ( ) [ ] ... ... [ ] ... [ ]
# [S] [S]         [S]     [S]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
#
# Picosecond 1:
#  0   1   2   3   4   5   6
# [ ] ( ) ... ... [ ] ... [ ]
# [S] [S]         [S]     [S]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
#  0   1   2   3   4   5   6
# [ ] (S) ... ... [ ] ... [ ]
# [ ] [ ]         [ ]     [ ]
# [S]             [S]     [S]
#                 [ ]     [ ]
#
#
# Picosecond 2:
#  0   1   2   3   4   5   6
# [ ] [S] (.) ... [ ] ... [ ]
# [ ] [ ]         [ ]     [ ]
# [S]             [S]     [S]
#                 [ ]     [ ]
#
#  0   1   2   3   4   5   6
# [ ] [ ] (.) ... [ ] ... [ ]
# [S] [S]         [ ]     [ ]
# [ ]             [ ]     [ ]
#                 [S]     [S]
#
#
# Picosecond 3:
#  0   1   2   3   4   5   6
# [ ] [ ] ... (.) [ ] ... [ ]
# [S] [S]         [ ]     [ ]
# [ ]             [ ]     [ ]
#                 [S]     [S]
#
#  0   1   2   3   4   5   6
# [S] [S] ... (.) [ ] ... [ ]
# [ ] [ ]         [ ]     [ ]
# [ ]             [S]     [S]
#                 [ ]     [ ]
#
#
# Picosecond 4:
#  0   1   2   3   4   5   6
# [S] [S] ... ... ( ) ... [ ]
# [ ] [ ]         [ ]     [ ]
# [ ]             [S]     [S]
#                 [ ]     [ ]
#
#  0   1   2   3   4   5   6
# [ ] [ ] ... ... ( ) ... [ ]
# [S] [S]         [S]     [S]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
#
# Picosecond 5:
#  0   1   2   3   4   5   6
# [ ] [ ] ... ... [ ] (.) [ ]
# [S] [S]         [S]     [S]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
#  0   1   2   3   4   5   6
# [ ] [S] ... ... [S] (.) [S]
# [ ] [ ]         [ ]     [ ]
# [S]             [ ]     [ ]
#                 [ ]     [ ]
#
#
# Picosecond 6:
#  0   1   2   3   4   5   6
# [ ] [S] ... ... [S] ... (S)
# [ ] [ ]         [ ]     [ ]
# [S]             [ ]     [ ]
#                 [ ]     [ ]
#
#  0   1   2   3   4   5   6
# [ ] [ ] ... ... [ ] ... ( )
# [S] [S]         [S]     [S]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
# In this situation, you are caught in layers 0 and 6, because your packet entered the layer when its scanner was at the top when you entered it. You are not caught in layer 1, since the scanner moved into the top of the layer once you were already there.
#
# The severity of getting caught on a layer is equal to its depth multiplied by its range. (Ignore layers in which you do not get caught.) The severity of the whole trip is the sum of these values. In the example above, the trip severity is 0*3 + 6*4 = 24.
#
# Given the details of the firewall you've recorded, if you leave immediately, what is the severity of your whole trip?
#
# To begin, get your puzzle input.
#
# --- Part Two ---
#
# Now, you need to pass through the firewall without being caught - easier said than done.
#
# You can't control the speed of the packet, but you can delay it any number of picoseconds. For each picosecond you delay the packet before beginning your trip, all security scanners move one step. You're not in the firewall during this time; you don't enter layer 0 until you stop delaying the packet.
#
# In the example above, if you delay 10 picoseconds (picoseconds 0 - 9), you won't get caught:
#
# State after delaying:
#  0   1   2   3   4   5   6
# [ ] [S] ... ... [ ] ... [ ]
# [ ] [ ]         [ ]     [ ]
# [S]             [S]     [S]
#                 [ ]     [ ]
#
# Picosecond 10:
#  0   1   2   3   4   5   6
# ( ) [S] ... ... [ ] ... [ ]
# [ ] [ ]         [ ]     [ ]
# [S]             [S]     [S]
#                 [ ]     [ ]
#
#  0   1   2   3   4   5   6
# ( ) [ ] ... ... [ ] ... [ ]
# [S] [S]         [S]     [S]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
#
# Picosecond 11:
#  0   1   2   3   4   5   6
# [ ] ( ) ... ... [ ] ... [ ]
# [S] [S]         [S]     [S]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
#  0   1   2   3   4   5   6
# [S] (S) ... ... [S] ... [S]
# [ ] [ ]         [ ]     [ ]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
#
# Picosecond 12:
#  0   1   2   3   4   5   6
# [S] [S] (.) ... [S] ... [S]
# [ ] [ ]         [ ]     [ ]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
#  0   1   2   3   4   5   6
# [ ] [ ] (.) ... [ ] ... [ ]
# [S] [S]         [S]     [S]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
#
# Picosecond 13:
#  0   1   2   3   4   5   6
# [ ] [ ] ... (.) [ ] ... [ ]
# [S] [S]         [S]     [S]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
#  0   1   2   3   4   5   6
# [ ] [S] ... (.) [ ] ... [ ]
# [ ] [ ]         [ ]     [ ]
# [S]             [S]     [S]
#                 [ ]     [ ]
#
#
# Picosecond 14:
#  0   1   2   3   4   5   6
# [ ] [S] ... ... ( ) ... [ ]
# [ ] [ ]         [ ]     [ ]
# [S]             [S]     [S]
#                 [ ]     [ ]
#
#  0   1   2   3   4   5   6
# [ ] [ ] ... ... ( ) ... [ ]
# [S] [S]         [ ]     [ ]
# [ ]             [ ]     [ ]
#                 [S]     [S]
#
#
# Picosecond 15:
#  0   1   2   3   4   5   6
# [ ] [ ] ... ... [ ] (.) [ ]
# [S] [S]         [ ]     [ ]
# [ ]             [ ]     [ ]
#                 [S]     [S]
#
#  0   1   2   3   4   5   6
# [S] [S] ... ... [ ] (.) [ ]
# [ ] [ ]         [ ]     [ ]
# [ ]             [S]     [S]
#                 [ ]     [ ]
#
#
# Picosecond 16:
#  0   1   2   3   4   5   6
# [S] [S] ... ... [ ] ... ( )
# [ ] [ ]         [ ]     [ ]
# [ ]             [S]     [S]
#                 [ ]     [ ]
#
#  0   1   2   3   4   5   6
# [ ] [ ] ... ... [ ] ... ( )
# [S] [S]         [S]     [S]
# [ ]             [ ]     [ ]
#                 [ ]     [ ]
#
# Because all smaller delays would get you caught, the fewest number of picoseconds you would need to delay to get through safely is 10.
#
# What is the fewest number of picoseconds that you need to delay the packet to pass through the firewall without being caught?
#
# Although it hasn't changed, you can still get your puzzle input.

from typing import Dict, Tuple, List

INPUT = """0: 4
1: 2
2: 3
4: 4
6: 6
8: 5
10: 6
12: 6
14: 6
16: 8
18: 8
20: 9
22: 12
24: 8
26: 8
28: 8
30: 12
32: 12
34: 8
36: 12
38: 10
40: 12
42: 12
44: 10
46: 12
48: 14
50: 12
52: 14
54: 14
56: 12
58: 14
60: 12
62: 14
64: 18
66: 14
68: 14
72: 14
76: 14
82: 14
86: 14
88: 18
90: 14
92: 17
"""

# INPUT = """0: 3
# 1: 2
# 4: 4
# 6: 4
# """


class Scanner:
    def __init__(self, depth: int, range_: int):
        if depth < 0:
            raise Exception()
        if range_ < 2:
            raise Exception()
        self._depth = depth
        self._range = range_
        self._scanner_pos = 0
        self._scanner_diff = 1

    def part_1_score(self) -> int:
        if self._scanner_pos == 0:
            return self._depth * self._range
        return 0

    def move_scanner(self):
        self._scanner_pos += self._scanner_diff
        if self._scanner_pos == 0:
            self._scanner_diff = 1
        elif self._scanner_pos == self._range - 1:
            self._scanner_diff = -1

    @staticmethod
    def from_input_lines(input_: str) -> Tuple[int, Dict[int, 'Scanner']]:
        res = dict()
        max_depth = -1
        for line in input_.splitlines():
            (depth_str, range_str) = line.split(':')
            depth_int = int(depth_str.strip())
            range_int = int(range_str.strip())
            max_depth = max(max_depth, depth_int)
            res[depth_int] = Scanner(depth_int, range_int)
        return (max_depth, res)


class BetterScanner:
    def __init__(self, depth: int, range_: int):
        if depth < 0:
            raise Exception()
        if range_ < 2:
            raise Exception()
        self._depth = depth
        self._range = range_

    def part_1_score(self) -> int:
        if self._depth % (self._range * 2 - 2) == 0:
            return self._depth * self._range
        return 0

    def part_2_score(self, delay: int) -> int:
        if (self._depth + delay) % (self._range * 2 - 2) == 0:
            return 1
        return 0

    def __repr__(self):
        return '{0} {1}'.format(self._depth, self._range * 2 - 2)


    @staticmethod
    def from_input_lines(input_: str) -> List['BetterScanner']:
        res = []
        for line in input_.splitlines():
            (depth_str, range_str) = line.split(':')
            depth_int = int(depth_str.strip())
            range_int = int(range_str.strip())
            res.append(BetterScanner(depth_int, range_int))
        return res


def bad_part1() -> None:
    (max_depth, scanners) = Scanner.from_input_lines(INPUT)
    my_pos = -1
    score = 0
    print(f'max_depth is {max_depth}')

    while True:
        my_pos += 1
        if my_pos > max_depth:
            break
        scanner = scanners.get(my_pos)
        if scanner is not None:
            score += scanner.part_1_score()
        for scanner in scanners.values():
            scanner.move_scanner()

    print(f'solution to part 1 is {score}')

def part1() -> None:
    scanners = BetterScanner.from_input_lines(INPUT)
    score = sum(_.part_1_score() for _ in scanners)
    print(f'solution to part 1 is {score}')


def part2() -> None:
    scanners = BetterScanner.from_input_lines(INPUT)
    delay = -1
    score = 1

    for scanner in scanners:
        print(repr(scanner))

    while score > 0:
        delay += 1
        score = sum(_.part_2_score(delay) for _ in scanners)

    print(f'solution to part 2 is {delay}')


def main() -> None:
    bad_part1()
    part1()
    part2()
    print('done')


if __name__ == '__main__':
    main()

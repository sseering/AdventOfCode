#!/usr/bin/env python3

# -- Day 16: Permutation Promenade ---
#
# You come upon a very unusual sight; a group of programs here appear to be dancing.
#
# There are sixteen programs in total, named a through p. They start by standing in a line: a stands in position 0, b stands in position 1, and so on until p, which stands in position 15.
#
# The programs' dance consists of a sequence of dance moves:
#
#     Spin, written sX, makes X programs move from the end to the front, but maintain their order otherwise. (For example, s3 on abcde produces cdeab).
#     Exchange, written xA/B, makes the programs at positions A and B swap places.
#     Partner, written pA/B, makes the programs named A and B swap places.
#
# For example, with only five programs standing in a line (abcde), they could do the following dance:
#
#     s1, a spin of size 1: eabcd.
#     x3/4, swapping the last two programs: eabdc.
#     pe/b, swapping programs e and b: baedc.
#
# After finishing their dance, the programs end up in order baedc.
#
# You watch the dance for a while and record their dance moves (your puzzle input). In what order are the programs standing after their dance?
#
# --- Part Two ---
#
# Now that you're starting to get a feel for the dance moves, you turn your attention to the dance as a whole.
#
# Keeping the positions they ended up in from their previous dance, the programs perform it again and again: including the first dance, a total of one billion (1000000000) times.
#
# In the example above, their second dance would begin with the order baedc, and use the same dance moves:
#
#     s1, a spin of size 1: cbaed.
#     x3/4, swapping the last two programs: cbade.
#     pe/b, swapping programs e and b: ceadb.
#
# In what order are the programs standing after their billion dances?

import fileinput
from typing import List, Optional


def part1(startline: Optional[str] = None) -> None:
    if startline is None:
        danceline = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p']
    else:
        danceline = [_ for _ in startline]

    def do_op(op_str: str):
        nonlocal danceline

        if op_str[0] == 's':
            i = int(op_str[1:]) * -1
            danceline = danceline[i:] + danceline[:i]
        elif op_str[0] == 'x':
            (a_str, b_str) = op_str[1:].split('/')
            a = int(a_str)
            b = int(b_str)
            (danceline[a], danceline[b]) = (danceline[b], danceline[a])
        elif op_str[0] == 'p':
            (a_str, b_str) = op_str[1:].split('/')
            a = danceline.index(a_str)
            b = danceline.index(b_str)
            (danceline[a], danceline[b]) = (danceline[b], danceline[a])
        else:
            raise Exception

    for line in fileinput.input('input16.txt'):
        for op in line.split(','):
            do_op(op)

    return ''.join(danceline)


def part2() -> None:
    idx_helpers = "kbednhopmfcjilag"
    danceline = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p']
    map_indices = [danceline.index(_) for _ in idx_helpers]

    for _ in range(1000000000):
        newline = list([danceline[i] for i in map_indices])
        danceline = newline

    print(''.join(danceline))


def better_part_2() -> None:
    danceline_begin = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p']
    danceline_end = ['k', 'b', 'e', 'd', 'n', 'h', 'o', 'p', 'm', 'f', 'c', 'j', 'i', 'l', 'a', 'g']
    mapping = [danceline_begin.index(_) for _ in danceline_end]
    danceline = danceline_begin

    def do_map():
        nonlocal danceline
        newline = [danceline[i] for i in mapping]
        danceline = newline

    print(''.join(danceline))
    do_map()
    print(''.join(danceline))
    do_map()
    print(''.join(danceline))


def main() -> None:
    print("1")
    print(part1())
    print("2")
    print(part1(part1()))
    print("3")
    print(part1(part1(part1())))
    print('---')
    better_part_2()
    print('done')


if __name__ == '__main__':
    main()

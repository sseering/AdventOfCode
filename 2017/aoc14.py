#!/usr/bin/env python3

# --- Day 14: Disk Defragmentation ---
#
# Suddenly, a scheduled job activates the system's disk defragmenter. Were the situation different, you might sit and watch it for a while, but today, you just don't have that kind of time. It's soaking up valuable system resources that are needed elsewhere, and so the only option is to help it finish its task as soon as possible.
#
# The disk in question consists of a 128x128 grid; each square of the grid is either free or used. On this disk, the state of the grid is tracked by the bits in a sequence of knot hashes.
#
# A total of 128 knot hashes are calculated, each corresponding to a single row in the grid; each hash contains 128 bits which correspond to individual grid squares. Each bit of a hash indicates whether that square is free (0) or used (1).
#
# The hash inputs are a key string (your puzzle input), a dash, and a number from 0 to 127 corresponding to the row. For example, if your key string were flqrgnkx, then the first row would be given by the bits of the knot hash of flqrgnkx-0, the second row from the bits of the knot hash of flqrgnkx-1, and so on until the last row, flqrgnkx-127.
#
# The output of a knot hash is traditionally represented by 32 hexadecimal digits; each of these digits correspond to 4 bits, for a total of 4 * 32 = 128 bits. To convert to bits, turn each hexadecimal digit to its equivalent binary value, high-bit first: 0 becomes 0000, 1 becomes 0001, e becomes 1110, f becomes 1111, and so on; a hash that begins with a0c2017... in hexadecimal would begin with 10100000110000100000000101110000... in binary.
#
# Continuing this process, the first 8 rows and columns for key flqrgnkx appear as follows, using # to denote used squares, and . to denote free ones:
#
# ##.#.#..-->
# .#.#.#.#
# ....#.#.
# #.#.##.#
# .##.#...
# ##..#..#
# .#...#..
# ##.#.##.-->
# |      |
# V      V
#
# In this example, 8108 squares are used across the entire 128x128 grid.
#
# Given your actual key string, how many squares are used?
#
# Your puzzle input is jxqlasbh.
#
# The first half of this puzzle is complete! It provides one gold star: *
# --- Part Two ---
#
# Now, all the defragmenter needs to know is the number of regions. A region is a group of used squares that are all adjacent, not including diagonals. Every used square is in exactly one region: lone used squares form their own isolated regions, while several adjacent squares all count as a single region.
#
# In the example above, the following nine regions are visible, each marked with a distinct digit:
#
# 11.2.3..-->
# .1.2.3.4
# ....5.6.
# 7.8.55.9
# .88.5...
# 88..5..8
# .8...8..
# 88.8.88.-->
# |      |
# V      V
#
# Of particular interest is the region marked 8; while it does not appear contiguous in this small view, all of the squares marked 8 are connected when considering the whole 128x128 grid. In total, in this example, 1242 regions are present.
#
# How many regions are present given your key string?
#
# Your puzzle input is still jxqlasbh.

from typing import List, Tuple
import functools
import operator


def advent_reverse(in_list: List[int], start: int, length: int) -> List[int]:
    res = [_ for _ in in_list]
    list_len = len(in_list)
    for off in range(length):
        dst_idx = (start + off) % list_len
        src_idx = (start + length - 1 - off) % list_len
        res[dst_idx] = in_list[src_idx]
    return res


def do_reverses(values: List[int], length_list: List[int], start: int, skip: int) -> Tuple[List[int], int, int]:
    for reverse_len in length_list:
        values = advent_reverse(values, start, reverse_len)
        start += skip + reverse_len
        skip += 1

    return (values, start, skip)


def knot_hash(input_: str) -> List[int]:
    skip = 0
    start = 0
    values = list(range(256))
    length_list = [int(_) for _ in input_.encode("ASCII")] + [17, 31, 73, 47, 23]

    for _ in range(64):
        (values, start, skip) = do_reverses(values, length_list, start, skip)

    xored = []
    for block_id in range(16):
        xored.append(functools.reduce(operator.xor, values[block_id * 16:(block_id + 1) * 16]))

    res =[]
    for bits in xored:
        res.append(1 if 0x80 & bits > 0 else 0)
        res.append(1 if 0x40 & bits > 0 else 0)
        res.append(1 if 0x20 & bits > 0 else 0)
        res.append(1 if 0x10 & bits > 0 else 0)
        res.append(1 if 0x08 & bits > 0 else 0)
        res.append(1 if 0x04 & bits > 0 else 0)
        res.append(1 if 0x02 & bits > 0 else 0)
        res.append(1 if 0x01 & bits > 0 else 0)
    return res


def part1() -> None:
    answer = 0
    key = "jxqlasbh"
    # key = "flqrgnkx"

    for i in range(128):
        answer += sum(knot_hash(key + "-" + str(i)))

    print(f'solution for part 1 is {answer}')


def part2() -> None:
    num_full = 0
    key = "jxqlasbh"
    # key = "flqrgnkx"
    answer = 0
    disk = []

    for i in range(128):
        disk_line = knot_hash(key + "-" + str(i))
        num_full += sum(disk_line)
        disk.append(disk_line)

    x = 0
    y = 0

    def eat_region(x: int, y: int) -> int:
        if x < 0 or x > 127:
            return 0
        if y < 0 or y > 127:
            return 0
        if disk[x][y] <= 0:
            return 0
        disk[x][y] = 0
        return 1 + eat_region(x+1, y) + eat_region(x-1, y) + eat_region(x, y+1) + eat_region(x, y-1)

    while num_full > 0:
        while disk[x][y] <= 0:
            x += 1
            if x > 127:
                x = 0
                y += 1
            if y > 127:
                raise Exception()

        num_full -= eat_region(x, y)
        answer += 1

    print(f'solution for part 2 is {answer}')


def main() -> None:
    part1()
    part2()
    print('done')


if __name__ == '__main__':
    main()

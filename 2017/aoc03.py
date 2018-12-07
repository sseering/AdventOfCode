#!/usr/bin/env python3

# --- Day 3: Spiral Memory ---
#
# You come across an experimental new kind of memory stored on an infinite two-dimensional grid.
#
# Each square on the grid is allocated in a spiral pattern starting at a location marked 1 and then counting up while spiraling outward. For example, the first few squares are allocated like this:
#
# 17  16  15  14  13
# 18   5   4   3  12
# 19   6   1   2  11
# 20   7   8   9  10
# 21  22  23---> ...
#
# While this is very space-efficient (no squares are skipped), requested data must be carried back to square 1 (the location of the only access port for this memory system) by programs that can only move up, down, left, or right. They always take the shortest path: the Manhattan Distance between the location of the data and square 1.
#
# For example:
#
#     Data from square 1 is carried 0 steps, since it's at the access port.
#     Data from square 12 is carried 3 steps, such as: down, left, left.
#     Data from square 23 is carried only 2 steps: up twice.
#     Data from square 1024 must be carried 31 steps.
#
# How many steps are required to carry the data from the square identified in your puzzle input all the way to the access port?
#
# Your puzzle input is 277678.
#
# --- Part Two ---
#
# As a stress test on the system, the programs here clear the grid and then store the value 1 in square 1. Then, in the same allocation order as shown above, they store the sum of the values in all adjacent squares, including diagonals.
#
# So, the first few squares' values are chosen as follows:
#
#     Square 1 starts with the value 1.
#     Square 2 has only one adjacent filled square (with value 1), so it also stores 1.
#     Square 3 has both of the above squares as neighbors and stores the sum of their values, 2.
#     Square 4 has all three of the aforementioned squares as neighbors and stores the sum of their values, 4.
#     Square 5 only has the first and fourth squares as neighbors, so it gets the value 5.
#
# Once a square is written, its value does not change. Therefore, the first few squares would receive the following values:
#
# 147  142  133  122   59
# 304    5    4    2   57
# 330   10    1    1   54
# 351   11   23   25   26
# 362  747  806--->   ...
#
# What is the first value written that is larger than your puzzle input?
#
# Your puzzle input is still 277678.

from typing import Dict, Tuple


INPUT = 277678


def part1() -> None:
    (x, y) = (0, 0)
    (minx, miny) = (0, 0)
    (maxx, maxy) = (0, 0)
    (dx, dy) = (1, 0)
    to_place = 1

    while to_place < INPUT:
        to_place += 1
        (x, y) = (x + dx, y + dy)

        rotate = False
        if x > maxx:
            rotate = True
            maxx = x
        elif x < minx:
            rotate = True
            minx = x
        elif y > maxy:
            rotate = True
            maxy = y
        elif y < miny:
            rotate = True
            miny = y

        if rotate:
            (dx, dy) = (-dy, dx)

    print('the soultion to part 1 is {0}'.format(abs(x) + abs(y)))


def part2() -> None:
    spiral = {(0,0): 1}

    (x, y) = (1, 0)
    (minx, miny) = (0, 0)
    (maxx, maxy) = (1, 0)
    (dx, dy) = (0, 1)

    val = 0

    def calc_spiral_sum(spiral: Dict[Tuple[int, int,], int], x: int, y: int) -> int:
        return sum(spiral.get((x + dx, y + dy), 0) for (dx, dy) in [(0, 1), (0, -1), (1, 0), (1, 1), (1, -1), (-1, 0), (-1, 1), (-1, -1)])

    while val <= INPUT:
        val = calc_spiral_sum(spiral, x, y)
        spiral[(x, y)] = val
        (x, y) = (x + dx, y + dy)

        rotate = False
        if x > maxx:
            rotate = True
            maxx = x
        elif x < minx:
            rotate = True
            minx = x
        elif y > maxy:
            rotate = True
            maxy = y
        elif y < miny:
            rotate = True
            miny = y

        if rotate:
            (dx, dy) = (-dy, dx)

    print(f'the solution for part 2 is {val}')


def main() -> None:
    # No code today. For part 1 I realized that the bottom right corner of each spiral was the sequence of odd squares. I found the smallest odd square smaller than my input and just counted from there.
    #
    # For part 2 the sequence is listed on OEIS. https://oeis.org/A141481

    part2()
    print('done')


if __name__ == '__main__':
    main()

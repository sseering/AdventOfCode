#!/usr/bin/env python3

# --- Day 10: Elves Look, Elves Say ---
#
# Today, the Elves are playing a game called look-and-say. They take turns making sequences by reading aloud the previous sequence and using that reading as the next sequence. For example, 211 is read as "one two, two ones", which becomes 1221 (1 2, 2 1s).
#
# Look-and-say sequences are generated iteratively, using the previous value as input for the next step. For each step, take the previous value, and replace each run of digits (like 111) with the number of digits (3) followed by the digit itself (1).
#
# For example:
#
#     1 becomes 11 (1 copy of digit 1).
#     11 becomes 21 (2 copies of digit 1).
#     21 becomes 1211 (one 2 followed by one 1).
#     1211 becomes 111221 (one 1, one 2, and two 1s).
#     111221 becomes 312211 (three 1s, two 2s, and one 1).
#
# Starting with the digits in your puzzle input, apply this process 40 times. What is the length of the result?
#
# Your puzzle input is 1321131112.
#
# The first half of this puzzle is complete! It provides one gold star: *
# --- Part Two ---
#
# Neat, right? You might also enjoy hearing John Conway talking about this sequence (that's Conway of Conway's Game of Life fame).
#
# Now, starting again with the digits in your puzzle input, apply this process 50 times. What is the length of the new result?
#
# Your puzzle input is still 1321131112.

INPUT = '1321131112'


def partx(n: int) -> None:
    def procss(s: str) -> str:
        last_char = None
        num_last_char = 0
        result = ''

        for char in s:
            if char != last_char:
                if last_char is not None:
                    result += str(num_last_char) + last_char
                num_last_char = 1
                last_char = char
            else:
                num_last_char += 1

        result += str(num_last_char) + last_char
        return result

    v = INPUT

    for _ in range(n):
        v = procss(v)

    l = len(v)

    print(f'solution for part ? is {l}')


def part1() -> None:
    partx(40)


def part2() -> None:
    conway_constant = 1.30357726903429639125709911215255189073070250465940487575486139062855088785246155712681576686442522555
    print('alt {0}'.format(492982 * conway_constant ** 10))
    partx(50)


def main() -> None:
    part1()
    part2()
    print('done')


if __name__ == '__main__':
    main()

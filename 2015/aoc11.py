#!/usr/bin/env python3

# --- Day 11: Corporate Policy ---
#
# Santa's previous password expired, and he needs help choosing a new one.
#
# To help him remember his new password after the old one expires, Santa has devised a method of coming up with a password based on the previous one. Corporate policy dictates that passwords must be exactly eight lowercase letters (for security reasons), so he finds his new password by incrementing his old password string repeatedly until it is valid.
#
# Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so on. Increase the rightmost letter one step; if it was z, it wraps around to a, and repeat with the next letter to the left until one doesn't wrap around.
#
# Unfortunately for Santa, a new Security-Elf recently started, and he has imposed some additional password requirements:
#
#     Passwords must include one increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
#     Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters and are therefore confusing.
#     Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.
#
# For example:
#
#     hijklmmn meets the first requirement (because it contains the straight hij) but fails the second requirement requirement (because it contains i and l).
#     abbceffg meets the third requirement (because it repeats bb and ff) but fails the first requirement.
#     abbcegjk fails the third requirement, because it only has one double letter (bb).
#     The next password after abcdefgh is abcdffaa.
#     The next password after ghijklmn is ghjaabcc, because you eventually skip all the passwords that start with ghi..., since i is not allowed.
#
# Given Santa's current password (your puzzle input), what should his next password be?
#
# Your puzzle input is hxbxwxba.
#
# The first half of this puzzle is complete! It provides one gold star: *
# --- Part Two ---
#
# Santa's password expired again. What's the next one?
#
# Your puzzle input is still hxbxwxba.

from typing import List, Tuple


def part12() -> None:
    LETTARS = 'abcdefghjkmnpqrstuvwxyz'

    val = [_ for _ in reversed('hxbxwxba')]

    def next_lettar(lettar: str) -> str:
        idx = LETTARS.find(lettar)
        if idx < 0 or (idx + 1) >= len(LETTARS):
            return None
        return LETTARS[idx + 1]

    def next_lettar_with_overflow(lettar: str) -> Tuple[str, bool]:
        v = next_lettar(lettar)
        return (v, False) if v is not None else ('a', True)

    def is_ok_pw(pw: List[str]) -> bool:
        first_pair = None
        second_pair = None
        for (a, b) in zip(pw, pw[1:]):
            if a != b:
                continue

            if first_pair is None:
                first_pair = a
            elif first_pair != a:
                second_pair = a
                break

        rising_seq = False
        for (a, b, c) in zip(pw, pw[1:], pw[2:]):
            if b == next_lettar(c) and a == next_lettar(b):
                rising_seq = True
                break

        return rising_seq and second_pair is not None

    def iterate_pw(pw: List[str]) -> List[str]:
        result = [_ for _ in pw]  # make copy of array

        idx = -1
        overflow = True
        while overflow:
            idx += 1
            (lettar, overflow) = next_lettar_with_overflow(result[idx])
            result[idx] = lettar

        return result

    while True:
        val = iterate_pw(val)
        if is_ok_pw(val):
            break

    print('solution for part 1 is {0}'.format(''.join(reversed(val))))

    while True:
        val = iterate_pw(val)
        if is_ok_pw(val):
            break

    print('solution for part 2 is {0}'.format(''.join(reversed(val))))


def main() -> None:
    part12()
    print('done')


if __name__ == '__main__':
    main()

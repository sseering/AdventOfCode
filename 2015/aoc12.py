#!/usr/bin/env python3

# --- Day 12: JSAbacusFramework.io ---
#
# Santa's Accounting-Elves need help balancing the books after a recent order. Unfortunately, their accounting software uses a peculiar storage format. That's where you come in.
#
# They have a JSON document which contains a variety of things: arrays ([1,2,3]), objects ({"a":1, "b":2}), numbers, and strings. Your first job is to simply find all of the numbers throughout the document and add them together.
#
# For example:
#
#     [1,2,3] and {"a":2,"b":4} both have a sum of 6.
#     [[[3]]] and {"a":{"b":4},"c":-1} both have a sum of 3.
#     {"a":[-1,1]} and [-1,{"a":1}] both have a sum of 0.
#     [] and {} both have a sum of 0.
#
# You will not encounter any strings containing numbers.
#
# What is the sum of all numbers in the document?
#
# --- Part Two ---
#
# Uh oh - the Accounting-Elves have realized that they double-counted everything red.
#
# Ignore any object (and all of its children) which has any property with the value "red". Do this only for objects ({...}), not arrays ([...]).
#
#     [1,2,3] still has a sum of 6.
#     [1,{"c":"red","b":2},3] now has a sum of 4, because the middle object is ignored.
#     {"d":"red","e":[1,2,3,4],"f":5} now has a sum of 0, because the entire structure is ignored.
#     [1,"red",5] has a sum of 6, because "red" in an array has no effect.
#
# Although it hasn't changed, you can still get your puzzle input.

import json
import fileinput


def part1() -> None:
    json_str = ""

    for line in fileinput.input('input12.json'):
        json_str += line.strip()

    json_dict = json.loads(json_str)

    sum_ = 0

    def walk(json_dict):
        nonlocal sum_
        if isinstance(json_dict, int):
            sum_ += json_dict
        elif isinstance(json_dict, float):
            sum_ += json_dict
        elif isinstance(json_dict, list):
            for _ in json_dict:
                walk(_)
        elif isinstance(json_dict, dict):
            for _ in json_dict.values():
                walk(_)

    walk(json_dict)

    print(f'solution for part 1 is {sum_}')


def part2() -> None:
    json_str = ""

    for line in fileinput.input('input12.json'):
        json_str += line.strip()

    json_dict = json.loads(json_str)

    sum_ = 0

    def walk(json_dict):
        nonlocal sum_

        if isinstance(json_dict, int):
            sum_ += json_dict
        elif isinstance(json_dict, float):
            sum_ += json_dict
        elif isinstance(json_dict, list):
            for _ in json_dict:
                walk(_)
        elif isinstance(json_dict, dict):
            vs = json_dict.values()
            if 'red' in vs:
                return
            for _ in vs:
                walk(_)

    walk(json_dict)

    print(f'solution for part 2 is {sum_}')


def main() -> None:
    part1()
    part2()
    print('done')


if __name__ == '__main__':
    main()

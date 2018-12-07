#!/usr/bin/env python3

# --- Day 9: Stream Processing ---
#
# A large stream blocks your path. According to the locals, it's not safe to cross the stream at the moment because it's full of garbage. You look down at the stream; rather than water, you discover that it's a stream of characters.
#
# You sit for a while and record part of the stream (your puzzle input). The characters represent groups - sequences that begin with { and end with }. Within a group, there are zero or more other things, separated by commas: either another group or garbage. Since groups can contain other groups, a } only closes the most-recently-opened unclosed group - that is, they are nestable. Your puzzle input represents a single, large group which itself contains many smaller ones.
#
# Sometimes, instead of a group, you will find garbage. Garbage begins with < and ends with >. Between those angle brackets, almost any character can appear, including { and }. Within garbage, < has no special meaning.
#
# In a futile attempt to clean up the garbage, some program has canceled some of the characters within it using !: inside garbage, any character that comes after ! should be ignored, including <, >, and even another !.
#
# You don't see any characters that deviate from these rules. Outside garbage, you only find well-formed groups, and garbage always terminates according to the rules above.
#
# Here are some self-contained pieces of garbage:
#
#     <>, empty garbage.
#     <random characters>, garbage containing random characters.
#     <<<<>, because the extra < are ignored.
#     <{!>}>, because the first > is canceled.
#     <!!>, because the second ! is canceled, allowing the > to terminate the garbage.
#     <!!!>>, because the second ! and the first > are canceled.
#     <{o"i!a,<{i<a>, which ends at the first >.
#
# Here are some examples of whole streams and the number of groups they contain:
#
#     {}, 1 group.
#     {{{}}}, 3 groups.
#     {{},{}}, also 3 groups.
#     {{{},{},{{}}}}, 6 groups.
#     {<{},{},{{}}>}, 1 group (which itself contains garbage).
#     {<a>,<a>,<a>,<a>}, 1 group.
#     {{<a>},{<a>},{<a>},{<a>}}, 5 groups.
#     {{<!>},{<!>},{<!>},{<a>}}, 2 groups (since all but the last > are canceled).
#
# Your goal is to find the total score for all groups in your input. Each group is assigned a score which is one more than the score of the group that immediately contains it. (The outermost group gets a score of 1.)
#
#     {}, score of 1.
#     {{{}}}, score of 1 + 2 + 3 = 6.
#     {{},{}}, score of 1 + 2 + 2 = 5.
#     {{{},{},{{}}}}, score of 1 + 2 + 3 + 3 + 3 + 4 = 16.
#     {<a>,<a>,<a>,<a>}, score of 1.
#     {{<ab>},{<ab>},{<ab>},{<ab>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
#     {{<!!>},{<!!>},{<!!>},{<!!>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
#     {{<a!>},{<a!>},{<a!>},{<ab>}}, score of 1 + 2 = 3.
#
# What is the total score for all groups in your input?
#
# To begin, get your puzzle input.
#
# The first half of this puzzle is complete! It provides one gold star: *
# --- Part Two ---
#
# Now, you're ready to remove the garbage.
#
# To prove you've removed it, you need to count all of the characters within the garbage. The leading and trailing < and > don't count, nor do any canceled characters or the ! doing the canceling.
#
#     <>, 0 characters.
#     <random characters>, 17 characters.
#     <<<<>, 3 characters.
#     <{!>}>, 2 characters.
#     <!!>, 0 characters.
#     <!!!>>, 0 characters.
#     <{o"i!a,<{i<a>, 10 characters.
#
# How many non-canceled characters are within the garbage in your puzzle input?

from typing import Iterable


def read_input() -> str:
    with open('input09.txt', mode='rt') as f:
        return f.read().strip()


def apply_negations(in_iter: Iterable[str]) -> Iterable[str]:
    skip = False
    for c in in_iter:
        if skip:
            skip = False
            continue
        if c == "!":
            skip = True
            continue
        yield c


def strip_garbage(in_iter: Iterable[str]) -> Iterable[str]:
    outside_garbage = True
    for c in in_iter:
        if outside_garbage:
            if c == "<":
                outside_garbage = False
            else:
                yield c
        else:
            outside_garbage = c == ">"


class BaumKnoten:
    def __init__(self, parent: 'BaumKnoten' = None):
        self._score = parent._score + 1 if parent is not None else 0
        self.parent = parent
        self._first_child = None
        self._next_sibling = None

    def add_child_and_return_it(self) -> 'BaumKnoten':
        res = BaumKnoten(self)

        if self._first_child is None:
            self._first_child = res
            return res

        add_here = self._first_child
        while add_here._next_sibling is not None:
            add_here = add_here._next_sibling
        add_here._next_sibling = res
        return res

    def _walk_tree(self) -> Iterable['BaumKnoten']:
        if self._first_child is not None:
            yield from self._first_child._walk_tree()
        if self._next_sibling is not None:
            yield from self._next_sibling._walk_tree()
        yield self

    def recursive_sum_score(self) -> int:
        return sum(_._score for _ in self._walk_tree())


def solve_part_1(input_str: str) -> int:
    root = BaumKnoten()
    current_node = root
    for c in strip_garbage(apply_negations(input_str)):
        if c == "{":
            current_node = current_node.add_child_and_return_it()
        elif c == "}":
            current_node = current_node.parent
        elif c != ",":
            print(f'unknown char {c}')

    return root.recursive_sum_score()


def part1() -> None:
    # print(solve_part_1('{}'), 1)
    # print(solve_part_1('{{{}}}'), 6)
    # print(solve_part_1('{{},{}}'), 5)
    # print(solve_part_1('{{{},{},{{}}}}'), 16)
    # print(solve_part_1('{<a>,<a>,<a>,<a>}'), 1)
    # print(solve_part_1('{{<ab>},{<ab>},{<ab>},{<ab>}}'), 9)
    # print(solve_part_1('{{<!!>},{<!!>},{<!!>},{<!!>}}'), 9)
    # print(solve_part_1('{{<a!>},{<a!>},{<a!>},{<ab>}}'), 3)
    input_str = read_input()
    print('solution to part 1 is {0}'.format(solve_part_1(input_str)))


def part2() -> None:
    input_str = read_input()
    in_iter = apply_negations(input_str)

    outside_garbage = True
    num_garbage_payload = 0
    for c in in_iter:
        if outside_garbage:
            if c == "<":
                outside_garbage = False
        else:
            if c == ">":
                outside_garbage = True
            else:
                num_garbage_payload += 1

    print(f'solution part 2 is {num_garbage_payload}')


def main() -> None:
    part1()
    part2()
    print('done')


if __name__ == '__main__':
    main()

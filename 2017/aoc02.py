#!/usr/bin/env python3

# --- Day 2: Corruption Checksum ---
#
# As you walk through the door, a glowing humanoid shape yells in your direction. "You there! Your state appears to be idle. Come help us repair the corruption in this spreadsheet - if we take another millisecond, we'll have to display an hourglass cursor!"
#
# The spreadsheet consists of rows of apparently-random numbers. To make sure the recovery process is on the right track, they need you to calculate the spreadsheet's checksum. For each row, determine the difference between the largest value and the smallest value; the checksum is the sum of all of these differences.
#
# For example, given the following spreadsheet:
#
# 5 1 9 5
# 7 5 3
# 2 4 6 8
#
#     The first row's largest and smallest values are 9 and 1, and their difference is 8.
#     The second row's largest and smallest values are 7 and 3, and their difference is 4.
#     The third row's difference is 6.
#
# In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
#
# What is the checksum for the spreadsheet in your puzzle input?
#
# To begin, get your puzzle input.
#
# The first half of this puzzle is complete! It provides one gold star: *
# --- Part Two ---
#
# "Great work; looks like we're on the right track after all. Here's a star for your effort." However, the program seems a little worried. Can programs be worried?
#
# "Based on what we're seeing, it looks like all the User wanted is some information about the evenly divisible values in the spreadsheet. Unfortunately, none of us are equipped for that kind of calculation - most of us specialize in bitwise operations."
#
# It sounds like the goal is to find the only two numbers in each row where one evenly divides the other - that is, where the result of the division operation is a whole number. They would like you to find those numbers on each line, divide them, and add up each line's result.
#
# For example, given the following spreadsheet:
#
# 5 9 2 8
# 9 4 7 3
# 3 8 6 5
#
#     In the first row, the only two numbers that evenly divide are 8 and 2; the result of this division is 4.
#     In the second row, the two numbers are 9 and 3; the result is 3.
#     In the third row, the result is 2.
#
# In this example, the sum of the results would be 4 + 3 + 2 = 9.
#
# What is the sum of each row's result in your puzzle input?
#
# Although it hasn't changed, you can still get your puzzle input.

from itertools import combinations


INPUT = """4168	3925	858	2203	440	185	2886	160	1811	4272	4333	2180	174	157	361	1555
150	111	188	130	98	673	408	632	771	585	191	92	622	158	537	142
5785	5174	1304	3369	3891	131	141	5781	5543	4919	478	6585	116	520	673	112
5900	173	5711	236	2920	177	3585	4735	2135	2122	5209	265	5889	233	4639	5572
861	511	907	138	981	168	889	986	980	471	107	130	596	744	251	123
2196	188	1245	145	1669	2444	656	234	1852	610	503	2180	551	2241	643	175
2051	1518	1744	233	2155	139	658	159	1178	821	167	546	126	974	136	1946
161	1438	3317	4996	4336	2170	130	4987	3323	178	174	4830	3737	4611	2655	2743
3990	190	192	1630	1623	203	1139	2207	3994	1693	1468	1829	164	4391	3867	3036
116	1668	1778	69	99	761	201	2013	837	1225	419	120	1920	1950	121	1831
107	1006	92	807	1880	1420	36	1819	1039	1987	114	2028	1771	25	85	430
5295	1204	242	479	273	2868	3453	6095	5324	6047	5143	293	3288	3037	184	987
295	1988	197	2120	199	1856	181	232	564	1914	1691	210	1527	1731	1575	31
191	53	714	745	89	899	854	679	45	81	726	801	72	338	95	417
219	3933	6626	2137	3222	1637	5312	238	5895	222	154	6649	169	6438	3435	4183
37	1069	166	1037	172	258	1071	90	497	1219	145	1206	143	153	1067	510
"""


def part1() -> None:
    def line_diff(line: str) -> int:
        nums = [int(_) for _ in line.split()]
        return max(nums) - min(nums)

    result = sum(line_diff(_) for _ in INPUT.splitlines())
    print(f'the result for part 1 is {result}')


def part2() -> None:
    def line_diff(line: str) -> int:
        nums = [int(_) for _ in line.split()]
        for (a, b) in combinations(nums, 2):
            max_ = a if a > b else b
            min_ = a if a < b else b
            if max_ % min_ == 0:
                return max_ // min_
        return 0

    result = sum(line_diff(_) for _ in INPUT.splitlines())
    print(f'the result for part 2 is {result}')


def main() -> None:
    part2()
    print('done')


if __name__ == '__main__':
    main()

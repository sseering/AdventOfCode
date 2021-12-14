#!/usr/bin/env python3

from collections import namedtuple, Counter

INPUT = """CBNBOKHVBONCPPBBCKVH

FK -> O
BK -> B
PB -> N
VS -> P
OF -> H
KP -> K
PS -> K
OV -> N
FO -> H
KN -> P
HF -> K
BV -> N
OO -> B
KC -> V
CK -> H
BC -> P
VV -> S
NS -> C
SF -> O
BN -> V
NH -> N
VP -> F
KH -> S
BO -> N
VN -> K
BB -> H
CH -> H
HP -> O
KK -> O
CB -> S
VC -> P
FH -> B
SP -> C
NF -> O
HN -> N
PO -> P
PP -> C
SO -> F
FB -> B
SB -> B
SC -> B
HK -> O
BF -> V
OB -> B
NC -> V
HC -> F
KO -> C
NV -> C
HB -> H
FP -> S
OS -> O
HH -> K
OK -> B
OH -> C
NP -> V
SN -> H
SK -> B
HV -> F
VF -> P
CP -> H
FN -> H
FV -> B
CN -> H
OC -> O
KV -> P
CF -> B
OP -> B
FC -> O
PC -> B
CV -> S
PV -> H
VK -> N
SS -> C
HO -> F
VH -> C
NB -> S
NN -> F
FF -> K
CC -> H
SV -> H
CO -> K
BP -> O
SH -> H
KS -> K
FS -> F
PF -> S
BS -> H
VO -> H
NK -> F
PK -> B
KB -> K
CS -> C
VB -> V
BH -> O
KF -> N
HS -> H
PH -> K
ON -> H
PN -> K
NO -> S"""

TEST_INPUT = """NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C""";


InsertionRule = namedtuple('InsertionRule', ['a', 'b', 'r'])


def build_insertion_rule(insertion_str: str) -> InsertionRule:
    # parse something like
    # CH -> B
    split = insertion_str.split()
    return InsertionRule(a=split[0][0], b=split[0][1], r=split[2])


def part1(polymer_manual: str, iterations: int) -> int:
    lines = polymer_manual.splitlines()
    template = lines[0]
    insertion_rules = [build_insertion_rule(_) for _ in lines[2:]]

    def make_polymer(template):
        a = None
        b = None
        for b in template:
            if a is not None:
                yield a
            for rule in insertion_rules:
                if rule.a == a and rule.b == b:
                    yield rule.r
                    break
            a = b
        yield b

    for _ in range(iterations):
        template = make_polymer(template)

    ctr = Counter()
    for char in template:
        ctr[char] += 1

    most = None
    least = None
    for (_, v) in ctr.most_common():
        if most is None:
            most = v
        least = v

    if most is None or least is None:
        raise ValueError()

    return most - least


def main() -> None:
    print("test 1 good: {}".format(part1(TEST_INPUT, 10) == 1588))
    print("part 1: {}".format(part1(INPUT, 10)))
    print('done')


if __name__ == '__main__':
    main()

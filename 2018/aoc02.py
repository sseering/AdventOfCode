#!/usr/bin/env python3

# --- Day 2: Inventory Management System ---
#
# You stop falling through time, catch your breath, and check the screen on the device. "Destination reached. Current Year: 1518. Current Location: North Pole Utility Closet 83N10." You made it! Now, to find those anomalies.
#
# Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either. But now that so many people have chimneys, maybe he could sneak in that way?" Another voice responds, "Actually, we've been working on a new kind of suit that would let him fit through tight spaces like that. But, I heard that a few days ago, they lost the prototype fabric, the design plans, everything! Nobody on the team can even seem to remember important details of the project!"
#
# "Wouldn't they have had enough fabric to fill several boxes in the warehouse? They'd be stored together, so the box IDs should be similar. Too bad it would take forever to search the warehouse for two similar box IDs..." They walk too far away to hear any more.
#
# Late at night, you sneak to the warehouse - who knows what kinds of paradoxes you could cause if you were discovered - and use your fancy wrist device to quickly scan every box and produce a list of the likely candidates (your puzzle input).
#
# To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number that have an ID containing exactly two of any letter and then separately counting those with exactly three of any letter. You can multiply those two counts together to get a rudimentary checksum and compare it to what your device predicts.
#
# For example, if you see the following box IDs:
#
#     abcdef contains no letters that appear exactly two or three times.
#     bababc contains two a and three b, so it counts for both.
#     abbcde contains two b, but no letter appears exactly three times.
#     abcccd contains three c, but no letter appears exactly two times.
#     aabcdd contains two a and two d, but it only counts once.
#     abcdee contains two e.
#     ababab contains three a and three b, but it only counts once.
#
# Of these box IDs, four of them contain a letter which appears exactly twice, and three of them contain a letter which appears exactly three times. Multiplying these together produces a checksum of 4 * 3 = 12.
#
# What is the checksum for your list of box IDs?
#
# To begin, get your puzzle input.
#
# --- Part Two ---
#
# Confident that your list of box IDs is complete, you're ready to find the boxes full of prototype fabric.
#
# The boxes will have IDs which differ by exactly one character at the same position in both strings. For example, given the following box IDs:
#
# abcde
# fghij
# klmno
# pqrst
# fguij
# axcye
# wvxyz
#
# The IDs abcde and axcye are close, but they differ by two characters (the second and fourth). However, the IDs fghij and fguij differ by exactly one character, the third (h and u). Those must be the correct boxes.
#
# What letters are common between the two correct box IDs? (In the example above, this is found by removing the differing character from either ID, producing fgij.)
#
# Although it hasn't changed, you can still get your puzzle input.

import collections
import itertools
from typing import List

INPUT = [
    "omlvgpokxfnctqyersabjwzizp",
    "omlvtdhxxflctqyersabjwziup",
    "omlvgdakxfnctqyersabzmziup",
    "omlvgdhkxfnchqyersarjwsiup",
    "omlvgdnkxfnctqyersabhwziuq",
    "omvlgdhkxfnctqyersajjwziup",
    "fmlvgdbkxfnctqyersabjwzqup",
    "omlvcdhexfnctqyersibjwziup",
    "omlvgdhkxfnctqyersoyjnziup",
    "omdbgdhpxfnctqyersabjwziup",
    "omlvgdbkxfnctiyersabjwziwp",
    "omlogdhkxfncpqyersabjfziup",
    "omlvgdhkxfncxayersabwwziup",
    "omlvgdhkxfgctqyepsabjnziup",
    "omlvzdhkxfnctqyerxabjwoiup",
    "orlvtdhoxfnctqyersabjwziup",
    "omgvgdhkxfnctqyetsarjwziup",
    "omlvgdhkxunctcqersabjwziup",
    "omlvgdhkxfnctqyertakjwziun",
    "omlvhdhkxfhetqyersabjwziup",
    "omlvjdhkxfnctqyersabjtzirp",
    "omsvgdhkifnctqyeryabjwziup",
    "ohlvgdhkxfncteyersabtwziup",
    "omlvgdhkxjqctqyerkabjwziup",
    "omljgdhkxfncxqiersabjwziup",
    "omlvgdhkxvnctqyetscbjwziup",
    "omlvgdhxxfnctqykrsabjwziui",
    "omlbgdhkxfnetqyersabjwliup",
    "omlvgvhkxfnctqyerjabjwzwup",
    "wmlvgdhkxfnctqyyrsabjwziuc",
    "omlvgdhkufnctqxersabjwpiup",
    "omlvgdtkxfnctqyercvbjwziup",
    "omtvgdhkxfnctqygrsmbjwziup",
    "omlvgdbkxfnctqyersagjwzrup",
    "omlvgdpksfnctqyorsabjwziup",
    "omlvgdlkxfnctqyerhaajwziup",
    "omlvgdhkxfnctqyersabjwkiqh",
    "omlvgdykxfnctqdersdbjwziup",
    "omligdhklfnctpyersabjwziup",
    "omlvzdhkxfnctryersabjwziap",
    "nmlvgdqkxfnctqyemsabjwziup",
    "omlvgdhkxoncqqyersabjyziup",
    "otlvgdhkxfnctqyersabjwzzhp",
    "omlvgdhvxfnctqyirsabjwziue",
    "omlvgohkxfnctqjersabjwzeup",
    "omlngdhkxfnytqyersabjwsiup",
    "gmlvgbhkxfnctqyersabjwziyp",
    "nmlvgxhkxfnctqyxrsabjwziup",
    "omlvwdhkufnctqyerfabjwziup",
    "omlvqdhkxfnctqyersabfmziup",
    "omlvgdhkxfnctqlerscbjeziup",
    "omlvgdhkxfncxqyerjabjgziup",
    "omlvgdhkxwnctqyersvbjwriup",
    "ozlvgdhkxfnctqyersabjjziuj",
    "omlvguhkxfnctqyersabjwznut",
    "ozlvwdhkxfactqyersabjwziup",
    "oplvgdhkxfnctqyersakjwiiup",
    "omlkgbhkxfnctqyetsabjwziup",
    "oukvgdhkxfnctqyerslbjwziup",
    "omllgwhkxfnctqyersasjwziup",
    "omlvgdqkvfnctqyjrsabjwziup",
    "omlvguhkxfnctqyepsakjwziup",
    "oblvgdhkxfnctqyersibjwciup",
    "omlvgdhkxfpetqyersnbjwziup",
    "omlvgdhkxfnctqyersabgwpmup",
    "ohlvgdhkxfnctqyersgbjwdiup",
    "omlvgkhkxfnctqyarsabjwziuj",
    "omtvgdhkxfnctqoersabjwzfup",
    "omlvgdhkxfncbqyersafjwzkup",
    "amlvgdhkmfnctqyorsabjwziup",
    "omlvndhkxfnctbyersagjwziup",
    "oslvgdhkxfactqyersabjwziip",
    "omlvgdhkxfnrtqyerumbjwziup",
    "omjvgdhaxfnctqyersajjwziup",
    "omlvgdhkxfyctqyersabjvziuf",
    "omlvgdhkxfgctqyervabjwzuup",
    "oclvhdhkxfnctqyirsabjwziup",
    "omlvgdhkxfnctqyrrsbbjwsiup",
    "nmlvghhkxfnctqyersayjwziup",
    "omlvgdhksfnzcqyersabjwziup",
    "omlvgdhbxknctqyerzabjwziup",
    "omlvgdhsxflctqyercabjwziup",
    "omlvgdhkxfncthyersabjpzgup",
    "omlvgdhkxfnhtqyersadjwzilp",
    "omlvgdhyxfnctqyershjjwziup",
    "omlvhdhkxfnctqytesabjwziup",
    "omlvgbhkxfnctqyhrsabjwmiup",
    "omlvnyhkxfnctqyersabbwziup",
    "omlvgdhkxfnhzqcersabjwziup",
    "omljgdhkvynctqyersabjwziup",
    "omrvgdhkxfnctqysrsabjmziup",
    "omlvgdhgxenctqyerfabjwziup",
    "omlvgdokxfncvqyersasjwziup",
    "omlvguhkxfnctqyersabbbziup",
    "imkvgdhkxfnctqyqrsabjwziup",
    "omlvgdikxfnctwyersabbwziup",
    "oulvgdhuxfngtqyersabjwziup",
    "omlvgdhkxfdctqqbrsabjwziup",
    "omlvgdhbofnctqyersmbjwziup",
    "omlzgdhkxfnctzyecsabjwziup",
    "oflvgdhkxfnctqyerpabjwzcup",
    "ommvgdhkxfnctqyicsabjwziup",
    "omlvgdhkxfnctqyewsabjwzisd",
    "ojlvgdhfxfnctqyersabjwzihp",
    "smlvgdhkxfnctqyzrsabjwaiup",
    "ohlvgdhkxfnctqyersabnwziue",
    "jslvgdhkxfnctqdersabjwziup",
    "omlvgdhkdenctwyersabjwziup",
    "orljgdhkxbnctqyersabjwziup",
    "omlvgdhkxfnctaaersabjwzrup",
    "qmlvgdhknfncqqyersabjwziup",
    "omlvgdhkxfnctqyerssbjwncup",
    "omlvgdhkxynctqdercabjwziup",
    "omivgdhpxfnctqiersabjwziup",
    "omuvgdbkxfnctqyersajjwziup",
    "omlvbdokxfnctqyehsabjwziup",
    "gmlvgdhkxcnctqyemsabjwziup",
    "hmlvgdhkxfncsqyersabjwzidp",
    "omlvgdhkxftztqytrsabjwziup",
    "omlvgdhkxfnatqyeesabjbziup",
    "omlvodhkxfnctqbirsabjwziup",
    "omlvgdhsifnctqyersabjwziop",
    "oyvvgdhkxfnctqyersabjwzinp",
    "qmlvgdhkxfnctqyersdbawziup",
    "omlvguhkxfncuqyersabjwzipp",
    "omspgdhkxfnctqyersabjwzifp",
    "omlvgdhkxfnamqyeryabjwziup",
    "omlvgdhkngnctqyxrsabjwziup",
    "omdvcdhkxfnctqynrsabjwziup",
    "omyvgdhkxfnctqyeryabjyziup",
    "hmlvgdhkxfnctqyersabjwzwap",
    "ombvgdhkxfyctqyersabjwziuk",
    "omlvadhkxfnctqyersoqjwziup",
    "ollugdhkxfnctqyersabjwzizp",
    "omlvgdhkxfncvqmersabjwiiup",
    "omlvgdkkxfnupqyersabjwziup",
    "omlvgdhkxfncratersabjwziup",
    "oklvgdskxfnctqyersabjkziup",
    "omlvgdhkxfnctqyernebgwziup",
    "omsvgdhkxfnctqyersaejwziuv",
    "omlvgdhkxfrctlynrsabjwziup",
    "omlggdhkxfnctqyersbbjmziup",
    "omlvgdhfxfnctqyehrabjwziup",
    "omqvgdhkxfnctqcersabjwzfup",
    "omlvgdhklfncqxyersabjwziup",
    "omlvgxhkxfnctqyersabebziup",
    "omlfgdhkxfnctjyersabkwziup",
    "omlvgdhkxfnctqysrtabjwqiup",
    "omlvgdhkxfnltqaersabfwziup",
    "ofhvgdhkxfnctqyessabjwziup",
    "omlvpdekxfnctqyerscbjwziup",
    "omlvcdhkxlnbtqyersabjwziup",
    "omlvfdhkxfnctqyersabjwrnup",
    "omlvddhkxfncdqyersabjwziut",
    "omlvgdhkxfnctqxersabjhiiup",
    "omidgdhkxfnctqyeysabjwziup",
    "omlogdhkxfnptqyersabjwniup",
    "omlvgdhkxfnwthyersabjwziuz",
    "omevgdhkxgnctbyersabjwziup",
    "omlvgdhkxfnytqyersabjozuup",
    "omlvgvhkxfmctqyersabjwziuw",
    "oelvgdhkxfoctqyersadjwziup",
    "lmlvgdhkxfnctqeersabjwzisp",
    "omlvgdhkxfcctqyersasjwzibp",
    "gmlvgdhkyfnctqyersabjwziuz",
    "omlvgdhkxfnctslersabjwziuf",
    "omlvgehkxfnctqyeosabjwziyp",
    "otlggdhkxfjctqyersabjwziup",
    "bmjvgdhixfnctqyersabjwziup",
    "omlvgqhkxfnctqdezsabjwziup",
    "omlvgbhkxfnciqnersabjwziup",
    "omlvgdhlxfnctqydrsdbjwziup",
    "omlvgdhkxfncfqyersabjwxinp",
    "ymlvgdhkxfnctqyersabhwziui",
    "omdvgdhkxfnctqyersabjwxdup",
    "bmlvgdhkxfnwtwyersabjwziup",
    "dmlvgmhkxfnctqyxrsabjwziup",
    "omlvgdhkxbnntqyersabjiziup",
    "omlvgdhkmfnctlyersgbjwziup",
    "omlvgdhkxfnctqhersablwzixp",
    "ommvgdhkxfwctqyersabnwziup",
    "omlkgdhjxfnctqyersabjwjiup",
    "omlvgdhrxfnctqyeasabjnziup",
    "omvvgdhkxtnctqyersabjtziup",
    "omlvgdhkufgctqyersabfwziup",
    "omqvgwhkxfnctqyevsabjwziup",
    "oalvgdhkyfyctqyersabjwziup",
    "omlvgdhkxfnctqyefvabjwhiup",
    "jmlvgdakxfnctqyersabjwtiup",
    "gmlvgmhkxfnctqyersaqjwziup",
    "omlvgdhkxcnctqydrszbjwziup",
    "omlvgdhkxfnctxnersxbjwziup",
    "omlvgyhkxfnctqyersabjeaiup",
    "omlcgdhkxfncvqyersabjoziup",
    "omlvgdhkxfycttyercabjwziup",
    "omlmgdhkpsnctqyersabjwziup",
    "lmlvglhkxfnctqdersabjwziup",
    "omlvgdhxdfncoqyersabjwziup",
    "omlvgdhkxfnctqyersabjwkixv",
    "oplvgdhkxfnctiyersabjoziup",
    "omlvgdnkxfnctdyersebjwziup",
    "omlvguckxfnctqwersabjwziup",
    "omlvgdhojfnctqyersabjoziup",
    "opjvxdhkxfnctqyersabjwziup",
    "omevgdhkdflctqyersabjwziup",
    "omlvgilkxfncaqyersabjwziup",
    "omlvgdhkqfnctqyersabunziup",
    "dmlvgdhkxrnctqyerssbjwziup",
    "omlvgdzcxfnctqyersabjwniup",
    "omlvgdhkxfnctqyeraabpsziup",
    "omlvgdhkxfnctqlersabjtziul",
    "omlvgbhkxfnctqyeysabjwpiup",
    "omlvgdhvxfnmttyersabjwziup",
    "omlvgdhkxznctqyersabewziua",
    "oqlvgdhkxfnctqjersabjfziup",
    "omlvgdhkqfnctqyoysabjwziup",
    "omlvgdhkxfnctqylrzabbwziup",
    "oalvguhkxfnctqyersabawziup",
    "omlvgdokxfncvqyersasjlziup",
    "omlvgdhkcfnctqyersazjwzfup",
    "oslvgdhpxfnctqyhrsabjwziup",
    "omlvgdhkxfnotqcqrsabjwziup",
    "umlvgdhlxfnctqyersnbjwziup",
    "oxlvgdhkxfnktjyersabjwziup",
    "omlvgdhkxhncnqyersabjwzirp",
    "jmlvgdhkxfncfqyersabjwzqup",
    "omlvgdhkbfnutvyersabjwziup",
    "omhvgddkxfnctqyersabqwziup",
    "omlvgdukxfnbtqyersabjwzjup",
    "oylvndhkxfnctqversabjwziup",
    "omlvgdhkcfnctqyersamjwfiup",
    "omlvgdskxfnctqyerssbjgziup",
    "qmlvgdhkxfncxqyersabiwziup",
    "omlvghhkxfnctwyersaljwziup",
    "omlvgdhkpfnbtqyersnbjwziup",
    "omlvgthkxfnctnyersabjwziut",
    "omlvgdhkpfnctqyeisabjfziup",
    "omlvgdhrxrnctqyersabjwzigp",
    "omlvjdhkxfnctqyersabpwwiup",
    "omlvgdhkxfnctsyersabjwzixl",
    "amlvgdhktfnctqyersabfwziup",
    "oklvvdhkxfnctoyersabjwziup",
    "rmlvgdhkxfncwqyersabxwziup",
    "omlvgdhkxfnctqyersabiwzjfp",
    "omlvgehkxfnctqyersebjzziup",
    "omlvgdhkxfncaqyersabwwzoup",
    "omlvgdhkxfncjqyersanjwfiup",
    "omlvgdhkwfnctqyersqbjwziux",
    "omrvgdhjxfnctqyeksabjwziup",
    "omlvgdhkxfnctpyersaftwziup",
]


def part1() -> int:
    # INPUT = [
    #     'abcdef',
    #     'bababc',
    #     'abbcde',
    #     'abcccd',
    #     'aabcdd',
    #     'abcdee',
    #     'ababab',
    # ]

    twice = 0
    thrice = 0
    for i in INPUT:
        c = collections.Counter()
        for l in i:
            c[l] += 1
        twice_inc = False
        thrice_inc = False
        for (_, count) in c.items():
            if count == 2:
                twice_inc = True
            elif count == 3:
                thrice_inc = True
        twice += 1 if twice_inc else 0
        thrice += 1 if thrice_inc else 0
    return twice * thrice


def part2() -> List[str]:
    # INPUT = [
    #     'abcde',
    #     'fghij',
    #     'klmno',
    #     'pqrst',
    #     'fguij',
    #     'axcye',
    #     'wvxyz',
    # ]

    res = []

    for (a, b) in itertools.combinations(INPUT, 2):
        different = 0
        for (la, lb) in zip(a, b):
            different += 1 if la != lb else 0
            if different > 1:
                break
        if different == 1:
            res.append(''.join(la if la == lb else '' for (la, lb) in zip(a, b)))

    return res


def main() -> None:
    print("part 1: {0}".format(part1()))
    print("part 2: {0}".format(', '.join(part2())))
    print('done')


if __name__ == "__main__":
    main()

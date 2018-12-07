// --- Day 2: Inventory Management System ---
//
// You stop falling through time, catch your breath, and check the screen on the device. "Destination reached. Current Year: 1518. Current Location: North Pole Utility Closet 83N10." You made it! Now, to find those anomalies.
//
// Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either. But now that so many people have chimneys, maybe he could sneak in that way?" Another voice responds, "Actually, we've been working on a new kind of suit that would let him fit through tight spaces like that. But, I heard that a few days ago, they lost the prototype fabric, the design plans, everything! Nobody on the team can even seem to remember important details of the project!"
//
// "Wouldn't they have had enough fabric to fill several boxes in the warehouse? They'd be stored together, so the box IDs should be similar. Too bad it would take forever to search the warehouse for two similar box IDs..." They walk too far away to hear any more.
//
// Late at night, you sneak to the warehouse - who knows what kinds of paradoxes you could cause if you were discovered - and use your fancy wrist device to quickly scan every box and produce a list of the likely candidates (your puzzle input).
//
// To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number that have an ID containing exactly two of any letter and then separately counting those with exactly three of any letter. You can multiply those two counts together to get a rudimentary checksum and compare it to what your device predicts.
//
// For example, if you see the following box IDs:
//
//     abcdef contains no letters that appear exactly two or three times.
//     bababc contains two a and three b, so it counts for both.
//     abbcde contains two b, but no letter appears exactly three times.
//     abcccd contains three c, but no letter appears exactly two times.
//     aabcdd contains two a and two d, but it only counts once.
//     abcdee contains two e.
//     ababab contains three a and three b, but it only counts once.
//
// Of these box IDs, four of them contain a letter which appears exactly twice, and three of them contain a letter which appears exactly three times. Multiplying these together produces a checksum of 4 * 3 = 12.
//
// What is the checksum for your list of box IDs?
//
// To begin, get your puzzle input.
//
// --- Part Two ---
//
// Confident that your list of box IDs is complete, you're ready to find the boxes full of prototype fabric.
//
// The boxes will have IDs which differ by exactly one character at the same position in both strings. For example, given the following box IDs:
//
// abcde
// fghij
// klmno
// pqrst
// fguij
// axcye
// wvxyz
//
// The IDs abcde and axcye are close, but they differ by two characters (the second and fourth). However, the IDs fghij and fguij differ by exactly one character, the third (h and u). Those must be the correct boxes.
//
// What letters are common between the two correct box IDs? (In the example above, this is found by removing the differing character from either ID, producing fgij.)
//
// Although it hasn't changed, you can still get your puzzle input.

extern crate itertools;

use std::collections::HashMap;
use itertools::Itertools;

const INPUT: [&'static str; 250] = [
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
];

fn part1() -> i32 {
    // const INPUT: [&'static str; 7] = [
    //     "abcdef",
    //     "bababc",
    //     "abbcde",
    //     "abcccd",
    //     "aabcdd",
    //     "abcdee",
    //     "ababab",
    // ];
    
    let mut doubles = 0;
    let mut triples = 0;
    for line in INPUT.iter() {
        let mut freqs = HashMap::new();
        for letter in line.chars() {
            *(freqs.entry(letter).or_insert(0)) += 1;
        }
        let mut got_double = false;
        let mut got_triple = false;
        for (_, freq) in &freqs {
            if *freq == 2 {
                got_double = true;
            }
            if *freq == 3 {
                got_triple = true;
            }
        }
        if got_double {
            doubles += 1;
        }
        if got_triple {
            triples += 1;
        }
    }
    return doubles * triples;
}

// fn combinations2<T>(i: T) -> i32
// 
// where T: Sized, T::Item: Clone{
//     return 0;
// }

fn part2() -> Vec<String> {
    // const INPUT: [&'static str; 7] = [
    //     "abcde",
    //     "fghij",
    //     "klmno",
    //     "pqrst",
    //     "fguij",
    //     "axcye",
    //     "wvxyz",
    // ];

    let mut res = vec![];

    for pair in INPUT.iter().combinations(2) {
        let a = pair[0];
        let b = pair[1];
        let mut num_diff = 0;
        let mut diff_idx = 0;

        for (idx, (la, lb)) in a.chars().zip(b.chars()).enumerate() {
            if la != lb {
                num_diff += 1;
                diff_idx = idx;
            }
        }

        if num_diff == 1 {
            let s = a.chars().enumerate().filter_map(|(i, c)| if i != diff_idx { Some(c) } else  { None }).collect::<String>();
            res.push(s);
        }
    }
    return res;
}

fn main() {
    println!("part 1: {0}", part1());
    println!("part 2: {0:?}", part2());
    println!("done");
}



// --- Day 1: The Tyranny of the Rocket Equation ---
// Santa has become stranded at the edge of the Solar System while delivering presents to other planets! To accurately calculate his position in space, safely align his warp drive, and return to Earth in time to save Christmas, he needs you to bring him measurements from fifty stars.
// 
// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
// 
// The Elves quickly load you into a spacecraft and prepare to launch.
// 
// At the first Go / No Go poll, every Elf is Go until the Fuel Counter-Upper. They haven't determined the amount of fuel required yet.
// 
// Fuel required to launch a given module is based on its mass. Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
// 
// For example:
// 
// For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
// For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
// For a mass of 1969, the fuel required is 654.
// For a mass of 100756, the fuel required is 33583.
// The Fuel Counter-Upper needs to know the total fuel requirement. To find it, individually calculate the fuel needed for the mass of each module (your puzzle input), then add together all the fuel values.
// 
// What is the sum of the fuel requirements for all of the modules on your spacecraft?
// 
// To begin, get your puzzle input.
//
// Your puzzle answer was 3266053.
// 
// The first half of this puzzle is complete! It provides one gold star: *
// 
// --- Part Two ---
// During the second Go / No Go poll, the Elf in charge of the Rocket Equation Double-Checker stops the launch sequence. Apparently, you forgot to include additional fuel for the fuel you just added.
// 
// Fuel itself requires fuel just like a module - take its mass, divide by three, round down, and subtract 2. However, that fuel also requires fuel, and that fuel requires fuel, and so on. Any mass that would require negative fuel should instead be treated as if it requires zero fuel; the remaining mass, if any, is instead handled by wishing really hard, which has no mass and is outside the scope of this calculation.
// 
// So, for each module mass, calculate its fuel and add it to the total. Then, treat the fuel amount you just calculated as the input mass and repeat the process, continuing until a fuel requirement is zero or negative. For example:
// 
// A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0, which would call for a negative fuel), so the total fuel required is still just 2.
// At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel. So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
// The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
// What is the sum of the fuel requirements for all of the modules on your spacecraft when also taking into account the mass of the added fuel? (Calculate the fuel requirements for each module separately, then add them all up at the end.)
// 
// Although it hasn't changed, you can still get your puzzle input.

const INPUT_P1_TEST: [& str; 4] = [
    "12",
    "14",
    "1969",
    "100756",
];

const INPUT_P2_TEST: [& str; 3] = [
    "14",
    "1969",
    "100756",
];

const INPUT: [& str; 100] = [
    "66910",
    "78957",
    "58510",
    "142350",
    "105820",
    "87317",
    "100743",
    "51390",
    "92804",
    "80752",
    "70169",
    "111892",
    "104715",
    "143166",
    "126158",
    "78712",
    "139223",
    "133863",
    "85912",
    "53883",
    "64812",
    "102254",
    "52482",
    "91855",
    "117520",
    "140253",
    "76706",
    "106693",
    "57948",
    "57578",
    "115640",
    "131273",
    "115373",
    "145219",
    "100889",
    "106447",
    "72347",
    "120250",
    "56898",
    "146689",
    "138246",
    "85068",
    "55292",
    "124814",
    "136750",
    "51820",
    "70396",
    "92806",
    "86093",
    "70467",
    "122356",
    "148530",
    "85569",
    "100492",
    "87062",
    "123225",
    "73872",
    "102104",
    "91194",
    "95077",
    "88352",
    "114906",
    "141056",
    "87220",
    "106517",
    "88867",
    "95883",
    "130158",
    "76702",
    "134241",
    "50561",
    "119258",
    "61669",
    "140396",
    "145201",
    "76914",
    "102281",
    "56618",
    "145968",
    "99542",
    "116789",
    "135633",
    "114646",
    "84253",
    "50650",
    "69127",
    "95446",
    "55357",
    "81180",
    "126940",
    "133743",
    "52261",
    "117429",
    "82291",
    "110373",
    "67626",
    "58014",
    "125342",
    "129508",
    "96332",
];


fn part1(input: &[& str]) -> u32 {
    return input.iter().map(|s| -> u32 { s.parse().unwrap() }).map(|i| -> u32 { i / 3 - 2}).sum();
    // return input.iter().map(|s| -> u32 { s.parse().unwrap() }).map(|i| -> u32 { i / 3 - 2}).fold(0, |run, elem| run + elem);
}

fn part2(input: &[& str]) -> u32 {
    fn cost(m: u32) -> u32 {
        let fuel = m / 3;
        if fuel <= 2 {
            return 0;
        }
        let fuel = fuel - 2;
        // println!("got fuel {}", fuel);
        return fuel + cost(fuel);
    }
    return input.iter().map(|s| -> u32 { s.parse().unwrap() }).map(cost).sum();
}

fn main() {
    println!("part 1 selftest good: {}", part1(&INPUT_P1_TEST) == (2 + 2 + 654 + 33583));
    println!("part 1: {}", part1(&INPUT));
    println!("part 2 selftest good: {}", part2(&INPUT_P2_TEST) == (2 + 966 + 50346));
    println!("part 2: {}", part2(&INPUT));
}


// https://old.reddit.com/r/adventofcode/comments/e4axxe/2019_day_1_solutions/f98ekeo/
//
// Two closed form implementations of part2 that I dont understand.
// Written in Python.
// Taken from them link above.
//
//
// def to_b3(n: int) -> List[int]:
//     if n < 0:
//         return ValueError()
//     if n == 0:
//         return [0]
// 
//     res = []
// 
//     while n > 0:
//         (n, trigit) = divmod(n, 3)
//         res.insert(0, trigit)
// 
//     return res
// 
// 
// def betaveros(m: int) -> int:
//     trigits = to_b3(m + 3)
//     s = sum(trigits)
//     f = trigits[0]
//     d = len(trigits)
//     return m/2 - s/2 - f - 3*d + 15/2
// 
// 
// def sim642(m: int) -> int:
//     mp3 = m + 3
//     max_ = math.floor(math.log(mp3 / 4, 3)) + 1
// 
//     return sum(max(0, math.floor(mp3 / (3**i)) - 3) for i in range(1, max_))
// 
// 
// def main() -> None:
//     print(betaveros(14))
//     print(sim642(14))
//     print(betaveros(1969))
//     print(sim642(1969))
//     print(betaveros(100756))
//     print(sim642(100756))
// 
//     print('done')

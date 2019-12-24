// --- Day 6: Universal Orbit Map ---
//
// You've landed at the Universal Orbit Map facility on Mercury. Because navigation in space often involves transferring between orbits, the orbit maps here are useful for finding efficient routes between, for example, you and Santa. You download a map of the local orbits (your puzzle input).
//
// Except for the universal Center of Mass (COM), every object in space is in orbit around exactly one other object. An orbit looks roughly like this:
//
//                   \
//                    \
//                     |
//                     |
// AAA--> o            o <--BBB
//                     |
//                     |
//                    /
//                   /
//
// In this diagram, the object BBB is in orbit around AAA. The path that BBB takes around AAA (drawn with lines) is only partly shown. In the map data, this orbital relationship is written AAA)BBB, which means "BBB is in orbit around AAA".
//
// Before you use your map data to plot a course, you need to make sure it wasn't corrupted during the download. To verify maps, the Universal Orbit Map facility uses orbit count checksums - the total number of direct orbits (like the one shown above) and indirect orbits.
//
// Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain can be any number of objects long: if A orbits B, B orbits C, and C orbits D, then A indirectly orbits D.
//
// For example, suppose you have the following map:
//
// COM)B
// B)C
// C)D
// D)E
// E)F
// B)G
// G)H
// D)I
// E)J
// J)K
// K)L
//
// Visually, the above map of orbits looks like this:
//
//         G - H       J - K - L
//        /           /
// COM - B - C - D - E - F
//                \
//                 I
//
// In this visual representation, when two objects are connected by a line, the one on the right directly orbits the one on the left.
//
// Here, we can count the total number of orbits as follows:
//
//     D directly orbits C and indirectly orbits B and COM, a total of 3 orbits.
//     L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a total of 7 orbits.
//     COM orbits nothing.
//
// The total number of direct and indirect orbits in this example is 42.
//
// What is the total number of direct and indirect orbits in your map data?
//
// To play, please identify yourself via one of these services:
//
// The first half of this puzzle is complete! It provides one gold star: *
// --- Part Two ---
//
// Now, you just need to figure out how many orbital transfers you (YOU) need to take to get to Santa (SAN).
//
// You start at the object YOU are orbiting; your destination is the object SAN is orbiting. An orbital transfer lets you move from any object to an object orbiting or orbited by that object.
//
// For example, suppose you have the following map:
//
// COM)B
// B)C
// C)D
// D)E
// E)F
// B)G
// G)H
// D)I
// E)J
// J)K
// K)L
// K)YOU
// I)SAN
//
// Visually, the above map of orbits looks like this:
//
//                           YOU
//                          /
//         G - H       J - K - L
//        /           /
// COM - B - C - D - E - F
//                \
//                 I - SAN
//
// In this example, YOU are in orbit around K, and SAN is in orbit around I. To move from K to I, a minimum of 4 orbital transfers are required:
//
//     K to J
//     J to E
//     E to D
//     D to I
//
// Afterward, the map of orbits looks like this:
//
//         G - H       J - K - L
//        /           /
// COM - B - C - D - E - F
//                \
//                 I - SAN
//                  \
//                   YOU
//
// What is the minimum number of orbital transfers required to move from the object YOU are orbiting to the object SAN is orbiting? (Between the objects they are orbiting - not between YOU and SAN.)
//
// Although it hasn't changed, you can still get your puzzle input.

#[allow(unused)]
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[allow(unused)]
const TEST_INPUT: [&str; 11] = [
    "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
];

#[allow(unused)]
const INPUT: [&str; 1057] = [
    "97W)B43", "R63)RTM", "19C)SHD", "F85)8Z8", "5D9)Z5T", "RG5)R48", "HJC)NVP", "46W)1SL",
    "BFP)34P", "2M5)NWQ", "CJD)M9C", "8BK)QK5", "TWT)53F", "KT1)7FJ", "WG8)TBN", "FMZ)2RY",
    "734)H1Y", "8XM)GSZ", "TXP)Z9D", "NLX)TNW", "7ST)N94", "W68)1WS", "S1J)TDP", "5C2)8C4",
    "ZW4)GLV", "MHN)CVN", "4FH)NRP", "4SC)155", "6D9)PG7", "L44)CZ6", "KC5)PX6", "PFD)MGG",
    "5JJ)GBH", "63X)QYN", "N96)VM4", "WGN)JMD", "Y9P)RS9", "N6C)3HD", "4KF)1JP", "M31)3RJ",
    "W71)9C5", "FLV)M31", "J6Q)HSP", "ZPF)27T", "YXX)MJZ", "Q43)F38", "FPM)599", "TJX)M6S",
    "N94)5L3", "6LN)T2D", "9QK)S49", "N81)PD5", "VG4)264", "MS6)HYV", "JBP)HCJ", "SV7)C2F",
    "LZT)NCW", "7FJ)SV7", "TYJ)S5S", "YJY)2WW", "KKL)JDY", "QBL)5VX", "R7W)C7X", "1KP)JLP",
    "NKQ)9C6", "BVR)LWN", "3ZW)F31", "Q86)188", "V5D)CX3", "7L7)RKS", "6HC)LCZ", "T2F)8HT",
    "KFX)H6G", "D1Z)NXM", "5NH)PFD", "FRR)TWD", "GVY)KJW", "GBX)LVT", "KGZ)CNH", "GPZ)7J8",
    "4QY)6LN", "9VD)4QY", "4LY)SH6", "P9Y)RJW", "WK5)1HQ", "SPS)667", "VDN)1VF", "HCJ)ZRW",
    "SLV)C4Y", "MS6)76G", "LPD)Y9P", "CZ6)2C1", "VGT)J3F", "YS4)FMZ", "SNR)KFL", "Z4P)4FJ",
    "WQW)WBX", "13Z)NXK", "3Z2)VX9", "XZJ)KDN", "BH9)HNB", "7W1)F78", "B3H)H2Q", "ZN3)G75",
    "3VY)NN9", "2W3)9F7", "X8L)NKQ", "1J7)41S", "DN3)NV7", "NY1)KSJ", "HFG)NXQ", "27T)QHD",
    "VX9)4WC", "TTJ)MQT", "NVH)JJ7", "9L5)D3H", "VJM)MJD", "XZ3)N81", "83R)ZSZ", "9S5)FYR",
    "CMZ)W1P", "2L2)XRP", "XP9)JSF", "JM9)H51", "3GG)3M8", "6D9)L44", "5G2)2RN", "1KK)ZS3",
    "68S)JMG", "TDP)3W9", "XXY)LMB", "8GD)JVN", "H9L)7N1", "7J8)KQJ", "HNB)RN8", "SGV)DS4",
    "B2P)2M5", "HSP)XXY", "4JJ)SXX", "1ZZ)HYW", "MVQ)F27", "RMW)P9Y", "HLR)HF7", "6MS)X73",
    "5Z5)VQY", "XS9)VDN", "3VT)38P", "LPB)BQH", "1RT)XS9", "9F7)K3K", "G75)3H5", "6CD)B2F",
    "BTY)2PK", "2BH)1WY", "Y8Y)1QJ", "B9G)3LD", "172)65H", "8HC)YDG", "PX6)41H", "X4P)FH4",
    "JSF)6SW", "ZBY)XGM", "2M1)SYH", "W6B)3VY", "MGG)48J", "R41)C9C", "B43)V78", "M84)9S5",
    "PTF)23B", "ZJZ)33X", "ZBY)YHL", "56M)21K", "7F8)3R6", "GY6)QBL", "CM3)P9B", "LWN)JZ8",
    "NWQ)NK4", "9MB)M8X", "D12)TWT", "8WF)KKL", "RB9)4C4", "23B)93Z", "ZQZ)FSP", "QMW)DN3",
    "HRL)GDF", "1PN)795", "B7L)DYL", "CF8)68K", "FYR)JP2", "X4D)DB5", "NM2)39M", "FKQ)G4J",
    "Y9D)91R", "SHD)56S", "C5X)9KN", "24S)4JJ", "R48)KG1", "8NS)QD3", "72X)WD9", "WWH)Q86",
    "1MF)99H", "RMB)QY1", "BNP)Y6V", "FJS)13R", "KXS)QH8", "GM7)YC5", "VMM)B3T", "M8X)3GG",
    "815)H7Q", "KSJ)CM3", "VLP)T8B", "JDY)X4P", "PQH)CNL", "1QX)FTN", "CTV)VB6", "HYW)LGL",
    "HLC)7VN", "L54)YJ8", "NKP)ZJZ", "4GQ)1CL", "96B)PDN", "S37)7L4", "2RY)BKW", "D5P)XF3",
    "MF3)S26", "8DN)NNN", "7L4)5H7", "CD4)H4M", "H5D)JM7", "C7D)T2F", "SMF)W61", "J7P)F22",
    "41H)8CF", "DQR)C8H", "Q3D)417", "GRX)MVQ", "5GN)BZB", "9SY)K2Y", "X57)XH6", "JQX)B8Q",
    "78L)85W", "PDN)13Z", "21K)SRY", "D5V)ZYC", "VXF)PF3", "815)184", "76G)X44", "FDW)B9G",
    "KJW)7JQ", "667)YQW", "F31)QLK", "JNK)W7V", "46W)HCB", "56S)F3Y", "93Z)1T7", "RBQ)KRY",
    "894)XXW", "33X)XG5", "591)KCF", "X44)JQX", "J91)7L7", "DDZ)MKC", "7W2)5TX", "M6S)7J9",
    "H7G)CK7", "145)2VG", "FJS)MCC", "PJT)NK1", "7BY)MC3", "GBH)33J", "F4Y)FVB", "JFX)K8R",
    "5VX)815", "VWG)PLJ", "JHH)QJ4", "4G6)R2K", "KL7)J5Y", "XXM)5NH", "H5L)Y4Z", "85W)8XV",
    "YFK)PTV", "4R3)8P5", "TVW)MZK", "V6C)KKX", "P23)DPN", "8V4)SC9", "SH6)JCF", "PTF)1DP",
    "JCF)XHT", "1VR)ZKZ", "FFW)RPW", "QN1)MBD", "F7K)V92", "VM4)ZD2", "FTN)2MC", "L5V)ZR9",
    "COM)1MF", "XZ3)6H3", "KGP)4FH", "M77)R62", "2R2)GTL", "SYH)JXP", "98K)F4P", "6K3)GNT",
    "BWB)Q3D", "VV8)YOU", "MC3)SF5", "T1V)1CQ", "37V)TJX", "CNL)PJT", "9YW)FX6", "862)NZW",
    "J7B)PVT", "WXN)X57", "SXX)6DP", "ZKZ)BWB", "XNW)9XX", "ZQZ)7W1", "F27)NZV", "C7X)5D9",
    "64K)JNP", "5H7)HXC", "XDJ)GXK", "33J)TBK", "1JP)7WX", "LHQ)TQK", "YJC)2LJ", "4YL)72X",
    "GXK)BTF", "1CL)X87", "VF9)KC5", "8CF)KQ7", "2P8)3VT", "Z11)D1R", "2PK)1YT", "DLM)T93",
    "2VG)NM2", "R96)CC3", "YHL)J7B", "K41)ZN3", "G6N)L3V", "57Y)NLJ", "MG8)P8P", "2C5)5GN",
    "3G3)SQ3", "WBX)XT7", "SRY)R63", "79R)VG9", "BTF)N1J", "LYR)W68", "6QN)4TH", "YRT)BS8",
    "MM8)Y37", "BCW)BDZ", "MCC)CH2", "44Y)RMB", "PF3)V7K", "ZR9)HSN", "YRZ)Z19", "5TG)SL6",
    "NDL)PQH", "63K)73H", "V3C)KXS", "JLM)14R", "4TH)4KF", "J76)5N1", "WJN)NKP", "TV6)19C",
    "Z8F)4S7", "F6W)1VR", "417)QFZ", "67J)GSQ", "TYV)GDL", "JMD)4BY", "7PX)FYX", "K8R)W9L",
    "LGL)QMW", "KPX)V7D", "53F)FJS", "3GH)61Z", "QYD)JNK", "VSP)NWK", "JNP)2M1", "VLH)FDW",
    "KVT)85X", "NSK)FFW", "D2J)NVK", "B54)Z82", "5MP)1R5", "J38)SMF", "2QG)3GY", "37V)CDG",
    "XPN)6XR", "HCB)MPY", "ZX9)WK5", "5WF)SRB", "DLS)G7D", "FGS)5MP", "1T7)X6X", "KG1)9LV",
    "QTB)F37", "9LV)JH9", "3RJ)8DJ", "FRB)MM8", "KKX)LZT", "HH9)RF7", "D1R)89F", "QJ4)1L6",
    "DT9)KGP", "N4S)P1P", "F52)S37", "XH6)Y27", "TXV)YFJ", "ND8)ZX9", "43N)F4Y", "NNY)S1J",
    "PG7)BNP", "RPW)WWH", "4S3)VJB", "Z82)JGP", "BQY)64L", "SQ3)91C", "D3V)XGG", "3Z2)FPM",
    "TW8)Y8L", "HSN)S4N", "H6G)DMP", "SHS)SWG", "5N7)6QN", "M8X)WNH", "6V1)PPM", "1R5)MF3",
    "H9B)CFF", "GSZ)WGN", "LZQ)GPZ", "RP6)2C5", "X87)D3V", "PTV)P9X", "G1T)YRZ", "2NW)C42",
    "BQH)JXN", "CFJ)2L2", "MVW)LQQ", "SRB)JM9", "YXL)HD2", "7J9)6V1", "ZPF)B3H", "GSJ)NBY",
    "C8H)Y8Y", "1BS)H5L", "XGG)6MS", "KZF)734", "DD1)8G9", "QHL)95X", "3LD)9H1", "D8Q)1KK",
    "XWH)Y4M", "P8P)2BH", "188)C2G", "8XV)CFJ", "V7K)H7G", "4G5)W2M", "TPX)9WY", "MJZ)V2W",
    "YC5)F1T", "W61)H25", "9P3)8R8", "1XZ)642", "DXD)TXP", "MJZ)SLV", "CN8)NKZ", "LYS)ZNL",
    "MQT)J38", "LKZ)TSQ", "HNQ)9XZ", "226)84R", "8HT)591", "4BY)6X1", "VLH)XTX", "DPD)RG5",
    "QLK)HZL", "92P)CTV", "GMF)78T", "2RY)RKV", "9NQ)HRL", "YDG)YX4", "5X3)V7S", "8G9)QP9",
    "GDF)6HC", "5TX)FD4", "4QT)MSV", "13R)NPK", "GNT)ZBY", "4S7)MCY", "N4S)DK9", "VYW)8PD",
    "ZSN)24S", "JGP)J91", "YS8)YFK", "XXW)QN1", "8R7)92P", "FD4)T1V", "XHT)PZB", "DYV)VSP",
    "JJ7)N8D", "WPM)NLR", "QHD)GBK", "5R9)1V4", "QYN)PXJ", "TZM)XXM", "172)WY2", "NB5)GGC",
    "ZRW)9VV", "F5K)WR3", "FSS)P4Z", "T2D)4G5", "1X9)YRT", "C7V)F46", "NLJ)HNQ", "3DV)1GD",
    "1NB)G1T", "1ML)HH9", "YZK)JBP", "TDP)8BK", "R62)H9L", "NWK)98Y", "J55)7ST", "SF5)BFP",
    "MQD)D5L", "XT7)P1Y", "JHB)MG9", "SSM)8K8", "NXQ)W5P", "MBT)B3D", "P1P)YJY", "C9C)BCW",
    "GQ6)KFX", "QP9)HM4", "PG7)XNW", "XXW)ZHS", "V84)LPD", "XG5)9CV", "QRL)V5D", "8R8)DT9",
    "FBK)G25", "1SK)37V", "L8Q)38M", "GCX)QW1", "JP9)ZW4", "W15)GH9", "9VV)WWN", "MDP)97C",
    "NCW)JFR", "GBD)9PM", "Q77)7PX", "ZB1)XPN", "88Y)B7L", "N96)862", "TBK)5R9", "R2K)T1W",
    "DPN)BS7", "96X)4YL", "Z8F)4R3", "S4N)TZM", "PLJ)78L", "GGC)T65", "BZ2)H5D", "YJY)L8Q",
    "34P)1ML", "HXC)LZQ", "GXF)SFV", "FYR)6B4", "T93)FFK", "39M)9NQ", "1V4)NTH", "6PG)W15",
    "TV6)C2K", "MYH)HYC", "1PN)VLP", "62K)JNV", "PG8)ZL9", "RHY)DPD", "N1J)MQN", "C36)894",
    "264)ZB1", "CH2)FPT", "R7R)QG8", "4WC)VG4", "22N)Y5Y", "NSQ)8V4", "NTH)M77", "L6J)GBD",
    "QML)4QT", "F82)V6C", "Y55)2J3", "FYX)P23", "LJW)5QL", "3XW)GBW", "JRN)RB9", "MQN)563",
    "48J)293", "3CP)JHB", "VV7)5TG", "1GD)8LL", "D8H)7BY", "61Z)4GX", "F78)KL7", "L3V)NSQ",
    "TKH)2QG", "F85)YS4", "XPN)43N", "4RF)W7C", "YFJ)226", "92P)SAN", "N9J)G6N", "HF7)9LR",
    "H68)6D9", "6H3)HLC", "887)1PN", "2C1)XT3", "CS2)SSM", "CNH)4GQ", "QD3)635", "G7D)172",
    "NLR)VV7", "3CZ)N6C", "GTL)J6Q", "D3H)D99", "19C)VGT", "T75)FGS", "MGY)96X", "J5Y)BKR",
    "3RJ)VDX", "RKV)376", "FFK)XP9", "F96)B2P", "G4J)FVT", "QFZ)1JH", "3L3)RHY", "3PN)2G8",
    "YJ8)5N7", "14R)X7P", "H1T)SHG", "1XB)RMZ", "ZVB)H68", "ZHS)CJD", "DW1)9MB", "795)5P9",
    "84R)2R2", "KDN)QTB", "38P)JP9", "2WW)2FM", "C4Y)67J", "5V3)4M3", "LMB)6N4", "DYL)ND8",
    "98Y)BQY", "ZYC)JRN", "1FX)R41", "9CV)PGG", "S5S)68S", "PMB)1G6", "ZKX)9S2", "Z18)1XB",
    "JH9)ZL5", "12K)WV2", "FVB)9VD", "5PN)XWM", "JP2)4RF", "XRP)3Z2", "F96)YS8", "B3D)5C2",
    "M6S)BH9", "F86)145", "WRP)C5X", "MBD)5PN", "WV9)RMW", "T9S)VXF", "JVN)WG8", "SC9)VYW",
    "91C)BM1", "Y3Y)RPN", "91R)X3J", "NN3)CMZ", "HYC)LPB", "WH4)WRP", "6B4)1SK", "BT7)HJ8",
    "MBT)4LY", "LQQ)8DN", "BDZ)F9M", "SYH)MGY", "XF3)JFJ", "1BV)79R", "J6Q)QBZ", "3HD)TYJ",
    "7WX)P34", "226)MBT", "1HQ)DLS", "SSM)NY1", "PZB)8NS", "NKZ)YJC", "W7V)Y3Y", "Y6V)TVW",
    "QBZ)QRL", "8DJ)GCX", "FZC)3GH", "4R1)ZKX", "WNH)B54", "FYJ)L5V", "887)F96", "7TV)JDD",
    "W68)17B", "VDX)FBK", "BW8)MQD", "54Y)3L3", "GLV)1J7", "353)MYG", "D2J)T9S", "8K8)2P8",
    "3H5)B6N", "1YT)RBQ", "V78)1RT", "YQW)SHJ", "RJW)VF9", "GRH)TYV", "2FM)TW8", "X3J)LHQ",
    "B8Q)D6G", "293)3CP", "3GY)R7R", "PBS)MHN", "1JH)WH4", "XTX)4J9", "B3J)8VM", "KSJ)3XW",
    "1LR)F55", "3W9)4YX", "Y6V)LYR", "7MF)1XZ", "F46)BVG", "635)DQR", "P4Z)5HT", "MCL)1BV",
    "JXN)G22", "6CD)1BS", "JMG)9SY", "CDG)7W2", "VB6)DLM", "JRN)7TN", "599)BMS", "XT6)MVW",
    "9WY)9YW", "N94)42C", "R9Z)NNY", "YQW)HFN", "MJD)Y55", "2M1)VD6", "TBN)2JQ", "68G)YQN",
    "C2F)HQG", "ZNL)GXF", "7TN)DPT", "557)9P3", "S49)FKQ", "L6Y)VJM", "C2S)MCL", "SH1)MG8",
    "G25)JLM", "TSQ)HLR", "FHC)D2J", "RPN)F85", "SFV)XT6", "NZV)6PG", "QH8)JFX", "NNN)GRH",
    "642)KT1", "N8D)8Y9", "S66)F82", "V92)YTT", "B3T)T75", "MVW)YHZ", "F1T)NDL", "PVT)1ZZ",
    "56F)NN3", "JFJ)7MF", "S2K)LJW", "P9Y)98K", "F3Y)5X3", "1G6)N9J", "9XX)SNR", "XDD)YXX",
    "68K)MYH", "3R6)63X", "L3J)KBZ", "MFG)XHN", "2G8)DDZ", "GH9)4N5", "BKW)B27", "KDD)1FX",
    "6QN)PG8", "K3K)46W", "HM4)1Y5", "1BS)XZ3", "RG5)WXT", "G8F)HYT", "M9C)XDJ", "Y8L)WJN",
    "MKC)9L5", "1VF)F6W", "RXR)97W", "ZCW)9ZR", "RZF)G9J", "JMD)1KP", "X73)NSK", "FX6)MXL",
    "ZL5)3DV", "RN8)LYS", "RF7)56F", "376)GVY", "8C4)K41", "H51)5Z5", "4M3)64K", "JGP)J55",
    "5QL)L54", "7N1)FYJ", "XWN)MWH", "Q3D)4R1", "3M8)54Y", "P9B)5KD", "44Y)95Q", "ZL9)XWH",
    "SGK)4G6", "3NJ)1QX", "Y5Y)557", "TWD)WV9", "KZF)Q1Q", "QY1)16J", "NK1)C2S", "HQG)VMM",
    "836)2W3", "345)44Y", "97C)SSB", "X8L)JG9", "TQK)F5K", "8DN)9FD", "5LT)Z11", "5P9)6K3",
    "WY2)56M", "P23)CD4", "YJC)MS6", "5KD)57Y", "7J9)WPM", "1DP)8V1", "JNV)NLX", "M5P)BW8",
    "YHZ)G8F", "VJB)X8L", "6X1)L3J", "W2M)FN4", "C2K)JHH", "JLP)C7D", "GBW)887", "184)H1T",
    "LVT)LGV", "SHG)TV6", "9XG)FHC", "BRC)CF8", "KFL)353", "NZW)FRB", "FN2)5WF", "KBZ)LKZ",
    "5N1)3ZW", "8PD)22N", "Q1Q)CN8", "2F4)FN2", "B2F)PMB", "1SL)1X9", "Y4M)Z4P", "YQN)T8N",
    "JV3)4S3", "9C5)DD1", "B84)5V3", "2RN)3L6", "XTF)KCX", "Y4Z)TTJ", "T8B)W6B", "YX4)8XM",
    "JFR)5JJ", "HQ4)JVD", "9PM)2NW", "9C6)QHL", "P1Y)12K", "BZB)VV8", "1WY)62K", "DPT)VLH",
    "NPK)5LT", "RPN)7F8", "9S2)V84", "WR3)BZ2", "89F)X4D", "5HT)V6N", "HYV)PBS", "T65)88Y",
    "F22)D5P", "HZL)XWN", "JDD)M84", "8P5)Q43", "5G3)3G3", "B6N)RXR", "WXT)BVR", "2JQ)68G",
    "95X)SHS", "JBP)B55", "4YX)D5V", "KCF)Y7L", "ZWF)126", "F37)D8Q", "JZ8)6CD", "ZDK)D12",
    "2MC)GY6", "6XR)QSY", "WV2)FRR", "G22)SGK", "KCX)SH1", "155)C7V", "183)HFG", "K41)BT7",
    "BH9)ZCW", "1Y5)S66", "1L6)RZF", "P34)DW1", "Y9Q)B3J", "42C)7FR", "VQY)3PN", "NWK)J76",
    "D99)JV3", "914)ZVB", "NN9)YXL", "GRH)WXN", "DK9)D4H", "DS4)Y4H", "D5L)KZF", "C2G)ZSN",
    "7VN)F7X", "9NQ)3NJ", "64L)FZC", "S26)Y9Q", "4N5)CS2", "X7P)5G2", "RKS)SGV", "PNK)W71",
    "9KN)R7W", "K2Y)9R5", "8J4)63K", "9H1)TPX", "LMB)HY1", "NVK)N96", "5ZC)B95", "PPM)5G3",
    "CNL)KGZ", "JG9)D5J", "17B)Y9D", "XHN)VWG", "8Z8)HF4", "32R)D8H", "F38)XX1", "PLJ)TKH",
    "38M)MDP", "HYT)HQ4", "NBY)WQW", "GSQ)GBX", "XT3)2F4", "V7S)D1Z", "3ZW)3CZ", "WWN)F7K",
    "NVP)1LR", "KRY)GSJ", "G9J)D4W", "JXP)PFQ", "Z9D)5ZC", "LYR)KPX", "FSP)8HC", "16J)Q77",
    "MPY)1NB", "F78)R9Z", "RN8)H9B", "F3N)XTF", "PD5)96B", "5L3)183", "X97)RP6", "VD6)ZWF",
    "F4P)GRX", "M77)836", "4FJ)M3N", "HFN)8J4", "NRP)GM7", "QSY)C36", "MSV)XZJ", "126)L6Y",
    "99H)XDD", "Z5T)BTY", "TNW)8WF", "HJ8)4SC", "9LR)BRC", "F7X)SPS", "1VR)B84", "9R5)GMF",
    "H1Y)NB5", "6DP)TXV", "4J9)N4S", "8Y2)PTF", "YBJ)ZDK", "1WS)83R", "BMS)FSS", "MG9)FLV",
    "BM1)QML", "1SK)KDD", "SF5)X5V", "3HD)9QK", "P9X)ZR1", "YC5)HJC", "BKR)V3C", "WD9)ZPF",
    "TNW)S2K", "GDL)Z18", "5D9)F52", "ZD2)NVH", "F55)345", "6N4)7TV", "X5V)9XG", "64K)YBJ",
    "8Y9)ZQZ", "LCZ)MFG", "H4M)Z8F", "V2W)J7P", "73H)H9J", "V7D)M5P", "FN4)PNK", "NK4)R96",
    "T1W)914", "HY1)HWY", "H5D)8Y2", "3NJ)QYD", "2LJ)DYV", "SL6)KVT", "H2Q)DXD", "CFF)L6J",
    "D5J)F86", "41S)8R7", "H9J)YZK", "QW1)X97", "B27)GMP", "8V1)GQ6", "8VM)F3N", "Y4H)8GD",
    "SHJ)32R",
];

struct BaumKnoten<'s> {
    label: &'s str,
    first_child: Option<Box<BaumKnoten<'s>>>,
    next_sibling: Option<Box<BaumKnoten<'s>>>,
}

impl<'s> BaumKnoten<'s> {
    fn find<'r: 's>(&mut self, label: &'r str) -> Option<&mut BaumKnoten<'s>> {
        // println!("looking if self {} is {}", self.label, label);
        if self.label == label {
            // println!("found {} in self", label);
            return Some(self);
        }

        // println!("looking in first child for {}", label);
        let o_fc = self.first_child.as_mut().and_then(|fc| fc.find(label));
        if o_fc.is_some() {
            // println!("found {} in first child", label);
            return o_fc;
        }

        // println!("looking in next sibling for {}", label);
        let o_ns = self.next_sibling.as_mut().and_then(|ns| ns.find(label));
        if o_ns.is_some() {
            // println!("found {} in next sibling", label);
            return o_ns;
        }

        return None;
    }

    // #[allow(unused)]
    // fn add_child_by_subtree(&mut self, subtree: BaumKnoten<'s>) {
    //     if self.first_child.is_none() {
    //         self.first_child = Some(Box::new(subtree));
    //     } else {
    //         let mut walk = self.first_child.as_mut().unwrap();
    //         while walk.next_sibling.is_some() {
    //             walk = walk.next_sibling.as_mut().unwrap();
    //         }
    //         walk.next_sibling = Some(Box::new(subtree));
    //     }
    // }

    fn add_child_by_label<'r: 's>(&mut self, label: &'r str) {
        if self.first_child.is_none() {
            self.first_child = Some(Box::new(BaumKnoten {
                label,
                first_child: None,
                next_sibling: None,
            }));
        } else {
            let mut walk = self.first_child.as_mut().unwrap();
            while walk.next_sibling.is_some() {
                walk = walk.next_sibling.as_mut().unwrap();
            }
            walk.next_sibling = Some(Box::new(BaumKnoten {
                label,
                first_child: None,
                next_sibling: None,
            }));
        }
    }

    fn new<'r: 's>(label: &'r str) -> BaumKnoten<'s> {
        return BaumKnoten {
            label,
            first_child: None,
            next_sibling: None,
        };
    }

    fn generate_graphviz<'r: 's>(&self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();

        if self.first_child.is_none() {
            return res;
        }

        let mut child = self.first_child.as_ref().unwrap();
        res.push(format!("\"{}\" -> \"{}\";", self.label, child.label));
        res.append(&mut child.generate_graphviz());
        while child.next_sibling.is_some() {
            child = child.next_sibling.as_ref().unwrap();
            res.push(format!("\"{}\" -> \"{}\";", self.label, child.label));
            res.append(&mut child.generate_graphviz());
        }

        return res;
    }

    fn part_1_score(&self, base_score: u32) -> u32 {
        let mut res = base_score;

        if self.first_child.is_some() {
            let mut walk = self.first_child.as_ref().unwrap();
            res += walk.part_1_score(base_score + 1);
            while walk.next_sibling.is_some() {
                walk = walk.next_sibling.as_ref().unwrap();
                res += walk.part_1_score(base_score + 1);
            }
        }

        return res;
    }
}

fn build_tree<'a, 'b: 'a>(mut input: Vec<(&'b str, &'b str)>) -> BaumKnoten<'a> {
    let mut root = BaumKnoten::new("COM");

    while input.len() > 0 {
        input.retain(|(parent_label, child_label)| -> bool {
            if let Some(parent) = root.find(parent_label) {
                parent.add_child_by_label(child_label);
                return false;
            }
            return true;
        });
    }

    return root;

    // TODO can I finish an implemntation that only iterates once over input

    // let mut root = BaumKnoten::new("COM");

    // // let mut disc_subt: Vec<&mut BaumKnoten> = Vec::new(); // disconnected subtrees
    // // disc_subt.push(&mut root);

    // let mut disc_subt: HashMap<&str, &mut BaumKnoten> = HashMap::new(); // disconnected subtrees
    // disc_subt.insert("COM", &mut root);

    // 'next_input: for (parent_label, child_label) in input {
    //     let mut parent: Option<&mut BaumKnoten> = None;
    //     for (subtree_root_label, subtree_root) in disc_subt.iter_mut() {
    //         parent = subtree_root.find(parent_label);
    //         if parent.is_some() {
    //             break;
    //         }

    //         // if let Some(parent) = subtree_root.find(parent_label) {
    //         //     p = Some(parent);
    //         //     // parent.add_child(child_label);
    //         // }
    //     }

    //     match parent {
    //         Some(p) => match disc_subt.remove(child_label) {
    //             Some(child_subtree) => {
    //                 // p.add_child_by_subtree(child_subtree);
    //             }
    //             None => {
    //                 // p.add_child_by_label(child_label);
    //             }
    //         },
    //         None => match disc_subt.remove(child_label) {
    //             Some(child_subtree) => {}
    //             None => {}
    //         },
    //     }

    //     // for subtree_root in &mut disc_subt {
    //     //     if let Some(parent) = subtree_root.find(parent_label) {}
    //     // }

    //     // root.find(parent).unwrap().add_child(child);
    //     // match disc_subt.remove(parent_label) {
    //     //     Some(parent) => {}
    //     //     None => {
    //     //         disconnected.push(BaumKnoten::new());
    //     //     }
    //     // }

    //     // let p = root.find(parent);
    //     // match p {
    //     //     Some(pp) => {
    //     //         pp.add_child(child);
    //     //     }
    //     //     None => {
    //     //         println!(
    //     //             "should have found parent {} of {} but didn't",
    //     //             parent, child
    //     //         );
    //     //         break;
    //     //     }
    //     // }
    // }

    // let mut root = BaumKnoten::new("COM");
    // return root;
}

fn parse_input<'a>(input: &'a [&str]) -> Vec<(&'a str, &'a str)> {
    input
        .iter()
        .map(|s| -> (&str, &str) {
            let vec: Vec<&str> = s.split(")").collect();
            return (vec[0], vec[1]);
        })
        // .collect::<Vec<(&str, &str)>>()
        .collect()
}

// struct BaumKnoten<'a> {
//     label: &'a str,
//     first_child: Option<&'a mut BaumKnoten<'a>>,
//     next_sibling: Option<&'a mut BaumKnoten<'a>>,
// }
//
// impl<'a> BaumKnoten<'a> {
//     fn find<'b>(&'a mut self, label: &'b str) -> Option<&'a mut BaumKnoten<'a>> {
//         if self.label == label {
//             return Some(self);
//         }
//
//         let o_fc = (self.first_child).as_mut().and_then(|fc| fc.find(label));
//         if o_fc.is_some() {
//             return o_fc;
//         }
//
//         let o_ns = (self.next_sibling).as_mut().and_then(|ns| ns.find(label));
//         if o_ns.is_some() {
//             return o_ns;
//         }
//
//         return None;
//     }
//
//     fn add_child(&'a mut self, label: &'a str) {
//         if self.first_child.is_none() {
//             self.first_child = Some(&mut BaumKnoten {
//                 label,
//                 first_child: None,
//                 next_sibling: None,
//             });
//         } else {
//         }
//     }
// }

// fn find<'a, 'b>(root: &'a mut BaumKnoten<'a>, label: &'b str) -> Option<&'a mut BaumKnoten<'a>> {
//     if root.label == label {
//         return Some(root);
//     }
//
//     let o_fc = (root.first_child).as_mut().and_then(|fc| find(fc, label));
//     if o_fc.is_some() {
//         return o_fc;
//     }
//
//     let o_ns = (root.next_sibling).as_mut().and_then(|ns| find(ns, label));
//     if o_ns.is_some() {
//         return o_ns;
//     }
//
//     return None;
// }

#[test]
fn test_input() {
    let tree = build_tree(parse_input(&TEST_INPUT));
    assert_eq!(tree.part_1_score(0), 42);
}

fn main() -> std::io::Result<()> {
    let tree = build_tree(parse_input(&TEST_INPUT));
    let graphviz = tree.generate_graphviz();

    let mut test_gv = File::create("/tmp/test.gv")?;
    test_gv.write_all("digraph G{".as_bytes())?;
    test_gv.write_all(graphviz.join("").as_bytes())?;
    test_gv.write_all("}".as_bytes())?;
    println!("wrote /tmp/test.gv");

    let tree = build_tree(parse_input(&INPUT));
    let graphviz = tree.generate_graphviz();

    let mut input_gv = File::create("/tmp/input.gv")?;
    input_gv.write_all("digraph G{".as_bytes())?;
    input_gv.write_all(graphviz.join("").as_bytes())?;
    input_gv.write_all("}".as_bytes())?;
    println!("wrote /tmp/input.gv");

    println!("part 1: {}", tree.part_1_score(0));

    return Ok(());
}

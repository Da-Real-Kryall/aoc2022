// Day 1 solution by Da-Real-Kryall

use tap::prelude::*;

pub fn part1() -> String {
    include_str!("../inputs/day01.txt")
        .split("\n\n")
        .map(|x| x.lines().map(|y| y.parse::<i128>().unwrap()).sum::<i128>())
        .max()
        .unwrap()
        .to_string()

    //let input = include_str!("../inputs/day01.txt");
    //let mut current_total = 0;
    //let mut max = 0;
    //for line in input.lines() {
    //    if line == "" {
    //        current_total = 0;
    //        continue;
    //    }
    //    current_total = current_total + line.parse::<i128>().unwrap();
    //    if current_total > max {
    //        max = current_total;
    //    }
    //}
    //max.to_string()
}

//208191
pub fn part2() -> String {
    include_str!("../inputs/day01.txt")
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .map(|y| y.parse::<i128>().unwrap())
                .sum::<i128>()
        })
        .collect::<Vec<i128>>()
        .tap_mut(|s| s.sort())
        .into_iter()
        .rev()
        .take(3)
        .sum::<i128>().to_string()
}
    //let input = include_str!("../inputs/day01.txt");
    //let mut current_total = 0;
    //let mut max: [i128; 3] = [0, 0, 0];
    //for line in input.lines() {
    //    if line == "" {
    //        if current_total > max[0] {
    //            max[2] = max[1];
    //            max[1] = max[0];
    //            max[0] = current_total;
    //        }
    //        else if current_total > max[1] {
    //            max[2] = max[1];
    //            max[1] = current_total;
    //        }
    //        else if current_total > max[2] {
    //            max[2] = current_total;
    //        }
    //        current_total = 0;
    //        continue;
    //    }
    //    current_total = current_total + line.parse::<i128>().unwrap();
    //}
    //(max[0]+max[1]+max[2]).to_string()
//}

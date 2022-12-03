//this takes in two arguments; the day number and part number
//it runs either part1 or part2 functions in main.rs in dayXX folder

use std::env;

mod solutions;


fn main() {
    // Get the day number and part number from the command line
    let args: Vec<String> = env::args().collect();
    let day = {
        let x = &args[1];
        if x.len() == 1 {
            format!("0{}", x).to_string()
        } else {
            x.to_string()
        }
    };
    let day = day.as_str();

    let part = &args[2];

    // Run the solution
    let solution = match day {
        "01" => {
            if part == "1" {
                solutions::day01::part1()
            } else {
                solutions::day01::part2()
            }
        }
        "02" => {
            if part == "1" {
                solutions::day02::part1()
            } else {
                solutions::day02::part2()
            }
        }
        "03" => {
            if part == "1" {
                solutions::day03::part1()
            } else {
                solutions::day03::part2()
            }
        }
        "04" => {
            if part == "1" {
                solutions::day04::part1()
            } else {
                solutions::day04::part2()
            }
        }
        "05" => {
            if part == "1" {
                solutions::day05::part1()
            } else {
                solutions::day05::part2()
            }
        }
        "06" => {
            if part == "1" {
                solutions::day06::part1()
            } else {
                solutions::day06::part2()
            }
        }
        "07" => {
            if part == "1" {
                solutions::day07::part1()
            } else {
                solutions::day07::part2()
            }
        }
        "08" => {
            if part == "1" {
                solutions::day08::part1()
            } else {
                solutions::day08::part2()
            }
        }
        "09" => {
            if part == "1" {
                solutions::day09::part1()
            } else {
                solutions::day09::part2()
            }
        }
        "10" => {
            if part == "1" {
                solutions::day10::part1()
            } else {
                solutions::day10::part2()
            }
        }
        "11" => {
            if part == "1" {
                solutions::day11::part1()
            } else {
                solutions::day11::part2()
            }
        }
        "12" => {
            if part == "1" {
                solutions::day12::part1()
            } else {
                solutions::day12::part2()
            }
        }
        "13" => {
            if part == "1" {
                solutions::day13::part1()
            } else {
                solutions::day13::part2()
            }
        }
        "14" => {
            if part == "1" {
                solutions::day14::part1()
            } else {
                solutions::day14::part2()
            }
        }
        "15" => {
            if part == "1" {
                solutions::day15::part1()
            } else {
                solutions::day15::part2()
            }
        }
        "16" => {
            if part == "1" {
                solutions::day16::part1()
            } else {
                solutions::day16::part2()
            }
        }
        "17" => {
            if part == "1" {
                solutions::day17::part1()
            } else {
                solutions::day17::part2()
            }
        }
        "18" => {
            if part == "1" {
                solutions::day18::part1()
            } else {
                solutions::day18::part2()
            }
        }
        "19" => {
            if part == "1" {
                solutions::day19::part1()
            } else {
                solutions::day19::part2()
            }
        }
        "20" => {
            if part == "1" {
                solutions::day20::part1()
            } else {
                solutions::day20::part2()
            }
        }
        "21" => {
            if part == "1" {
                solutions::day21::part1()
            } else {
                solutions::day21::part2()
            }
        }
        "22" => {
            if part == "1" {
                solutions::day22::part1()
            } else {
                solutions::day22::part2()
            }
        }
        "23" => {
            if part == "1" {
                solutions::day23::part1()
            } else {
                solutions::day23::part2()
            }
        }
        "24" => {
            if part == "1" {
                solutions::day24::part1()
            } else {
                solutions::day24::part2()
            }
        }
        "25" => {
            if part == "1" {
                solutions::day25::part1()
            } else {
                solutions::day25::part2()
            }
        }
        _ => {
            println!("Day {} not implemented yet", day);
            0
        }
    };
    // Print the solution
    println!("{}", solution);
}
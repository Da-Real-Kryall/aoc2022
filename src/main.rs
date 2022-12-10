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

    let part = args[2].as_str();

    // Run the solution
    let solution: String = match day {
        "01" => match part {
            "1" => solutions::day01::part1(),
            "2" => solutions::day01::part2(),
            _ => panic!("Invalid part number"),
        },
        "02" => match part {
            "1" => solutions::day02::part1(),
            "2" => solutions::day02::part2(),
            _ => panic!("Invalid part number"),
        },
        "03" => match part {
            "1" => solutions::day03::part1(),
            "2" => solutions::day03::part2(),
            _ => panic!("Invalid part number"),
        },
        "04" => match part {
            "1" => solutions::day04::part1(),
            "2" => solutions::day04::part2(),
            _ => panic!("Invalid part number"),
        },
        "05" => match part {
            "1" => solutions::day05::part1(),
            "2" => solutions::day05::part2(),
            _ => panic!("Invalid part number"),
        },
        "06" => match part {
            "1" => solutions::day06::part1(),
            "2" => solutions::day06::part2(),
            _ => panic!("Invalid part number"),
        },
        "07" => match part {
            "1" => solutions::day07::part1(),
            "2" => solutions::day07::part2(),
            _ => panic!("Invalid part number"),
        },
        "08" => match part {
            "1" => solutions::day08::part1(),
            "2" => solutions::day08::part2(),
            _ => panic!("Invalid part number"),
        },
        "09" => match part {
            "1" => solutions::day09::part1(),
            "2" => solutions::day09::part2(),
            _ => panic!("Invalid part number"),
        },
        "10" => match part {
            "1" => solutions::day10::part1(),
            "2" => solutions::day10::part2(),
            _ => panic!("Invalid part number"),
        },
        "11" => match part {
            "1" => solutions::day11::part1(),
            "2" => solutions::day11::part2(),
            _ => panic!("Invalid part number"),
        },
        "12" => match part {
            "1" => solutions::day12::part1(),
            "2" => solutions::day12::part2(),
            _ => panic!("Invalid part number"),
        },
        "13" => match part {
            "1" => solutions::day13::part1(),
            "2" => solutions::day13::part2(),
            _ => panic!("Invalid part number"),
        },
        "14" => match part {
            "1" => solutions::day14::part1(),
            "2" => solutions::day14::part2(),
            _ => panic!("Invalid part number"),
        },
        "15" => match part {
            "1" => solutions::day15::part1(),
            "2" => solutions::day15::part2(),
            _ => panic!("Invalid part number"),
        },
        "16" => match part {
            "1" => solutions::day16::part1(),
            "2" => solutions::day16::part2(),
            _ => panic!("Invalid part number"),
        },
        "17" => match part {
            "1" => solutions::day17::part1(),
            "2" => solutions::day17::part2(),
            _ => panic!("Invalid part number"),
        },
        "18" => match part {
            "1" => solutions::day18::part1(),
            "2" => solutions::day18::part2(),
            _ => panic!("Invalid part number"),
        },
        "19" => match part {
            "1" => solutions::day19::part1(),
            "2" => solutions::day19::part2(),
            _ => panic!("Invalid part number"),
        },
        "20" => match part {
            "1" => solutions::day20::part1(),
            "2" => solutions::day20::part2(),
            _ => panic!("Invalid part number"),
        },
        "21" => match part {
            "1" => solutions::day21::part1(),
            "2" => solutions::day21::part2(),
            _ => panic!("Invalid part number"),
        },
        "22" => match part {
            "1" => solutions::day22::part1(),
            "2" => solutions::day22::part2(),
            _ => panic!("Invalid part number"),
        },
        "23" => match part {
            "1" => solutions::day23::part1(),
            "2" => solutions::day23::part2(),
            _ => panic!("Invalid part number"),
        },
        "24" => match part {
            "1" => solutions::day24::part1(),
            "2" => solutions::day24::part2(),
            _ => panic!("Invalid part number"),
        },
        "25" => match part {
            "1" => solutions::day25::part1(),
            "2" => solutions::day25::part2(),
            _ => panic!("Invalid part number"),
        },
        _ => panic!("Invalid day number"),
    };
    // Print the solution
    println!("{}", solution);
}

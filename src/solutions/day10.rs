// Day 10 solution by Da-Real-Kryall
fn advance_clock_1(x: &i32, cycle_num: &mut i32, signal_strength_sum: &mut i32, num_iter: i32) -> bool {
    for _ in 0..num_iter {
        *cycle_num += 1;
        if [20, 60, 100, 140, 180, 220].contains(cycle_num) {
            *signal_strength_sum += *cycle_num * *x;
            if *cycle_num == 220 {
                return true;
            }
        }
    }
    return false;
}
pub fn part1() -> String {
    let input = include_str!("../inputs/day10.txt");
    let mut x: i32 = 1;
    let mut cycle_num: i32 = 0;
    let mut signal_strength_sum: i32 = 0;
    for line in input.lines() {
        let mut instruction = line.split(" ");
        match instruction.next().unwrap() {
            "noop" => {
                if advance_clock_1(&x, &mut cycle_num, &mut signal_strength_sum, 1) {
                    return signal_strength_sum.to_string();
                }
            }
            "addx" => {
                if advance_clock_1(&x, &mut cycle_num, &mut signal_strength_sum, 2) {
                    return signal_strength_sum.to_string();
                }
                x += instruction.next().unwrap().parse::<i32>().unwrap()
            }
            _ => {}
        }
    }
    return signal_strength_sum.to_string()
}

fn advance_clock_2(x: &i32, cycle_num: &mut i32, screen: &mut String, num_iter: i32) {
    for _ in 0..num_iter {
        if *cycle_num % 40 == 0 {
            screen.push('\n');
        }
        if (*cycle_num % 40).abs_diff(*x) <= 1 {
            screen.push('█');
        } else {
            screen.push(' ');
        }
        *cycle_num += 1;
    }
}
pub fn part2() -> String {
    let input = include_str!("../inputs/day10.txt");

    let mut x: i32 = 1;
    let mut cycle_num: i32 = 0;
    let mut screen: String = String::new();
    
    for line in input.lines() {
        let mut instruction = line.split(" ");
        match instruction.next().unwrap() {
            "noop" => {
                advance_clock_2(&x, &mut cycle_num, &mut screen, 1);
            }
            "addx" => {
                advance_clock_2(&x, &mut cycle_num, &mut screen, 2);
                x += instruction.next().unwrap().parse::<i32>().unwrap()
            }
            _ => {}
        }
    }
    return screen;
}
// Day 4 solution by Da-Real-Kryall

pub fn part1() -> String {
    let input = include_str!("../inputs/day04.txt");
    let mut current_total = 0;
    for line in input.lines() {
        // 18-49,17-19
        let split = line.split(",");
        let range_a_string = split.clone().next().unwrap().to_string();
        let range_b_string = split.clone().last().unwrap().to_string();
        let range_a: (i128, i128) = (
            range_a_string.split("-").next().unwrap().parse().unwrap(),
            range_a_string.split("-").last().unwrap().parse().unwrap(),
        );
        let range_b: (i128, i128) = (
            range_b_string.split("-").next().unwrap().parse().unwrap(),
            range_b_string.split("-").last().unwrap().parse().unwrap(),
        );
        //check if a contains b
        if range_a.0 <= range_b.0 && range_a.1 >= range_b.1 {
            current_total += 1;
            println!("{}-{} contains {}-{}", range_a.0, range_a.1, range_b.0, range_b.1);
        }
        // now check if b contains a
        else if range_b.0 <= range_a.0 && range_b.1 >= range_a.1 {
            current_total += 1;
            println!("{}-{} contains {}-{}", range_b.0, range_b.1, range_a.0, range_a.1);
        }
    }
    current_total.to_string()
}

pub fn part2() -> String {
    let input = include_str!("../inputs/day04.txt");
    let mut current_total = 0;
    for line in input.lines() {
        // 18-49,17-19
        let split = line.split(",");
        let range_a_string = split.clone().next().unwrap().to_string();
        let range_b_string = split.clone().last().unwrap().to_string();
        let range_a: (i128, i128) = (
            range_a_string.split("-").next().unwrap().parse().unwrap(),
            range_a_string.split("-").last().unwrap().parse().unwrap(),
        );
        let range_b: (i128, i128) = (
            range_b_string.split("-").next().unwrap().parse().unwrap(),
            range_b_string.split("-").last().unwrap().parse().unwrap(),
        );
        // if it intersects, it means one point is inside the range of the other. because this is reciprocal, i only need to check if a's start or end intersects b at all.
        if (range_a.0 >= range_b.0 && range_a.0 <= range_b.1) || (range_a.1 >= range_b.0 && range_a.0 <= range_b.1) {
            current_total += 1;
            println!("{}-{} intersects {}-{}", range_a.0, range_a.1, range_b.0, range_b.1);
        }
    }
    current_total.to_string()
}
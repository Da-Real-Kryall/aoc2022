// Day 11 solution by Da-Real-Kryall

pub fn part1() -> String {
    let input = include_str!("../inputs/day11.txt");
    //items, operation particles, divisible by, monkey to throw to if true, monkey if false, number of throws
    let mut monkeys: Vec<(Vec<usize>, Vec<String>, usize, usize, usize, usize)> = vec![];
    for string in input.split("\n\n") {
        let mut lines = string.split("\n").into_iter().skip(1);

        let starting_items: Vec<usize> = lines.next().unwrap()[18..]
            .split(", ")
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let operation_particles: Vec<String> = lines.next().unwrap()[19..]
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>(); //  Test: divisible by

        let divisible_by: usize = lines.next().unwrap()[21..].parse::<usize>().unwrap();

        let true_monke: usize = lines.next().unwrap()[29..].parse::<usize>().unwrap();

        let false_monke: usize = lines.next().unwrap()[30..].parse::<usize>().unwrap();

        let activity = 0;

        monkeys.push((
            starting_items,
            operation_particles,
            divisible_by,
            true_monke,
            false_monke,
            activity,
        ));
    }

    for _ in 0..20 {
        for index in 0..monkeys.len() {
            for worry_level in monkeys[index].0.clone() {
                let var = match monkeys[index].1[2].as_str() {
                    "old" => worry_level,
                    _ => monkeys[index].1[2].parse::<usize>().unwrap(),
                };

                let end = match monkeys[index].1[1].as_str() {
                    "*" => worry_level * var,
                    "+" => worry_level + var,
                    _ => 0,
                } / 3;

                monkeys[index].5 += 1;

                let recipient;
                if end % monkeys[index].2 == 0 {
                    recipient = monkeys[index].3;
                } else {
                    recipient = monkeys[index].4;
                }
                monkeys[recipient].0.push(end);
            }
            monkeys[index].0 = Vec::new();
        }
    }

    let mut two_highest = monkeys.iter().map(|x| x.5).collect::<Vec<usize>>();
    two_highest.sort();
    two_highest.reverse();
    two_highest = two_highest.into_iter().take(2).collect::<Vec<usize>>();

    (two_highest[0] * two_highest[1]).to_string()
}

pub fn part2() -> String {
    let input = include_str!("../inputs/day11.txt");
    //items, operation particles, divisible by, monkey to throw to if true, monkey if false, number of throws
    let mut monkeys: Vec<(Vec<usize>, Vec<String>, usize, usize, usize, usize)> = vec![];
    for string in input.split("\n\n") {
        let mut lines = string.split("\n").into_iter().skip(1);

        let starting_items: Vec<usize> = lines.next().unwrap()[18..]
            .split(", ")
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let operation_particles: Vec<String> = lines.next().unwrap()[19..]
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>(); //  Test: divisible by

        let divisible_by: usize = lines.next().unwrap()[21..].parse::<usize>().unwrap();

        let true_monke: usize = lines.next().unwrap()[29..].parse::<usize>().unwrap();

        let false_monke: usize = lines.next().unwrap()[30..].parse::<usize>().unwrap();

        let activity = 0;

        monkeys.push((
            starting_items,
            operation_particles,
            divisible_by,
            true_monke,
            false_monke,
            activity,
        ));
    }

    for _ in 0..10000 {
        for index in 0..monkeys.len() {
            for worry_level in monkeys[index].0.clone() {
                let var = match monkeys[index].1[2].as_str() {
                    "old" => worry_level,
                    _ => monkeys[index].1[2].parse::<usize>().unwrap(),
                };

                let mut end = match monkeys[index].1[1].as_str() {
                    "*" => worry_level * var,
                    "+" => worry_level + var,
                    _ => 0,
                };

                end %= {
                    let mut x = monkeys[0].2;
                    for monkey in monkeys.clone().iter().skip(1) {
                        x *= monkey.2;
                    }
                    x
                };

                monkeys[index].5 += 1;

                let recipient;
                if end % monkeys[index].2 == 0 {
                    recipient = monkeys[index].3;
                } else {
                    recipient = monkeys[index].4;
                }
                monkeys[recipient].0.push(end);
            }
            monkeys[index].0 = Vec::new();
        }
    }

    let mut two_highest = monkeys.iter().map(|x| x.5).collect::<Vec<usize>>();
    two_highest.sort();
    two_highest.reverse();
    two_highest = two_highest.into_iter().take(2).collect::<Vec<usize>>();

    (two_highest[0] * two_highest[1]).to_string()
}
//120062 is too low
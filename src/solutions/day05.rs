// Day 5 solution by Da-Real-Kryall

/*
[V]     [B]                     [C]
[C]     [N] [G]         [W]     [P]
[W]     [C] [Q] [S]     [C]     [M]
[L]     [W] [B] [Z]     [F] [S] [V]
[R]     [G] [H] [F] [P] [V] [M] [T]
[M] [L] [R] [D] [L] [N] [P] [D] [W]
[F] [Q] [S] [C] [G] [G] [Z] [P] [N]
[Q] [D] [P] [L] [V] [D] [D] [C] [Z]
 1   2   3   4   5   6   7   8   9
*/

pub fn part1() -> String {
    fn move_box(args: Vec<&str>, boxes: &mut Vec<Vec<char>>) {
        let amount: usize = args[1].parse::<usize>().unwrap();
        let start: usize = args[3].parse::<usize>().unwrap() - 1;
        let dest: usize = args[5].parse::<usize>().unwrap() - 1;

        //println!("Moving {} from \n{:?} \nto: \n{:?}", amount, boxes[start], boxes[dest]);
        for _ in 0..amount {
            let box_char = *boxes[start].clone().last().unwrap();
            boxes[dest].push(box_char.clone());
            boxes[start].pop();
        }
        //println!("Result: \n{:?} \n{:?}\n\n", boxes[start], boxes[dest]);
    }

    let input = include_str!("../inputs/day05.txt");
    let num_piles: usize = 9;
    let mut piles: Vec<Vec<char>> = vec![vec![]; num_piles];

    for line in input.lines() {
        if line == "" {
            break;
        }
        let letter_indexes: Vec<usize> = line.match_indices("[").map(|(i, _)| i + 1).collect();
        for letter_index in letter_indexes {
            let box_char = line.chars().nth(letter_index).unwrap();
            piles[letter_index / 4].push(box_char);
        }
    }

    let mut piles: Vec<Vec<char>> = piles
        .into_iter()
        .map(|pile: Vec<char>| pile.into_iter().rev().collect())
        .collect();

    //move boxes
    for line in input.lines().skip(10) {
        move_box(line.split_whitespace().collect::<Vec<&str>>(), &mut piles);
    }

    piles
        .into_iter()
        .map(|pile: Vec<char>| pile.last().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("")
}

pub fn part2() -> String {
    fn move_box(args: Vec<&str>, boxes: &mut Vec<Vec<char>>) {
        let amount: usize = args[1].parse::<usize>().unwrap();
        let start: usize = args[3].parse::<usize>().unwrap() - 1;
        let dest: usize = args[5].parse::<usize>().unwrap() - 1;

        let mut picked_up_boxes: Vec<char> = vec![];
        for _ in 0..amount {
            let box_char = *boxes[start].clone().last().unwrap();
            picked_up_boxes.push(box_char.clone());
            boxes[start].pop();
        }
        picked_up_boxes.reverse();
        boxes[dest].append(&mut picked_up_boxes);
    }

    let input = include_str!("../inputs/day05.txt");
    let num_piles: usize = 9;
    let mut piles: Vec<Vec<char>> = vec![vec![]; num_piles];

    for line in input.lines() {
        if line == "" {
            break;
        }
        let letter_indexes: Vec<usize> = line.match_indices("[").map(|(i, _)| i + 1).collect();
        for letter_index in letter_indexes {
            let box_char = line.chars().nth(letter_index).unwrap();
            piles[letter_index / 4].push(box_char);
        }
    }

    let mut piles: Vec<Vec<char>> = piles
        .into_iter()
        .map(|pile: Vec<char>| pile.into_iter().rev().collect())
        .collect();

    //move boxes
    for line in input.lines().skip(10) {
        move_box(line.split_whitespace().collect::<Vec<&str>>(), &mut piles);
    }

    piles
        .into_iter()
        .map(|pile: Vec<char>| pile.last().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("")
}

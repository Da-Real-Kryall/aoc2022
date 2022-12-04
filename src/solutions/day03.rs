// Day 3 solution by Da-Real-Kryall

fn priority(letter: char) -> i128 {
    match letter {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}

pub fn part1() -> String {
    let input = include_str!("../inputs/day03.txt");
    let mut total_score: i128 = 0;
    for line in input.lines() {
        let mut sack_a = String::new();
        let mut found_char = false;
        for (index, letter) in line.chars().enumerate() {
            if found_char == false {
                if index < line.len() / 2 {
                    sack_a = format!("{}{}", sack_a, letter);
                } else {
                    for char in sack_a.chars() {
                        if char == letter && found_char == false{
                            println!("{}", char);
                            println!("{}", priority(char));
                            total_score += priority(letter);
                            found_char = true;
                        }
                    }
                }
            }
        }
    }
    total_score.to_string()
}

pub fn part2() -> String {
    let input = include_str!("../inputs/day03.txt");
    let mut candidates = String::new();
    let mut total_score: i128 = 0;

    for index in 0..(input.lines().count() / 3) {

        let sack_a = input.lines().nth(index * 3).unwrap();
        let sack_b = input.lines().nth(index * 3 + 1).unwrap();
        let sack_c = input.lines().nth(index * 3 + 2).unwrap();

        let sacks = vec![sack_a, sack_b, sack_c];
        let mut candidates = String::new();

        for sack in sacks {
            for letter in sack.chars() {
                if sack_a.matches(letter).count() != 0 && sack_b.matches(letter).count() != 0 && sack_c.matches(letter).count() != 0 && candidates.matches(letter).count() == 0 {
                    candidates = format!("{}{}", candidates, letter);
                }
            }
        }

        for letter in candidates.chars() {
            total_score += priority(letter);
        }
        
    }
    total_score.to_string()
}

// why are you depressed? there's so much goodness in the world!
// idk, why do you have athsma? there's so much air in the world!

// Day 2 solution by Da-Real-Kryall

// A is rock, B is paper and C is scissors for opponent
// X is rock, Y is paper and Z is scissors for response
pub fn part1() -> String {
    let input = include_str!("../inputs/day02.txt");
    let mut total_score: i128 = 0;
    for line in input.lines() {
        // opponent is first letter in line
        let opponent: char = line.chars().nth(0).unwrap();
        let response: char = line.chars().nth(2).unwrap();
        total_score += match opponent {
            'A' => match response {
                //rock
                'X' => 1 + 3, //rock
                'Y' => 2 + 6, //paper
                'Z' => 3 + 0, //scissors
                _ => 0,
            },
            'B' => match response {
                //paper
                'X' => 1 + 0,
                'Y' => 2 + 3,
                'Z' => 3 + 6,
                _ => 0,
            },
            'C' => match response {
                //scissors
                'X' => 1 + 6,
                'Y' => 2 + 0,
                'Z' => 3 + 3,
                _ => 0,
            },
            _ => 0,
        };
    }
    total_score.to_string()
}

pub fn part2() -> String {
    let input = include_str!("../inputs/day02.txt");
    let mut current_total = 0;

    // X means I need to lose.
    // Y means I need to end in a draw.
    // Z means I need to win.

    // rock is 1, paper is 2 and scissors is 3

    for line in input.lines() {
        let opponent = line.chars().nth(0).unwrap();
        let response = line.chars().nth(2).unwrap();
        current_total += match opponent {
            'A' => match response {
                //rock
                'X' => 0 + 3, //lose
                'Y' => 3 + 1, //draw
                'Z' => 6 + 2, //win
                _ => 0,
            },
            'B' => match response {
                //paper
                'X' => 0 + 1,
                'Y' => 3 + 2,
                'Z' => 6 + 3,
                _ => 0,
            },
            'C' => match response {
                //scissors
                'X' => 0 + 2,
                'Y' => 3 + 3,
                'Z' => 6 + 1,
                _ => 0,
            },
            _ => 0,
        }
    }
    current_total.to_string()
}

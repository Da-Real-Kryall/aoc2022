// Day 6 solution by Da-Real-Kryall

pub fn part1() -> String {
    let input = include_str!("../inputs/day06.txt");
    let mut letter_history: String = String::new();
    for i in 0..3 {
        letter_history += input.chars().nth(i).unwrap().to_string().as_str();
    }
    for (index, letter) in input.chars().into_iter().enumerate().skip(3) {
        letter_history += letter.to_string().as_str();
        //println!("{}, {}", letter_history, letter);
        let mut all_1 = true;
        for letter in letter_history.chars() {
            if letter_history.chars().filter(|&x| x == letter).count() != 1 {
                all_1 = false;
                break;
            }
        }
        if all_1 {
            return (index+1).to_string();
        }
        println!("{}", letter_history);
        letter_history = letter_history.chars().skip(1).collect::<String>();
    }
    return "not found :(".to_string();
}

pub fn part2() -> String {
    let input = include_str!("../inputs/day06.txt");
    let mut letter_history: String = String::new();
    for i in 0..13 {
        letter_history += input.chars().nth(i).unwrap().to_string().as_str();
    }
    for (index, letter) in input.chars().into_iter().enumerate().skip(3) {
        letter_history += letter.to_string().as_str();
        //println!("{}, {}", letter_history, letter);
        let mut all_1 = true;
        for letter in letter_history.chars() {
            if letter_history.chars().filter(|&x| x == letter).count() != 1 {
                all_1 = false;
                break;
            }
        }
        if all_1 {
            return (index+1).to_string();
        }
        println!("{}", letter_history);
        letter_history = letter_history.chars().skip(1).collect::<String>();
    }
    return "not found :(".to_string();
}
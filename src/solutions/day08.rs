// Day 8 solution by Da-Real-Kryall

const SIZE: usize = 99;

fn check_visible(x: usize, y: usize, tree_map: &[[u32; SIZE]; SIZE]) -> bool {
    let tree_height = tree_map[y][x];

    let mut visible = false;

    for (i, v) in [x, y].iter().enumerate() {
        for range in [0..*v, *v + 1..SIZE] {
            let mut visible_from_side = true;
            for _v in range {
                if tree_map[[y, _v][i]][[_v, x][i]] >= tree_height {
                    visible_from_side = false;
                    break;
                }
            }
            visible |= visible_from_side;
        }
    }
    visible
}

pub fn part1() -> String {
    let input = include_str!("../inputs/day08.txt");
    let mut tree_map: [[u32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    for y in 0..SIZE {
        for x in 0..SIZE {
            tree_map[y][x] = input
                .lines()
                .nth(y)
                .unwrap()
                .chars()
                .nth(x)
                .unwrap()
                .to_digit(10)
                .unwrap();
            print!("{}", tree_map[y][x]);
        }
        println!();
    }
    println!();

    let mut current_total = 0;
    for y in 0..SIZE {
        for x in 0..SIZE {
            if check_visible(x, y, &tree_map) {
                print!("1");
                current_total += 1;
            } else {
                print!("0");
            }
        }
        println!("");
    }

    current_total.to_string()
}

pub fn part2() -> String {
    let input = include_str!("../inputs/day08.txt");
    let mut current_total = 0;
    for y in 1..SIZE - 1 {
        for x in 1..SIZE - 1 {
            let tree_height = input[y * (SIZE + 1) + x..y * (SIZE + 1) + x + 1]
                .parse::<u32>()
                .unwrap();
            let mut score = 1;
            for (i, v) in [x, y].iter().enumerate() {
                for range in [
                    (0..*v).rev().collect::<Vec<_>>(),
                    (*v + 1..SIZE).collect::<Vec<_>>(),
                ] {
                    let mut number_of_trees = 0;
                    for _v in range {
                        number_of_trees += 1;
                        let _i = ([y, _v][i]) * (SIZE + 1) + ([_v, x][i]);
                        if input[_i.._i + 1].parse::<u32>().unwrap() >= tree_height {
                            break;
                        }
                    }
                    score *= number_of_trees;
                }
            }
            current_total = current_total.max(score);
        }
    }
    current_total.to_string()
}

// Day 7 solution by Da-Real-Kryall

fn cd_back(path: &str) -> String {
    let mut tokens = path.split("/").collect::<Vec<&str>>();
    tokens.pop();
    tokens.join("/")
}

pub fn part1() -> String {
    let input = include_str!("../inputs/day07.txt");

    let mut files: Vec<(String, usize)> = Vec::new();

    let mut current_directory: String = String::new();
    for line in input.lines().skip(1) {
        let mut tokens = line.split_whitespace();
        let token = tokens.next().unwrap();
        match token {
            "$" => match tokens.next().unwrap() {
                "cd" => {
                    let token = tokens.next().unwrap();
                    if token == ".." {
                        current_directory = cd_back(&current_directory);
                    } else {
                        current_directory += "/";
                        current_directory += token;
                    }
                }
                _ => {}
            },
            "dir" => {
                let token = tokens.next().unwrap();
                files.push((current_directory.clone() + "/" + token, 0));
            }
            _ => {
                // add file size to parent directory aka current directory
                let file_size = token.parse::<usize>().unwrap();

                let mut temp_dir = current_directory.clone();
                for _ in 0..current_directory.matches("/").count() {
                    for file in files.iter_mut() {
                        if file.0 == temp_dir {
                            file.1 += file_size;
                            break;
                        }
                    }
                    temp_dir = cd_back(&temp_dir);
                }
            }
        }
    }
    println!("{:?}", files);

    let mut current_total = 0;
    for file in files {
        if file.1 <= 100000 {
            current_total += file.1;
        }
    }
    current_total.to_string()
}

pub fn part2() -> String {
    let input = include_str!("../inputs/day07.txt");

    let mut files: Vec<(String, usize)> = Vec::new();

    let mut current_directory: String = String::new();
    for line in input.lines().skip(1) {
        let mut tokens = line.split_whitespace();
        let token = tokens.next().unwrap();
        match token {
            "$" => match tokens.next().unwrap() {
                "cd" => {
                    let token = tokens.next().unwrap();
                    if token == ".." {
                        current_directory = cd_back(&current_directory);
                    } else {
                        current_directory += "/";
                        current_directory += token;
                    }
                }
                _ => {}
            },
            "dir" => {
                let token = tokens.next().unwrap();
                files.push((current_directory.clone() + "/" + token, 0));
            }
            _ => {
                // add file size to parent directory aka current directory
                let file_size = token.parse::<usize>().unwrap();

                let mut temp_dir = current_directory.clone();
                for _ in 0..current_directory.matches("/").count() {
                    for file in files.iter_mut() {
                        if file.0 == temp_dir {
                            file.1 += file_size;
                            break;
                        }
                    }
                    temp_dir = cd_back(&temp_dir);
                }
            }
        }
    }
    //println!("{:?}", files);

    let mut used_space: i128 = 0;
    for file in files.clone() {
        if file.0.matches("/").count() > 1 {
            continue;
        }
        used_space += file.1.clone() as i128;
    }

    let minimum_cleared_space = 30000000 - 70000000 + used_space;
    let mut candidates = Vec::new();
    for file in files.clone() {
        if file.1 as i128 >= minimum_cleared_space {
            candidates.push(file.1)
        }
    }

    let mut smallest: usize = 0;
    for candidate in candidates {
        if smallest == 0 || candidate < smallest {
            smallest = candidate;
        }
    }

    smallest.to_string()
}

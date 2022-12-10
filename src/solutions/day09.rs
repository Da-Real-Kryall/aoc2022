// Day 9 solution by Da-Real-Kryall

pub fn part1() -> String {
    let input = include_str!("../inputs/day09.txt");
    let mut positions_visited: Vec<(i32, i32)> = vec![(0, 0)];
    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    for line in input.lines() {
        let mut line = line.split(" ");
        let direction = line.next().unwrap();
        let distance = line.next().unwrap().parse::<u32>().unwrap();
        for _ in 0..distance {
            let _head_pos = head_position;
            match direction {
                "U" => head_position.1 += 1,
                "D" => head_position.1 -= 1,
                "L" => head_position.0 -= 1,
                "R" => head_position.0 += 1,
                _ => panic!("Invalid direction"),
            }
            let dy: i32 = head_position.1 - tail_position.1;
            let dx: i32 = head_position.0 - tail_position.0;

            if dy.abs() > 1 {
                tail_position.1 += dy.signum();
                if dx.abs() == 1 {
                    tail_position.0 += dx.signum();
                }
            }
            if dx.abs() > 1 {
                tail_position.0 += dx.signum();
                if dy.abs() == 1 {
                    tail_position.1 += dy.signum();
                }
            }
            if !positions_visited.contains(&tail_position) {
                positions_visited.push(tail_position);
            }
        }
    }
    ////visualize the path
    //let mut grid: Vec<Vec<char>> = vec![vec!['.'; 100]; 100];
    //for (x, y) in positions_visited.iter() {
    //    grid[50+*y as usize][50+*x as usize] = '#';
    //}
    //for row in grid.iter() {
    //    for col in row.iter() {
    //        print!("{}", col);
    //    }
    //    println!();
    //}
    let current_total = positions_visited.len();
    current_total.to_string()
}

pub fn part2() -> String {
    let input = include_str!("../inputs/day09.txt");
    let mut positions_visited: Vec<(i32, i32)> = vec![(0, 0)];
    let mut body_positions: [(i32, i32); 10] = [(0, 0); 10];
    for line in input.lines() {
        let mut line = line.split(" ");
        let direction = line.next().unwrap();
        let distance = line.next().unwrap().parse::<u32>().unwrap();
        for _ in 0..distance {
            match direction {
                "U" => body_positions[0].1 += 1,
                "D" => body_positions[0].1 -= 1,
                "L" => body_positions[0].0 -= 1,
                "R" => body_positions[0].0 += 1,
                _ => panic!("Invalid direction"),
            }
            for i in 0..body_positions.len() - 1 {
                let head_position = body_positions[i];

                let dy: i32 = head_position.1 - body_positions[i + 1].1;
                let dx: i32 = head_position.0 - body_positions[i + 1].0;

                if dy.abs() > 1 {
                    body_positions[i + 1].1 += dy.signum();
                    if dx.abs() == 1 {
                        body_positions[i + 1].0 += dx.signum();
                    }
                }
                if dx.abs() > 1 {
                    body_positions[i + 1].0 += dx.signum();
                    if dy.abs() == 1 {
                        body_positions[i + 1].1 += dy.signum();
                    }
                }
            }
            if !positions_visited.contains(&body_positions[9]) {
                positions_visited.push(body_positions[9]);
            }
        }
    }
    let current_total = positions_visited.len();
    current_total.to_string()
}

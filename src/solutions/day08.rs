// Day 8 solution by Da-Real-Kryall

const SIZE: usize = 99;

fn check_visible(x: usize, y: usize, tree_map: &[[u32; SIZE]; SIZE]) -> bool {
    let tree_height = tree_map[y][x];
    
    let mut visible = false;

    for (i, v) in [x, y].iter().enumerate() {
        for range in [0..*v, *v+1..SIZE] {
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

    
    //let tree_height = tree_map[y][x];
    //let mut visible = false;
    //
    //let mut visible_from_top = true;
    //let mut visible_from_bottom = true;
    //let mut visible_from_left = true;
    //let mut visible_from_right = true;
    //
    ////check all tiles above the current one
    //for _y in 0..y {
    //    if tree_map[_y][x] >= tree_height {
    //        visible_from_top = false;
    //    }
    //}
    //
    ////check all tiles below the current one
    //for _y in y+1..SIZE {
    //    if tree_map[_y][x] >= tree_height {
    //        visible_from_bottom = false;
    //    }
    //}
    //
    ////check all tiles to the left of the current one
    //for _x in 0..x {
    //    if tree_map[y][_x] >= tree_height {
    //        visible_from_left = false;
    //    }
    //}
    //
    ////check all tiles to the right of the current one
    //for _x in x+1..SIZE {
    //    if tree_map[y][_x] >= tree_height {
    //        visible_from_right = false;
    //    }
    //}
    //
    //if visible_from_top || visible_from_bottom || visible_from_left || visible_from_right {
    //    visible = true;
    //}
    //
    //visible
}

fn check_scenic_score(x: usize, y: usize, tree_map: &[[u32; SIZE]; SIZE]) -> usize {
    let tree_height = tree_map[y][x];

    let mut score = 1;
    for (i, v) in [x, y].iter().enumerate() {
        for range in [(0..*v).rev().collect::<Vec<_>>(), (*v+1..SIZE).collect::<Vec<_>>()] {
            let mut number_of_trees = 0;
            for _v in range {
                number_of_trees += 1;
                if tree_map[[y, _v][i]][[_v, x][i]] >= tree_height {
                    break;
                }
            }
            score *= number_of_trees;
        }
    }
    score
    //let mut number_of_trees_above = 0;
    //let mut number_of_trees_below = 0;
    //let mut number_of_trees_left = 0;
    //let mut number_of_trees_right = 0;
    //
    //for _y in (0..y).rev() {
    //    number_of_trees_above += 1;
    //    if tree_map[_y][x] >= tree_height {
    //        break;
    //    }
    //}
    //
    //for _y in y+1..SIZE {
    //    number_of_trees_below += 1;
    //    if tree_map[_y][x] >= tree_height {
    //        break;
    //    }
    //}
    //
    //for _x in (0..x).rev() {
    //    number_of_trees_left += 1;
    //    if tree_map[y][_x] >= tree_height {
    //        break;
    //    }
    //}
    //
    //for _x in x+1..SIZE {
    //    number_of_trees_right += 1;
    //    if tree_map[y][_x] >= tree_height {
    //        break;
    //    }
    //}
    //
    //(number_of_trees_above * number_of_trees_below * number_of_trees_left * number_of_trees_right)
}

pub fn part1() -> String {
    let input = include_str!("../inputs/day08.txt");
    let mut tree_map: [[u32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    for y in 0..SIZE {
        for x in 0..SIZE {
            tree_map[y][x] = input.lines().nth(y).unwrap().chars().nth(x).unwrap().to_digit(10).unwrap();
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
    let mut tree_map: [[u32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    for y in 0..SIZE {
        for x in 0..SIZE {
            tree_map[y][x] = input.lines().nth(y).unwrap().chars().nth(x).unwrap().to_digit(10).unwrap();
            print!("{}", tree_map[y][x]);
        }
        println!();
    }
    println!();
    
    
    let mut current_total = 0;
    for y in 1..SIZE-1 {
        for x in 1..SIZE-1 {
            current_total = current_total.max(check_scenic_score(x, y, &tree_map));
        }
    }
    current_total.to_string()
}
//291840
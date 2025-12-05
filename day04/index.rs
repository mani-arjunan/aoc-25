use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// part1
// fn main() -> io::Result<()> {
//     let mut result: i64 = 0;
//     // let static_input = "\
//     //         ..@@.@@@@.
//     //         @@@.@.@.@@
//     //         @@@@@.@.@@
//     //         @.@@@@..@.
//     //         @@.@@@@.@@
//     //         .@@@@@@@.@
//     //         .@.@.@.@@@
//     //         @.@@@.@@@@
//     //         .@@@@@@@@.
//     //         @.@.@@@.@.";
//     // let reader = BufReader::new(static_input.as_bytes());
//     let file = File::open("./input.txt")?;
//     let reader = BufReader::new(file);
//     let direction: [[i64; 2]; 8] = [
//         [0, 1],
//         [0, -1],
//         [1, 0],
//         [-1, 0],
//         [-1, 1],
//         [-1, -1],
//         [1, -1],
//         [1, 1]
//     ];
//     let mut grid: Vec<Vec<char>> = Vec::new();
//
//     for line_result in reader.lines() {
//         let line = line_result?;
//         let trimmed_line = line.trim();
//
//         let parts = trimmed_line.trim().chars().collect();
//
//         grid.push(parts);
//     }
//
//     for i in 0..grid.len() {
//         for j in 0..grid[0].len() {
//             if grid[i][j] == '@' {
//                 let mut count = 0;
//
//                 for [dx, dy] in direction {
//                     let new_x = i as i64 + dx;
//                     let new_y = j as i64 + dy;
//
//                     if new_x >= 0 && new_y >= 0 && new_x < (grid.len() as i64) && new_y < (grid[0].len() as i64) {
//                         let nx = new_x as usize;
//                         let ny = new_y as usize;
//
//                         if grid[nx][ny] == '@' {
//                             count += 1;
//                         }
//                     }
//                 }
//
//                 if count < 4 {
//                     result += 1;
//                 }
//             }
//         }
//     }
//
//     println!("{}", result);
//
//
//     Ok(())
// }

fn recurse(grid: Vec<Vec<char>>, mut result: i64) -> (Vec<Vec<char>>, i64) {
    let direction: [[i64; 2]; 8] = [
        [0, 1],
        [0, -1],
        [1, 0],
        [-1, 0],
        [-1, 1],
        [-1, -1],
        [1, -1],
        [1, 1]
    ];
    let row_len = grid.len();
    let col_len = grid[0].len();
    let mut new_grid = grid.clone();
    let mut local_result = 0;

    for i in 0..row_len {
        for j in 0..col_len {
            if grid[i][j] == '@' {
                let mut count = 0;

                for [dx, dy] in direction {
                    let nx = i as i64 + dx;
                    let ny = j as i64 + dy;

                    if nx >= 0 && ny >= 0 && nx < row_len as i64 && ny < col_len as i64 {
                        if grid[nx as usize][ny as usize] == '@' {
                            count += 1;
                        }
                    }
                }

                if count < 4 {
                    new_grid[i][j] = 'x';
                    local_result += 1;
                } else {
                    new_grid[i][j] = '@';
                }
            } else {
                new_grid[i][j] = '.';
            }
        }
    }

    if local_result == 0 {
        return (new_grid, result)
    }

    result += local_result;

    return recurse(new_grid, result);
}

fn main() -> io::Result<()> {
    // let static_input = "\
    //         ..@@.@@@@.
    //         @@@.@.@.@@
    //         @@@@@.@.@@
    //         @.@@@@..@.
    //         @@.@@@@.@@
    //         .@@@@@@@.@
    //         .@.@.@.@@@
    //         @.@@@.@@@@
    //         .@@@@@@@@.
    //         @.@.@@@.@.";
    // let reader = BufReader::new(static_input.as_bytes());
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed_line = line.trim();

        let parts = trimmed_line.trim().chars().collect();

        grid.push(parts);
    }

    let (_, result) = recurse(grid, 0);

    println!("{}", result);


    Ok(())
}

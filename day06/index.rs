use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// part 1
// fn main() -> io::Result<()> {
//     let mut result: i64 = 0;
// //     let static_input = "123 328  51 64 
// // 45 64  387 23 
// // 6 98  215 314
// // *   +   *   +  ";
// //     let reader = BufReader::new(static_input.as_bytes());
//     let file = File::open("./input.txt")?;
//     let reader = BufReader::new(file);
//
//     let mut input_arr: Vec<String> = Vec::new();
//    
//     for line_result in reader.lines() {
//         let line = line_result?;
//
//         input_arr.push(line);
//     }
//
//     let operation: Vec<char> = input_arr.pop().expect("").trim().chars().filter(|&c| c != ' ').collect();
//     let final_arr: Vec<Vec<i64>> = input_arr.iter().map(|line| {
//         line
//             .split_whitespace()     
//             .map(|num| num.parse::<i64>().unwrap()) 
//             .collect::<Vec<i64>>()  
//     })
//     .collect();
//
//
//     for i in 0..final_arr[0].len() {
//         let mut j = 0;
//         let mut temp = -1;
//
//         while j < final_arr.len() {
//             if operation[i] == '*' {
//                 if temp == -1 {
//                     temp = 1 * final_arr[j][i];
//                 } else {
//                     temp *= final_arr[j][i];
//                 }
//             } else {
//                 if temp == -1 {
//                     temp = final_arr[j][i];
//                 } else {
//                     temp += final_arr[j][i];
//                 }
//             }
//             j += 1;
//         }
//
//         result += temp;
//     }
//     
//
//     println!("Result -> {}", result);
//
//     Ok(())
// }


// part 2
fn main() -> io::Result<()> {
//     let static_input = "123 328  51 64 
//  45 64  387 23 
//   6 98  215 314
// *   +   *   +  ";
//     let reader = BufReader::new(static_input.as_bytes());
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|s| !s.trim().is_empty())
        .collect();
    let operation_line = lines.pop().expect("missing operation line");
    let number_lines = lines;

    let operations: Vec<char> = operation_line
        .trim()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .rev()
        .collect();

    let mut columns: Vec<Vec<i64>> = Vec::new();
    let mut temp: Vec<i64> = Vec::new();

    for i in 0..number_lines[0].len() {
        let mut s = String::new();

        for j in 0..number_lines.len() {
            let chars: Vec<char> = number_lines[j].chars().collect();
            if i < chars.len() && chars[i] != ' ' {
                s.push(chars[i]);
            }
        }

        if s.is_empty() {
            columns.push(temp);
            temp = Vec::new();
        } else {
            let num = s.parse::<i64>().unwrap_or(0);
            temp.push(num);
        }
    }


    if !temp.is_empty() {
        columns.push(temp);
    }

    columns.reverse();

    let mut result: i64 = 0;

    for i in 0..columns.len() {
        let is_multiply = operations[i] == '*';
        let mut temp = if is_multiply { 1 } else { 0 };

        for j in 0..columns[i].len() {
            if is_multiply {
                temp *= columns[i][j];
            } else {
                temp += columns[i][j];
            }
        }

        result += temp;
    }

    println!("Result -> {}", result);
    Ok(())
}

use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn largest_num_based_on_length(num_in_str: &str, len: i64) -> String {
    let mut possible_to_remove: i64 = num_in_str.len() as i64 - len;
    let mut stack: Vec<char> = Vec::new();

    for ch in num_in_str.chars() {
        while possible_to_remove > 0 && !stack.is_empty() && stack.last().unwrap() < &ch {
            stack.pop();
            possible_to_remove -= 1;
        }
        stack.push(ch);
    }

    let joined_str = stack
        .iter()
        .take(len as usize)
        .collect::<String>();

    return joined_str;
}

 // part1
// fn main() -> io::Result<()> {
//     let mut result: i64 = 0;
//     // let static_input = "
//     //     987654321111111
//     //     811111111111119
//     //     234234234234278
//     //     818181911112111
//     // ";
//     // let reader = BufReader::new(static_input.as_bytes());
//     let file = File::open("./input.txt")?;
//     let reader = BufReader::new(file);
//
//     for line_result in reader.lines() {
//         let line = line_result?;
//         let trimmed_line = line.trim();
//
//         if trimmed_line.is_empty() {
//             continue;
//         }
//
//         let temp = largest_num_based_on_length(trimmed_line, 2);
//         let conv_num: i64 = temp.parse().expect("It should be a number");
//
//         result = result + conv_num;
//     }
//     println!("start => {}", result);
//
//     Ok(())
// }


// part 2
fn main() -> io::Result<()> {
    let mut result: i64 = 0;
    // let static_input = "
    //     987654321111111
    //     811111111111119
    //     234234234234278
    //     818181911112111
    // ";
    // let reader = BufReader::new(static_input.as_bytes());
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            continue;
        }

        let temp = largest_num_based_on_length(trimmed_line, 12);
        let conv_num: i64 = temp.parse().expect("It should be a number");

        result = result + conv_num;
    }
    println!("start => {}", result);

    Ok(())
}


// part2

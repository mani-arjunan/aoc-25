use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

 // part1
// fn main() -> io::Result<()> {
//     let mut result: i64 = 0;
//     // let static_input = "3-5
//     //         10-14
//     //         16-20
//     //         12-18
//     //
//     //         1
//     //         5
//     //         8
//     //         11
//     //         17
//     //         32
//     // ";
//     // let reader = BufReader::new(static_input.as_bytes());
//     let file = File::open("./input.txt")?;
//     let reader = BufReader::new(file);
//     let mut range_arr: Vec<String> = Vec::new();
//     let mut ids_arr: Vec<String> = Vec::new();
//     let mut range_arr_ends= false;
//
//     for line_result in reader.lines() {
//         let line = line_result?;
//         let trimmed_line = line.trim();
//
//         if trimmed_line.is_empty() {
//             range_arr_ends = true;
//             continue;
//         }
//
//         if !range_arr_ends {
//             range_arr_ends.push(trimmed_line.to_string());
//         } else {
//             idsArr.push(trimmed_line.to_string());
//         }
//     }
//
//     for j in 0..idsArr.len() {
//         for i in 0..range_arr.len() {
//             let parts: Vec<&str> = range_arr[i].trim().split('-').collect();
//             if let [start, end] = &parts[..] {
//                 let parsed_start: i64 = start.parse().expect("It should be number");
//                 let parsed_end: i64 = end.parse().expect("It should be number");
//                 let id: i64 = idsArr[j].parse().expect("It should be number");
//
//                 if id <= parsedEnd && id >= parsedStart {
//                     result += 1;
//                     break;
//                 }
//             }
//         }
//     }
//     println!("ResultIds -> {}", result);
//
//     Ok(())
// }

fn main() -> io::Result<()> {
    let mut result: i64 = 0;
    // let static_input = "3-5
    //         10-14
    //         16-20
    //         12-18
    //
    //         1
    //         5
    //         8
    //         11
    //         17
    //         32
    // ";
    // let reader = BufReader::new(static_input.as_bytes());
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut range_arr: Vec<String> = Vec::new();
    let mut current_highest: i64 = 0;
   

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            break;
        }

        range_arr.push(trimmed_line.to_string());
    }


    range_arr.sort_by(|a, b| {
        let a_start = a
            .split('-')
            .next()
            .and_then(|s| s.parse::<u128>().ok())
            .unwrap_or(0);

        let b_start = b
            .split('-')
            .next()
            .and_then(|s| s.parse::<u128>().ok())
            .unwrap_or(0);

        a_start.cmp(&b_start)
    });

    for i in 0..range_arr.len() {
        let parts: Vec<&str> = range_arr[i].trim().split('-').collect();

        if let [start, end] = &parts[..] {
            let parsed_start: i64 = start.parse().expect("It should be number");
            let parsed_end: i64 = end.parse().expect("It should be number");

            if current_highest == 0 {
                current_highest = parsed_end;
                let diff = parsed_end - parsed_start;
                result = result + diff + 1;
            } else {
                if parsed_start <= current_highest {
                    if parsed_end > current_highest {
                        let diff = parsed_end - current_highest;
                        current_highest = parsed_end;
                        result = result + diff;
                    }
                } else {
                    current_highest = parsed_end;
                    let diff = parsed_end - parsed_start;
                    result = result + diff + 1;
                }
            }
        }
    }
    println!("ResultIds -> {}", result);

    Ok(())
}

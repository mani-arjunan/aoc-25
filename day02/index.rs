use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

 // part1
// fn main() -> io::Result<()> {
//     let mut result = 0;
//     let staticInput = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
//     1698522-1698528,446443-446449,38593856-38593862,565653-565659,
//     824824821-824824827,2121212118-2121212124";
//     // let reader = BufReader::new(staticInput.as_bytes());
//     let file = File::open("./input.txt")?;
//     let reader = BufReader::new(file);
//
//     for line_result in reader.lines() {
//         let line = line_result?;
//         for pair in line.trim().split(',') {
//             let arr: Vec<&str> = pair.trim().split('-').collect();
//             if arr.len() > 1 {
//                 let start: i64 = arr[0].parse().expect("It should be a number");
//                 let end: i64 = arr[1].parse().expect("It should be a number");
//
//                 for i in start..=end {
//                     let str = i.to_string();
//                     let len: usize = str.len();
//
//                     if len % 2 == 0 {
//                         let temp = len / 2;
//                         let first_part = &str[0..temp];
//                         let second_part = &str[temp..len];
//
//                         if first_part == second_part {
//                             println!("{}", i);
//                             result = result + i;
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     println!("start => {}", result);
//
//     Ok(())
// }

// part2
fn form_repeated_string(sub_string: &str, repeat_count: i64) -> String {
    let mut result = String::from("");

    for _i in 0..repeat_count {
        result = result + sub_string;
    }

    return result
}

fn check_if_all_true(num_in_str: &str) -> bool {
    let first = &num_in_str[0..1];

    for i in 0..num_in_str.len() {
        let char = &num_in_str[i..i+1];
        if char != first {
            return false;
        }
    }

    return true;
}

fn check_is_invalid(num_in_str: &str, len: usize) -> bool {
    if num_in_str.len() > 1 && check_if_all_true(num_in_str) {
        return true
    }
    let half = len / 2;

    for i in 2..half {
        if len % i == 0 {
            let sub_string = &num_in_str[0..i];
            let repeat_count: i64 = ((len as f64) / (i as f64)) as i64;
            let repeated = form_repeated_string(sub_string, repeat_count);

            if repeated == num_in_str {
                return true;
            }
        }
    }
    return false;
}

fn main() -> io::Result<()> {
    let mut result = 0;
    // let staticInput = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    // 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    // 824824821-824824827,2121212118-2121212124";
    // let reader = BufReader::new(staticInput.as_bytes());
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        for pair in line.trim().split(',') {
            let arr: Vec<&str> = pair.trim().split('-').collect();
            if arr.len() > 1 {
                let start: i64 = arr[0].parse().expect("It should be a number");
                let end: i64 = arr[1].parse().expect("It should be a number");

                for i in start..=end {
                    let str = i.to_string();
                    let len: usize = str.len();

                    if len % 2 == 0 {
                        let temp = len / 2;
                        let first_part = &str[0..temp];
                        let second_part = &str[temp..len];

                        if first_part == second_part {
                            result = result + i;
                        } else {
                            let temp = check_is_invalid(&i.to_string(), len);

                            if temp {
                                result = result + i;
                            }
                        }
                    } else {
                        let temp = check_is_invalid(&i.to_string(), len);

                        if temp {
                            result = result + i;
                        }
                    }
                }
            }
        }
    }


    println!("result => {}", result);

    Ok(())
}

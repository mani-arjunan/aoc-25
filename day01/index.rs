use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// // part1
// fn main() -> io::Result<()> {
//     let max = 99 + 1;
//     let mut current: i32= 50;
//     let mut result = 0;
//     // let staticInput = "L68
//     //     L30
//     //     R48
//     //     L5
//     //     R60
//     //     L55
//     //     L1
//     //     L99
//     //     R14
//     //     L82";
//     // let reader = BufReader::new(staticInput.as_bytes());
//     let file = File::open("./input.txt")?;
//     let reader = BufReader::new(file);
//
//     for line_result in reader.lines() {
//         let line = line_result?;
//         let (direction, rot) = line.trim().split_at(1);
//         let rotation: i32 = rot.parse().expect("It should be a number");
//
//         if direction == "L" {
//             current = ((current - rotation) % max + max) % max;
//         } else  {
//             current = (current + rotation) % max;
//         }
//
//         if current == 0 {
//             result += 1;
//         }
//     }
//     println!("{}", result);
//
//     Ok(())
// }

// part2
fn main() -> io::Result<()> {
    let max: i32 = 99 + 1;
    let mut current: i32 = 50;
    let mut result = 0;
    // let staticInput = "L68
    //     L30
    //     R48
    //     L5
    //     R60
    //     L55
    //     L1
    //     L99
    //     R14
    //     L82";
    // let reader = BufReader::new(staticInput.as_bytes());
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        let (direction, rot) = line.trim().split_at(1);
        let rotation: i32 = rot.parse().expect("It should be a number");

        let mut first_rotation: i32;

        if direction == "L" {
            first_rotation = current;
        } else {
            first_rotation = max - current;
        }

        if first_rotation == 0 {
            first_rotation = max;
        }

        let final_rotation: f32 = (rotation - first_rotation) as f32 / max as f32;
        result += final_rotation.ceil() as i32;

        if direction == "L" {
            current = ((current - rotation) % max + max) % max;
        } else  {
            current = (current + rotation) % max;
        }

        if current == 0 {
            result += 1;
        }
    }
    println!("{}", result);

    return Ok(());
}

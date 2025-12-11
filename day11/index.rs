use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;


fn recurse(
    current: String,
    map: &HashMap<String, Vec<String>>,
    mut dac: bool,
    mut fft: bool,
    seen: &mut HashMap<String, i64>
) -> i64 {
    if current == "out" {
        // part 2
        if dac && fft {
            return 1;
        }

        return 0;
        // part1
        // return 1;
    }
    let key = format!("{}-{}-{}", current, dac, fft);
    let mut total = 0;

    if seen.contains_key(&key) {
        return *seen.get(&key).expect("Key must exist");
    }

    if current == "dac" {
        dac = true;
    }

    if current == "fft" {
        fft = true;
    }

    for i in 0..map[&current].len() {
        total += recurse(map[&current][i].clone(), map, dac, fft, seen);
    }

    seen.insert(key, total);

    return total;
}

// part 1
// fn main() -> io::Result<()> {
// //     let static_input = "aaa: you hhh
// // you: bbb ccc
// // bbb: ddd eee
// // ccc: ddd eee fff
// // ddd: ggg
// // eee: out
// // fff: out
// // ggg: out
// // hhh: ccc fff iii
// // iii: out";
// //     let reader = BufReader::new(static_input.as_bytes());
//     let file = File::open("./input.txt")?;
//     let reader = BufReader::new(file);
//
//     let mut input_map: HashMap<String, Vec<String>> = HashMap::new();
//    
//     for line_result in reader.lines() {
//         let line = line_result?;
//         
//         if let Some((key, values)) = line.split_once(':') {
//             let key = key.trim().to_string();
//             let values: Vec<String> = values
//                 .split_whitespace()
//                 .map(|s| s.trim().to_string())
//                 .collect();
//
//             input_map.insert(key, values);
//         }
//     }
//     let s = String::from("you");
//     let mut seen: HashMap<String, i64> = HashMap::new();
//
//     let total = recurse(s, &input_map, false, false, &mut seen);
//
//     println!("{}", total);
//
//
//     Ok(())
// }

// part2
fn main() -> io::Result<()> {
//        let static_input = "svr: aaa bbb
// aaa: fft
// fft: ccc
// bbb: tty
// tty: ccc
// ccc: ddd eee
// ddd: hub
// hub: fff
// eee: dac
// dac: fff
// fff: ggg hhh
// ggg: out
// hhh: out";
//     let reader = BufReader::new(static_input.as_bytes());
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut input_map: HashMap<String, Vec<String>> = HashMap::new();
   
    for line_result in reader.lines() {
        let line = line_result?;
        
        if let Some((key, values)) = line.split_once(':') {
            let key = key.trim().to_string();
            let values: Vec<String> = values
                .split_whitespace()
                .map(|s| s.trim().to_string())
                .collect();

            input_map.insert(key, values);
        }
    }
    let s = String::from("svr");
    let mut seen: HashMap<String, i64> = HashMap::new();

    let total = recurse(s, &input_map, false, false, &mut seen);

    println!("{}", total);


    Ok(())
}


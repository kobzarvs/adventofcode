use day_17::{part1, part2, read_file};

fn main() {
    let input = read_file();

    let result = part1::run(&input);
    println!("part I: {}", result);

    let result = part2::run(&input);
    println!("part II: {}", result);
}

// #![allow(unused_imports)]
// #![allow(unused_variables)]
// 
// use std::collections::VecDeque;
// use std::io::ErrorKind;
// use std::ops::{BitAnd, BitXor};
// use day_17::{parse, part2, Computer};
// 
// fn main() -> Result<(), ErrorKind>{
//     // let input = include_str!("../data.txt");
//     // let mut computer = parse(input);
//     
//     // match computer.find_correct_a() {
//     //     Some(a) => println!("Найдено правильное значение A: {}", a),
//     //     None => println!("Правильное значение A не найдено")
//     // }
// 
//     // let result = part2::run(&input)?;
//     // println!("part II: {}", result);
// 
//     // - 2, 4 -> bst A -> B = A & 7      ; 3 bits from A
//     // - 1, 5 -> bxl 1 -> B = B ~ '101   ; switch 1st bit
//     // - 7, 5 -> cdv B -> C = A >> B     ; A >> 0..7 bits
//     // - 4, 3 -> bxc   -> B = B ~ C      ; B xor C
//     // - 1, 6 -> bxl B -> B = B ~ '110   ; B xor '110
//     // - 0, 3 -> adv 3 -> A = A >> 3     ; next 3 bits from A
//     // - 5, 5 -> out B -> output B & 0x7 ; result = B
// 
//     let mut out = vec![2,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0];
//     out.reverse();
// 
//     let mut b: i64;
//     let mut c: i64;
//     let mut q: VecDeque<(i64, usize)> = VecDeque::new();
//     q.push_front((0, 0));
// 
//     let out_len = out.len();
//     'start: while let Some((a, step)) = q.pop_front() {
//         let mut tmp_a: i64;
//         for i in 0..8 {
//             tmp_a = a | i;
//             b = tmp_a & 7;
//             b = b ^ 0b101;
//             c = tmp_a >> b;
//             b = b ^ c;
//             b = b ^ 0b110;
//             let out_b = b % 8;
//             if out_b == out[step] {
//                 if step == out.len() - 1 {
//                     println!("FINISH: a = {}, b = {}, c = {}", tmp_a, b, c);
//                     break 'start;
//                 }
//                 q.push_back((tmp_a << 3, step + 1));
//             }
//         }
//     }
// 
//     println!();
// 
//     Ok(())
// }

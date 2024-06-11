struct Solution;

// impl Solution {
//     pub fn xor_operation(n: i32, start: i32) -> i32 {
//         (0..n)
//             .fold(0, |result, index| result ^ (start + 2 * index))
//     }
// }
// impl Solution {
//     pub fn xor_operation(n: i32, start: i32) -> i32 {
//         (0..n)
//             .map(|i| start + i * 2)
//             .fold(0, std::ops::BitXor::bitxor)
//     }
// }
impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (start..)
            .step_by(2)
            .take(n as usize)
            .fold(0, std::ops::BitXor::bitxor)
    }
}
// impl Solution {
//     pub fn xor_operation(n: i32, start: i32) -> i32 {
//         (start..)
//             .step_by(2)
//             .take(n as usize)
//             .fold(0, |i, j| i ^ j)
//     }
// }

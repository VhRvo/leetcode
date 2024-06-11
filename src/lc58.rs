struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.into_bytes()
            .iter()
            .rev()
            .skip_while(|&&ch| ch == b' ')
            .take_while(|&&ch| ch != b' ')
            .count() as i32
        // s.trim()
        //     .split(" ")
        //     .collect::<Vec<_>>()
        //     .last()
        //     .unwrap()
        //     .len() as i32
    }
}

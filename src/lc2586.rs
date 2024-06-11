struct Solution;
impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let vowels = [b'a', b'e', b'i', b'o', b'u'];
        words
            .iter()
            .skip(left as usize)
            .take((right + 1 - left) as usize)
            .filter(|&word| {
                let u8_chars = word.as_bytes();
                vowels.contains(u8_chars.first().unwrap())
                    && vowels.contains(u8_chars.last().unwrap())
            })
            .count() as i32
    }
}

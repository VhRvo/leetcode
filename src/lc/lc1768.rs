struct Solution;

impl Solution {
    // pub fn merge_alternately(word1: String, word2: String) -> String {
    //     let mut result = String::with_capacity(word1.len() + word2.len());
    //     let mut word1_iter = word1.chars().peekable();
    //     let mut word2_iter = word2.chars();
    //     for (ch1, ch2) in word1_iter.clone().zip(word2_iter.clone()) {
    //         result.push(ch1);
    //         result.push(ch2);
    //     }
    //     let min_length = word1.len().min(word2.len());
    //     for ch in word1_iter.skip(min_length) {
    //         result.push(ch);
    //     }
    //     for ch in word2_iter.skip(min_length) {
    //         result.push(ch);
    //     }
    //     result
    // }
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::with_capacity(word1.len() + word2.len());
        let mut word1_iter = word1.chars().peekable();
        let mut word2_iter = word2.chars().peekable();
        while word1_iter.peek().is_some() && word2_iter.peek().is_some() {
            result.push(word1_iter.next().unwrap());
            result.push(word2_iter.next().unwrap());
        }
        if word1_iter.peek().is_some() {
            for ch in word1_iter {
                result.push(ch);
            }
        }
        if word2_iter.peek().is_some() {
            for ch in word2_iter {
                result.push(ch);
            }
        }
        result
    }
}

#[test]
fn test() {}

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut record: HashMap<_, Vec<String>> = HashMap::new();
        for str in strs {
            let mut sorted = str.clone().into_bytes();
            sorted.sort();
            record.entry(sorted).or_default().push(str);
        }
        record.into_values().collect()
    }
}

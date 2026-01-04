struct Solution;

use std::collections::HashMap;
impl Solution {
    fn dfs_outer(
        answer_map: &mut HashMap<Vec<char>, bool>,
        pattern_map: &HashMap<(char, char), Vec<char>>,
        bottom: &Vec<char>,
    ) -> bool {
        if bottom.len() == 1 {
            return true;
        }
        // memoization
        match answer_map.get(bottom) {
            Some(answer) => *answer,
            None => {
                let mut new_bottom = vec![];
                let zipper: Vec<(char, char)> = bottom
                    .iter()
                    .cloned()
                    .zip(bottom.iter().cloned().skip(1))
                    .collect();
                let result = Self::dfs_inner(answer_map, pattern_map, &mut new_bottom, &zipper);
                answer_map.insert(bottom.clone(), result);
                result
            }
        }
    }

    fn dfs_inner(
        answer_map: &mut HashMap<Vec<char>, bool>,
        pattern_map: &HashMap<(char, char), Vec<char>>,
        bottom: &mut Vec<char>,
        zipper: &Vec<(char, char)>,
    ) -> bool {
        // remove more impossible cases
        if let Some(false) = answer_map.get(bottom) {
            return false;
        }
        if bottom.len() == zipper.len() {
            return Self::dfs_outer(answer_map, pattern_map, bottom);
        }

        let pattern = zipper[bottom.len()];
        match pattern_map.get(&pattern) {
            Some(blocks) => {
                for block in blocks {
                    bottom.push(*block);
                    let result = Self::dfs_inner(answer_map, pattern_map, bottom, &zipper);
                    bottom.pop();
                    if result {
                        return true;
                    }
                }
                return false;
            }
            None => {
                return false;
            }
        }
    }

    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let pattern_map = {
            let mut map = HashMap::new();
            for s in allowed {
                let chars = s.chars().collect::<Vec<char>>();
                let key: (char, char) = (chars[0], chars[1]);
                map.entry(key).or_insert_with(|| vec![]).push(chars[2]);
            }
            map
        };
        let bottom = bottom.chars().collect::<Vec<char>>();
        let mut answer_map = HashMap::new();
        Self::dfs_outer(&mut answer_map, &pattern_map, &bottom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::string::ToString;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::pyramid_transition(
                "BCD".to_string(),
                ["BCC", "CDE", "CEA", "FFF"].map(|s| s.to_string()).to_vec() // ["BCC", "CDE", "CEA", "FFF"].map(to_string).to_vec()
            ),
            true
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::pyramid_transition(
                "AAAA".to_string(),
                ["AAB", "AAC", "BCD", "BBE", "DEF"]
                    .map(|s| s.to_string())
                    .to_vec()
            ),
            false
        )
    }
    #[test]
    fn test3() {
        assert_eq!(
            {
                let start = std::time::Instant::now();
                let result = Solution::pyramid_transition(
                    "ABBBBBBG".to_string(),
                    [
                        "BDD", "BFB", "FDA", "CCC", "FBD", "FBG", "EEB", "GBA", "CFB", "ECB",
                        "GDA", "ECC", "EFE", "FCA", "DBA", "FAD", "BEE", "GEC", "CFC", "CFD",
                        "BCG", "CAC", "FCC", "GAC", "CAG", "CFF", "BEC", "EAD", "GBE", "FEG",
                        "BDC", "GBF", "FBB", "FBE", "DDG", "FDF", "EDF", "FAF", "BEB", "DDE",
                        "EEA", "FAG", "GDC", "DFE", "CDB", "DFD", "GDD", "DAF", "ECE", "DBG",
                        "DDD", "EFG", "FAC", "FEC", "GED", "DEE", "CCF", "EFD", "FDE", "GDF",
                        "ECG", "DAE", "FCG", "DCA", "EAF", "EED", "EEE", "FEA", "BFG", "EAA",
                        "GDE", "CEB", "FAE", "BDE", "EBB", "GFD", "DBB", "BFD", "EFF", "DAD",
                        "CDC", "DCD", "FFD", "DBD", "EFA", "BCB", "ECF", "CFA", "GFF", "CFG",
                        "EEG", "DAG", "CAE", "FED", "CCA", "EDE", "FDG", "BEF", "FDD", "BEG",
                        "FFE", "DDC", "BCE", "GAA", "GFE", "FFF", "CFE", "BDF", "DFG", "DEF",
                        "FEF", "GEE", "EAB", "GDB", "CEE", "BDA", "DEG", "DEC", "GCG", "DFB",
                        "EFB", "GAB", "CDF", "CAF", "BAB", "GCC", "DDA", "CDD", "FCB", "DAA",
                        "GBG", "BAC", "GCE", "GAG", "EAE", "EBG", "GBD", "BFE", "CCD", "EDD",
                        "FCD", "FBC", "DEB", "EDG", "GEA", "EBE", "GDG", "DBE", "BCD", "GBB",
                        "FEB", "DED", "CAD", "EDB", "CED", "GCD", "BFC", "GAE", "CEG", "EEC",
                        "FEE", "GCA", "FCF", "ECD", "DCC", "BDB", "EBD", "DCE", "BAE", "FFG",
                        "DDF", "DCF", "BED", "CAA", "BCF", "GBC", "CCB", "BCC", "GFC", "FFB",
                        "DFA", "BBA", "BBB", "BBC", "BBD", "BBE", "BBF", "BBG", "AAA", "ABA",
                        "ACA", "ADA", "AEA", "AFA", "BGG", "CGG", "DGG", "EGG", "FGG", "GGG",
                    ]
                    .map(|s| s.to_string())
                    .to_vec(),
                );
                let elapsed = start.elapsed();
                println!("Time elapsed: {:?}", elapsed);
                result
            },
            false
        )
    }
}

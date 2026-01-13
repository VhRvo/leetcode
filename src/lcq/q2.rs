struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
      todo!()
    }
    fn shuffle_inplace(nums: Vec<i32>, n: i32) -> Vec<i32> {
      /*
       a1b2c3d4 e5f6g7h8
       abcdefgh 12345678
      skip
        bcdefgh 1234567
      cross and swap
        bcdefg7 123456h
        1cdefg7 b23456h
         cdefgb 723456
      head and tail swap
         bdefgc 623457
          defgc 62345
          defgc 62345
       */
      /*
        abcdefgh 12345678
      target
        a1b2c3d4 e5f6g7h8
      skip
         bcdefgh 1234567
      cross
         1cdefgh b234567
         1cdefg7 b23456h
         1cdefg7 b23456h
      clear
          cdefg7 b23456
      cross 1
          bdefg7 c23456
          bdefg6 c23457
      target
        a1b2c3d4 e5f6g7h8
      clear
           defg6 c2345
      cross 2
           2efg6 cd345
           2ef56 cd346
      target
        a1b2c3d4 e5f6g7h8
      clear
            ef56 cd34
      cross 1
            cf56 ed34
            cf54 ed36
      clear
             f54 ed3
      cross 3
             354 edf
      clear
              54 ed
      cross 2
              d4 e5
      clear
               4 e
      // cross 1
      //          e 4

       */
      let n = n as usize;
      let mut part1 = (0, n);
      let mut part2 = (n, 2 * n);
      for ii in 0..n {


      }

      todo!()
    }
    fn shuffle_push(nums: Vec<i32>, n: i32) -> Vec<i32> {
        /*
        abcdefghij 0123456789
        a0cdefghij b123456789
          bdefghij c123456789
          bdefghij c123456789
          b1efghij cd23456789
            cfghij ed23456789
            c2ghij edf3456789
              dhij egf3456789
              d3ij egfh456789
                ej igfh456789
                e4 igfhj56789
                   fgihj56789
                   f5ihjg6789
                     ghji6789
                     g6jih789
                       hij789
         */
        let mut result = Vec::with_capacity(nums.len());
        let n = n as usize;
        for ii in 0..n {
            result.push(nums[ii]);
            result.push(nums[ii + n]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3),
            vec![2, 3, 5, 4, 1, 7]
        )
    }
}

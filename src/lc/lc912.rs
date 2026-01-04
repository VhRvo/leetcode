struct Solution;
impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        quick_sort(nums.as_mut_slice());
        nums
    }
}

fn quick_sort(items: &mut [i32]) {
    if items.len() > 2 {
        let mut pivot = 0;
        let mut lo = 1;
        let mut hi = items.len() - 1;
        while lo <= hi {
            println!("{:?}", items);
            while lo < hi && items[lo] < items[pivot] {
                lo += 1;
            }
            while lo < hi && items[hi] > items[pivot] {
                hi -= 1;
            }
            if lo < hi {
                items.swap(lo, hi);
                items.swap(pivot, lo);
                pivot = lo;
                lo += 1;
                hi -= 1;
            } else if lo == hi && items[lo] < items[pivot] {
                items.swap(lo, pivot);
                pivot = lo;
            }
        }
        println!("{:?}", items);
        let (front, rest) = items.split_at_mut(pivot);
        let (_, rest) = rest.split_at_mut(1);
        quick_sort(front);
        quick_sort(rest);
    } else if items.len() == 2 && items[0] > items[1] {
        items.swap(0, 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut items = [5, 2, 4, 3, 1];
        quick_sort(&mut items);
        assert_eq!(items, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test2() {
        let mut items = [4, 5, 2, 3, 1];
        quick_sort(&mut items);
        assert_eq!(items, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test3() {
        let mut items = [11, 14, 4, 2];
        quick_sort(&mut items);
        assert_eq!(items, [2, 4, 11, 14]);
    }

    #[test]
    fn test4() {
        let mut items = [8, 6, 10, 7, 9, 5, 2, 4, 3, 1];
        quick_sort(&mut items);
        assert_eq!(items, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}

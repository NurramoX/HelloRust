

fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut acc = 0;
    nums.into_iter().map( |a| {
        acc += a;
        acc
    }).collect()
}

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut acc = 0;
    let vec: Vec<_> = nums.into_iter().map( |a| {
        acc += a;
        acc
    }).collect();
    let mut i = 0;
    for (val, vacc) in nums.iter().zip(vec) {
        if (vacc-val) == (acc-val)/2f32 {
            return i;
        }
        i += 1;
    }
    -1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let nums = vec![];
        let expected = vec![];
        assert_eq!(running_sum(nums), expected);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![5];
        let expected = vec![5];
        assert_eq!(running_sum(nums), expected);
    }

    #[test]
    fn test_two_elements() {
        let nums = vec![3, 2];
        let expected = vec![3, 5];
        assert_eq!(running_sum(nums), expected);
    }

    #[test]
    fn test_multiple_elements() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![1, 3, 6, 10];
        assert_eq!(running_sum(nums), expected);
    }

    #[test]
    fn test_negative_elements() {
        let nums = vec![-1, -2, -3, -4];
        let expected = vec![-1, -3, -6, -10];
        assert_eq!(running_sum(nums), expected);
    }

    #[test]
    fn test_mixed_positive_negative_elements() {
        let nums = vec![1, -2, 3, -4, 5];
        let expected = vec![1, -1, 2, -2, 3];
        assert_eq!(running_sum(nums), expected);
    }

    #[test]
    fn test_zero_elements() {
        let nums = vec![0, 0, 0];
        let expected = vec![0, 0, 0];
        assert_eq!(running_sum(nums), expected);
    }

    #[test]
    fn test_large_numbers() {
        let nums = vec![100000, 200000, 300000];
        let expected = vec![100000, 300000, 600000];
        assert_eq!(running_sum(nums), expected);
    }
}

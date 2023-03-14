pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    (0..nums.len()).fold(vec![], |mut result, idx| {
        let max_idx = idx;
        let max_number = nums[idx];
        let r = (0..nums.len()).fold(0, |mut acc, jdx| {
            if max_number > nums[jdx] && max_idx != jdx {
                acc += 1;
            }
            acc
        });
        result.push(r);
        result
    })
}

#[test]
fn test_smaller_numbers_than_current() {
    assert_eq!(
        smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
        vec![4, 0, 1, 1, 3]
    );
    assert_eq!(
        smaller_numbers_than_current(vec![6, 5, 4, 8]),
        vec![2, 1, 0, 3]
    );
    assert_eq!(
        smaller_numbers_than_current(vec![7, 7, 7, 7]),
        vec![0, 0, 0, 0]
    );
}

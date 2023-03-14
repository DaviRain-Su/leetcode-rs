pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    (1..=nums.len()).fold(vec![], |mut acc, idx| {
        acc.push(Vec::from(&nums[0..idx]).iter().sum());
        acc
    })
}

#[test]
fn test_running_sum() {
    assert_eq!(vec![1, 3, 6, 10], running_sum(vec![1, 2, 3, 4]));
    assert_eq!(vec![1, 2, 3, 4, 5], running_sum(vec![1, 1, 1, 1, 1]));
    assert_eq!(vec![3, 4, 6, 16, 17], running_sum(vec![3, 1, 2, 10, 1]));
}

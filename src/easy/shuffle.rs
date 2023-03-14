pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    (0..(n as usize)).fold(vec![], |mut acc, idx| {
        acc.push(nums[idx]);
        acc.push(nums[idx + n as usize]);
        acc
    })
}

#[test]
fn test_shuffle() {
    assert_eq!(vec![2, 3, 5, 4, 1, 7], shuffle(vec![2, 5, 1, 3, 4, 7], 3));
    assert_eq!(
        vec![1, 4, 2, 3, 3, 2, 4, 1],
        shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4)
    );
    assert_eq!(vec![1, 2, 1, 2], shuffle(vec![1, 1, 2, 2], 2));
}

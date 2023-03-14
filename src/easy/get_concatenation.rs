pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut nums_clone = nums.clone();
    let mut nums = nums;
    nums.append(&mut nums_clone);
    nums
}

#[test]
fn test_get_concatenation() {
    assert_eq!(vec![1, 2, 1, 1, 2, 1], get_concatenation(vec![1, 2, 1]));
    assert_eq!(
        vec![1, 3, 2, 1, 1, 3, 2, 1],
        get_concatenation(vec![1, 3, 2, 1])
    );
}

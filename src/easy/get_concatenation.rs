pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(2 * nums.len());
    result.extend(&nums);
    result.extend(&nums);
    result
}

#[test]
fn test_get_concatenation() {
    assert_eq!(vec![1, 2, 1, 1, 2, 1], get_concatenation(vec![1, 2, 1]));
    assert_eq!(
        vec![1, 3, 2, 1, 1, 3, 2, 1],
        get_concatenation(vec![1, 3, 2, 1])
    );
}

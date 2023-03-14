pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().map(|v| nums[*v as usize]).collect()
}

#[test]
fn test_build_array() {
    assert_eq!(vec![0, 1, 2, 4, 5, 3], build_array(vec![0, 2, 1, 5, 3, 4]));
    assert_eq!(vec![4, 5, 0, 1, 2, 3], build_array(vec![5, 0, 1, 2, 3, 4]));
}

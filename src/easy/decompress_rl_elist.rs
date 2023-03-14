pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    (0..nums.len()).step_by(2).fold(vec![], |mut result, idx| {
        result.append(&mut vec![nums[idx + 1]; nums[idx] as usize]);
        result
    })
}

#[test]
fn test_decompress_rl_elist() {
    assert_eq!(decompress_rl_elist(vec![1, 2, 3, 4]), vec![2, 4, 4, 4]);
    assert_eq!(decompress_rl_elist(vec![1, 1, 2, 3]), vec![1, 3, 3]);
}

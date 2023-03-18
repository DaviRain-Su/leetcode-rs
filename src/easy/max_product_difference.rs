pub fn max_product_difference(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    (nums[nums.len() - 1] * nums[nums.len() - 2]) - (nums[0] * nums[1])
}

#[test]
fn tes_max_product_difference() {
    assert_eq!(max_product_difference(vec![5, 6, 2, 7, 4]), 34);
    assert_eq!(max_product_difference(vec![4, 2, 5, 9, 7, 4, 8]), 64);
}

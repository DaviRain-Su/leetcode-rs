pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut cnt = 0;
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if (nums[i] - nums[j]).abs() == k {
                cnt += 1;
            }
        }
    }

    cnt
}

#[test]
fn test_count_k_difference() {
    assert_eq!(count_k_difference(vec![1, 2, 2, 1], 1), 4);
    assert_eq!(count_k_difference(vec![1, 3], 3), 0);
}

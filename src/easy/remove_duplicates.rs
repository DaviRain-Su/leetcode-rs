pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let count = (1..nums.len()).fold(0, |mut cnt, idx| {
        if nums[idx] == nums[idx - 1] {
            cnt += 1;
        } else {
            nums[idx - cnt] = nums[idx];
        }
        cnt
    });

    (nums.len() - count) as i32
}

#[test]
fn test_remove_duplicates() {
    let mut temp_vec = vec![1, 1, 2];
    assert_eq!(2, remove_duplicates(&mut temp_vec));
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let count = (0..nums.len()).fold(0, |mut cnt, idx| {
        if val == nums[idx] {
            cnt += 1;
        } else {
            nums[idx - cnt] = nums[idx];
        }
        cnt
    });

    (nums.len() - count) as i32
}

#[test]
fn test_remove_element() {
    let mut temp = vec![3, 2, 2, 3];
    assert_eq!(2, remove_element(&mut temp, 3));
    // remove_element(&mut temp, 3);
    println!("temp is {temp:?}");
    let mut temp = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(5, remove_element(&mut temp, 2));
    // remove_element(&mut temp, 2);
    println!("temp is {temp:?}");
}

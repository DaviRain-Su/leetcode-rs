// usage chatgpt
pub fn can_be_increasing(nums: Vec<i32>) -> bool {
    let mut removed = false;
    let mut prev = nums[0];

    for i in 1..nums.len() {
        if nums[i] <= prev {
            if removed {
                return false;
            }
            removed = true;
            if i == 1 || nums[i] > nums[i - 2] {
                prev = nums[i];
            } else {
                prev = nums[i - 1];
            }
        } else {
            prev = nums[i];
        }
    }

    true
}

#[test]
fn test_can_be_increasing() {
    assert!(can_be_increasing(vec![1, 2, 10, 5, 7]));
    assert!(!can_be_increasing(vec![2, 3, 1, 2]));
    assert!(!can_be_increasing(vec![1, 1, 1]));
    assert!(can_be_increasing(vec![100, 21, 100]));
}

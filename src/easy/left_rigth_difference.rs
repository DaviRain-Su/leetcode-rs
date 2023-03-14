pub fn left_rigth_difference(nums: Vec<i32>) -> Vec<i32> {
    let left_sum = if nums.is_empty() {
        vec![0]
    } else {
        (0..nums.len()).fold(vec![], |mut acc, idx| {
            acc.push(Vec::from(&nums[0..idx]).iter().sum());
            acc
        })
    };

    let nums = nums.into_iter().rev().collect::<Vec<i32>>();
    let right_sum = if nums.is_empty() {
        vec![0]
    } else {
        (0..nums.len())
            .fold(vec![], |mut acc, idx| {
                acc.push(Vec::from(&nums[0..idx]).iter().sum());
                acc
            })
            .into_iter()
            .rev()
            .collect::<Vec<i32>>()
    };

    (0..nums.len()).fold(vec![], |mut acc, idx| {
        acc.push((left_sum[idx] - right_sum[idx]).abs());
        acc
    })
}

#[test]
fn test_left_rigth_difference() {
    assert_eq!(
        vec![15, 1, 11, 22],
        left_rigth_difference(vec![10, 4, 8, 3])
    );
    assert_eq!(vec![0], left_rigth_difference(vec![1]));
}

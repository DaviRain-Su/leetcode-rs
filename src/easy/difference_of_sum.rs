pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
    let element_sum = nums.iter().sum::<i32>();

    let digit_sum = nums
        .iter()
        .map(|num| {
            num.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap_or_default() as i32)
                .sum::<i32>()
        })
        .sum::<i32>();

    (element_sum - digit_sum).abs()
}

#[test]
fn test_difference_of_sum() {
    assert_eq!(difference_of_sum(vec![1, 15, 6, 3]), 9);
    assert_eq!(difference_of_sum(vec![1, 2, 3, 4]), 0);
}

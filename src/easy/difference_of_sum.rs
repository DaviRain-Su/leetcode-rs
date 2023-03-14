pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
    let element_sum = nums.iter().sum::<i32>();
    let digit_sum = nums.iter().fold(0, |mut acc, num| {
        let inn_sum = format!("{num}").chars().fold(0, |mut acc, v| {
            acc += format!("{v}").parse::<i32>().expect("never failed");
            acc
        });
        acc += inn_sum;
        acc
    });

    (element_sum - digit_sum).abs()
}

#[test]
fn test_difference_of_sum() {
    assert_eq!(difference_of_sum(vec![1, 15, 6, 3]), 9);
    assert_eq!(difference_of_sum(vec![1, 2, 3, 4]), 0);
}

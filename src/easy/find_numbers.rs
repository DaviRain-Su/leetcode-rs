pub fn find_numbers(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |acc, num| {
        if num.to_string().len() % 2 == 0 {
            acc + 1
        } else {
            acc
        }
    })
}

#[test]
fn test_find_numbers() {
    assert_eq!(find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    assert_eq!(find_numbers(vec![555, 901, 482, 1771]), 1);
}

pub fn find_gcd(nums: Vec<i32>) -> i32 {
    let max_num = nums.iter().max().cloned().unwrap_or_default();
    let min_num = nums.iter().min().cloned().unwrap_or_default();
    let mut div_value = min_num;
    while max_num % div_value != 0 || min_num % div_value != 0 {
        div_value -= 1;
    }
    div_value
}

#[test]
fn test_find_gcd() {
    assert_eq!(find_gcd(vec![2, 5, 6, 9, 10]), 2);
    assert_eq!(find_gcd(vec![7, 5, 6, 8, 3]), 1);
}

pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    // let arr_len = if arr.len() % 2 == 0 { arr.len() - 1} else { arr.len() };
    // (1..=arr_len).step_by(2).fold(0, |mut cnt, idx| {

    // })
    todo!()
}

#[test]
#[ignore = "have not implement"]
fn test_sum_odd_length_subarrays() {
    assert_eq!(sum_odd_length_subarrays(vec![1, 2]), 3);
    assert_eq!(sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
}

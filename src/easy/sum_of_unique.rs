pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    nums.into_iter()
        .fold(HashMap::new(), |mut map, num| {
            if let Some(v) = map.get_mut(&num) {
                *v += 1;
            } else {
                map.insert(num, 1);
            }
            map
        })
        .into_iter()
        .fold(0, |sum, (k, v)| if v == 1 { sum + k } else { sum })
}

#[test]
fn test_sum_of_unique() {
    assert_eq!(sum_of_unique(vec![1, 2, 3, 2]), 4);
    assert_eq!(sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
    assert_eq!(sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
}

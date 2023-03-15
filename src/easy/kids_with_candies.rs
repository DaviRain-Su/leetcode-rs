pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max_candies = *candies.iter().max().unwrap();
    candies
        .into_iter()
        .map(|candy| candy + extra_candies >= max_candies)
        .collect()
}

#[test]
fn test_kid_with_candies() {
    assert_eq!(
        vec![true, true, true, false, true],
        kids_with_candies(vec![2, 3, 5, 1, 3], 3)
    );
    assert_eq!(
        vec![true, false, false, false, false],
        kids_with_candies(vec![4, 2, 1, 1, 2], 1)
    );

    assert_eq!(
        vec![true, false, true],
        kids_with_candies(vec![12, 1, 12], 10)
    );
}

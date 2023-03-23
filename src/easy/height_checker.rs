pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut height_clone = heights.clone();
    height_clone.sort();
    heights.into_iter().zip(height_clone.into_iter()).fold(
        0,
        |cnt, (v1, v2)| {
            if v1 != v2 {
                cnt + 1
            } else {
                cnt
            }
        },
    )
}

#[test]
fn test_height_checker() {
    assert_eq!(3, height_checker(vec![1, 1, 4, 2, 1, 3]));
    assert_eq!(5, height_checker(vec![5, 1, 2, 3, 4]));
    assert_eq!(0, height_checker(vec![1, 2, 3, 4, 5]));
}

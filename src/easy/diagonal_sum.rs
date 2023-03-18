pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    mat.into_iter()
        .enumerate()
        .fold(0, |mut sum, (idx, value)| {
            sum += if idx == value.len() - 1 - idx {
                value[idx]
            } else {
                value[idx] + value[value.len() - 1 - idx]
            };
            sum
        })
}

#[test]
fn test_diagonal_sum() {
    assert_eq!(
        diagonal_sum(vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1]
        ]),
        8
    );
    assert_eq!(
        diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        25
    );
}

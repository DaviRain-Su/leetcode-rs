pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    grid.into_iter().fold(0, |mut cnt, innval| {
        cnt += innval
            .into_iter()
            .fold(0, |cnt, v| if v < 0 { cnt + 1 } else { cnt });
        cnt
    })
}

#[test]
fn test_count_negatives() {
    let grid = vec![
        vec![4, 3, 2, -1],
        vec![3, 2, 1, -1],
        vec![1, 1, -1, -2],
        vec![-1, -1, -2, -3],
    ];
    assert_eq!(count_negatives(grid), 8);
    assert_eq!(count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
}

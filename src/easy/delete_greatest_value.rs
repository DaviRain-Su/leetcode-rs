pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
    let grid = grid
        .into_iter()
        .map(|mut v| {
            v.sort();
            v
        })
        .collect::<Vec<_>>();
    if let Some(first) = grid.get(0) {
        (0..first.len()).fold(0, |mut sum, cjx| {
            sum += grid
                .iter()
                .fold(vec![], |mut temp_vec, item| {
                    temp_vec.push(item[cjx]);
                    temp_vec
                })
                .into_iter()
                .max()
                .unwrap_or_default();
            sum
        })
    } else {
        0
    }
}

#[test]
fn test_delete_greatest_value() {
    assert_eq!(delete_greatest_value(vec![vec![1, 2, 4], vec![3, 3, 1]]), 8);
    assert_eq!(delete_greatest_value(vec![vec![10]]), 10);
}

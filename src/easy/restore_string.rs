pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    s.chars()
        .enumerate()
        .fold(vec![' '; s.len()], |mut result, (i, c)| {
            result[indices[i] as usize] = c;
            result
        })
        .into_iter()
        .collect()
}

#[test]
fn test_restore_string() {
    assert_eq!(
        restore_string("codeleet".into(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
        "leetcode".to_string()
    );
    assert_eq!(
        restore_string("abc".into(), vec![0, 1, 2]),
        "abc".to_string()
    )
}

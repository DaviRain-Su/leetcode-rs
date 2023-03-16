pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut result = names
        .into_iter()
        .zip(heights.into_iter())
        .collect::<Vec<_>>();
    result.sort_by(|(_, k1), (_, k2)| k2.cmp(k1));
    result.into_iter().map(|(v, _)| v).collect()
}

#[test]
fn test_sort_people() {
    assert_eq!(
        sort_people(
            vec!["Mary".into(), "John".into(), "Emma".into()],
            vec![180, 165, 170]
        ),
        vec!["Mary".to_string(), "Emma".into(), "John".into()],
    );
    assert_eq!(
        sort_people(
            vec!["Alice".into(), "Bob".into(), "Bob".into()],
            vec![155, 185, 150]
        ),
        vec!["Bob".to_string(), "Alice".into(), "Bob".into()],
    );
}

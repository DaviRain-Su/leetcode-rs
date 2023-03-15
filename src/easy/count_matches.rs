// todo
pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    todo!()
}

#[test]
#[ignore]
fn test_count_matches() {
    assert_eq!(
        count_matches(
            vec![
                vec!["phone".into(), "blue".into(), "pixel".into()],
                vec!["computer".into(), "silver".into(), "lenovo".into()],
                vec!["phone".into(), "gold".into(), "iphone".into()]
            ],
            "color".into(),
            "silver".into()
        ),
        1
    );
    assert_eq!(
        count_matches(
            vec![
                vec!["phone".into(), "blue".into(), "pixel".into()],
                vec!["computer".into(), "silver".into(), "phone".into()],
                vec!["phone".into(), "gold".into(), "iphone".into()]
            ],
            "color".into(),
            "silver".into()
        ),
        1
    );
}

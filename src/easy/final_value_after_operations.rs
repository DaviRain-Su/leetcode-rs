pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations.into_iter().fold(0, |mut ret, operation| {
        match operation.as_str() {
            "++X" | "X++" => ret += 1,
            "--X" | "X--" => ret -= 1,
            _ => {}
        };
        ret
    })
}

#[test]
fn test_final_value_after_operations() {
    assert_eq!(
        1,
        final_value_after_operations(vec!["--X".into(), "X++".into(), "X++".into()])
    );
    assert_eq!(
        3,
        final_value_after_operations(vec!["++X".into(), "++X".into(), "X++".into()])
    );
    assert_eq!(
        0,
        final_value_after_operations(vec!["X++".into(), "++X".into(), "--X".into(), "X--".into()])
    );
}

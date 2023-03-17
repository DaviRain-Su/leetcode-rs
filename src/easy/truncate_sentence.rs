pub fn truncate_sentence(s: String, k: i32) -> String {
    s.split(' ')
        .take(k as usize)
        .enumerate()
        .fold(String::new(), |mut result, (idx, item)| {
            if idx + 1 == k as usize {
                result.push_str(item);
            } else {
                result.push_str(item);
                result.push(' ');
            }
            result
        })
}

#[test]
fn test_truncate_sentence() {
    assert_eq!(
        truncate_sentence("Hello how are you Contestant".into(), 4),
        "Hello how are you".to_string()
    );
    assert_eq!(
        truncate_sentence("What is the solution to this problem".into(), 4),
        "What is the solution".to_string()
    );
    assert_eq!(
        truncate_sentence("chopper is not a tanuki".into(), 5),
        "chopper is not a tanuki".to_string()
    );
}

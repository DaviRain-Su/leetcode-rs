pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    word1.into_iter().fold(String::new(), |mut result, word| {
        result.push_str(&word);
        result
    }) == word2.into_iter().fold(String::new(), |mut result, word| {
        result.push_str(&word);
        result
    })
}

#[test]
fn test_array_strings_are_equal() {
    assert!(array_strings_are_equal(
        vec!["ab".into(), "c".into()],
        vec!["a".into(), "bc".into()]
    ));
    assert!(!array_strings_are_equal(
        vec!["a".into(), "cb".into()],
        vec!["ab".into(), "c".into()]
    ));
    assert!(array_strings_are_equal(
        vec!["abc".into(), "d".into(), "defg".into()],
        vec!["abcddefg".into()]
    ));
}

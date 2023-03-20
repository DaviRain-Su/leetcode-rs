pub fn first_palindrome(words: Vec<String>) -> String {
    words
        .into_iter()
        .fold(vec![], |mut ret, word| {
            if word == word.chars().rev().collect::<String>() {
                ret.push(word);
            }
            ret
        })
        .get(0)
        .cloned() // Option cloned usage
        .unwrap_or_default()
}

#[test]
fn test_first_palindrome() {
    assert_eq!(
        first_palindrome(
            vec!["abc", "car", "ada", "racecar", "cool"]
                .into_iter()
                .map(|v| v.into())
                .collect()
        ),
        "ada".to_string()
    );
}

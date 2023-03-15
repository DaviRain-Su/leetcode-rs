pub fn length_of_last_word(s: String) -> i32 {
    if let Some(v) = s.trim().split(' ').collect::<Vec<_>>().last() {
        v.len() as i32
    } else {
        0
    }
}

#[test]
fn test_length_of_last_word() {
    assert_eq!(5, length_of_last_word("Hello World".into()));
    assert_eq!(4, length_of_last_word("   fly me   to   the moon  ".into()));
    assert_eq!(6, length_of_last_word("luffy is still joyboy".into()));
}

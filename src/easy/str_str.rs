pub fn str_str(haystack: String, needle: String) -> i32 {
    if let Some(idx) = haystack.find(&needle) {
        idx as i32
    } else {
        -1
    }
}

#[test]
fn test_str_str() {
    assert_eq!(0, str_str("sadbutsad".into(), "sad".into()));
    assert_eq!(-1, str_str("leetcode".into(), "leeto".into()));
}

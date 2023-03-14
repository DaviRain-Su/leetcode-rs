pub fn most_words_found(sentences: Vec<String>) -> i32 {
    sentences
        .iter()
        .fold(0usize, |mut max_words_cnt, sentence| {
            let result = sentence.as_str().split(' ').count();
            if max_words_cnt < result {
                max_words_cnt = result
            }
            max_words_cnt
        }) as i32
}

#[test]
fn test_most_words_found() {
    assert_eq!(
        most_words_found(vec![
            "alice and bob love leetcode".into(),
            "i think so too".into(),
            "this is great thanks very much".into()
        ]),
        6
    );
    assert_eq!(
        most_words_found(vec![
            "please wait".into(),
            "continue to fight".into(),
            "continue to win".into()
        ]),
        3
    );
}

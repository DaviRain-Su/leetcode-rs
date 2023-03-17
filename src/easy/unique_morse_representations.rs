use std::collections::HashSet;

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let dict = vec![
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];

    words
        .into_iter()
        .fold(vec![], |mut result, word| {
            let temp = word.chars().fold(String::new(), |mut ret, w| {
                ret.push_str(dict[(w as u8 - b'a') as usize]);
                ret
            });
            result.push(temp);
            result
        })
        .into_iter()
        .collect::<HashSet<_>>()
        .len() as i32
}

#[test]
fn test_unique_morse_representations() {
    assert_eq!(
        unique_morse_representations(
            vec!["gin", "zen", "gig", "msg"]
                .into_iter()
                .map(|v| v.into())
                .collect()
        ),
        2
    );
    assert_eq!(
        unique_morse_representations(vec!["a"].into_iter().map(|v| v.into()).collect()),
        1
    );
}

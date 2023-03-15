pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    jewels.chars().fold(0, |mut cnt, c| {
        cnt += stones.chars().filter(|s| s == &c).count() as i32;
        cnt
    })
}

#[test]
fn test_num_jewels_in_stones() {
    assert_eq!(num_jewels_in_stones("aA".into(), "aAAbbbb".into()), 3);
    assert_eq!(num_jewels_in_stones("z".into(), "ZZ".into()), 0);
}

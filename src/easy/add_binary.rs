/// todo can improve
pub fn add_binary(a: String, b: String) -> String {
    let a_dec = a
        .chars()
        .rev()
        .enumerate()
        .map(|(idx, c)| c.to_digit(10).unwrap() as u128 * 2_u128.pow(idx as u32))
        .sum::<u128>();

    let b_dec = b
        .chars()
        .rev()
        .enumerate()
        .map(|(idx, c)| c.to_digit(10).unwrap() as u128 * 2_u128.pow(idx as u32))
        .sum::<u128>();

    format!("{:b}", a_dec + b_dec)
}

#[test]
fn test_add_binary() {
    assert_eq!(add_binary("11".into(), "1".into()), "100".to_string());
    assert_eq!(
        add_binary("1010".into(), "1011".into()),
        "10101".to_string()
    );

    let result = "11"
        .to_string()
        .chars()
        .zip("1".to_string().chars())
        .collect::<Vec<_>>();
    println!("resutl = {result:?}");
}

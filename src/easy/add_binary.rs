// 这种实现方法的优点：
//
// 直接使用二进制加法进行操作，不需要先将字符串转换为十进制数。
// 使用迭代器a_iter和b_iter分别遍历两个字符串的字符，这可以简化代码结构。
// 使用loop循环，直到两个字符串的所有字符都处理完毕，并且没有进位（carry）为止。
// 使用unwrap_or(0)来处理字符串长度不相等的情况，这可以避免额外的循环或条件判断。
// 最后，将结果字符串翻转，得到正确的二进制加法结果。
// 这个实现方法更直接地执行二进制加法操作，可能在某些情况下性能更优。同时，代码结构也比较清晰，容易理解。
pub fn add_binary(a: String, b: String) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let mut a_iter = a.chars().rev();
    let mut b_iter = b.chars().rev();

    loop {
        let a_digit = a_iter.next().map(|c| c.to_digit(10).unwrap()).unwrap_or(0);
        let b_digit = b_iter.next().map(|c| c.to_digit(10).unwrap()).unwrap_or(0);

        if a_digit == 0 && b_digit == 0 && carry == 0 {
            break;
        }

        let sum = a_digit + b_digit + carry;
        carry = sum / 2;
        result.push_str(&(sum % 2).to_string());
    }

    result.chars().rev().collect()
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

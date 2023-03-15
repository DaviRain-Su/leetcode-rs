// 主要优化点如下：
//
// 直接将字符串num1和num2转换为数字字符的向量num1_chars和num2_chars，并将其反转。这样可以避免在循环中处理字符串长度不同的问题。
// 使用for循环遍历数字字符向量，直到最长的数字字符向量处理完毕。
// 使用get()和cloned().unwrap_or(0)来处理不同长度的数字字符向量，这样可以避免添加前导零和额外的条件判断。
// 计算当前位的和sum，并更新进位carry。
// 检查循环结束后是否还有进位，如果有，则添加到结果字符串。
// 这个优化后的版本更简洁，可读性更强，并减少了重复代码。同时，这种实现方式与add_binary函数的实现更为一致。
pub fn add_strings(num1: String, num2: String) -> String {
    let num1_chars = num1
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let num2_chars = num2
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let mut result = String::new();
    let mut carry = 0;

    for i in 0..num1_chars.len().max(num2_chars.len()) {
        let n1 = num1_chars.get(i).cloned().unwrap_or(0);
        let n2 = num2_chars.get(i).cloned().unwrap_or(0);

        let sum = n1 + n2 + carry;
        carry = sum / 10;
        result.push_str(&(sum % 10).to_string());
    }

    if carry > 0 {
        result.push_str(&carry.to_string());
    }

    result.chars().rev().collect()
}

#[test]
fn test_add_strings() {
    assert_eq!(add_strings("11".into(), "123".into()), "134".to_string());
    assert_eq!(add_strings("456".into(), "77".into()), "533".to_string());
    assert_eq!(add_strings("0".into(), "0".into()), "0".to_string());
    assert_eq!(add_strings("1".into(), "9".into()), "10".to_string());
    assert_eq!(add_strings("584".into(), "18".into()), "602".to_string());
    assert_eq!(add_strings("0".into(), "9".into()), "9".to_string());
    assert_eq!(add_strings("678".into(), "0".into()), "678".to_string());
}

pub fn add_strings(num1: String, num2: String) -> String {
    let num1_len = num1.len();
    let num2_len = num2.len();
    let mut num1 = num1;
    let mut num2 = num2;
    if num1_len < num2_len {
        let mut temp = vec!['0'; num2_len - num1_len]
            .into_iter()
            .collect::<String>();
        temp.push_str(&num1);
        num1 = temp;
    } else {
        let mut temp = vec!['0'; num1_len - num2_len]
            .into_iter()
            .collect::<String>();
        temp.push_str(&num2);
        num2 = temp;
    }

    let (flag, mut result) = num1.chars().rev().zip(num2.chars().rev()).fold(
        (false, vec![]),
        |(mut flag, mut result), (n1, n2)| {
            let n1 = n1.to_string().parse::<u8>().expect("never failed");
            let n2 = n2.to_string().parse::<u8>().expect("never failed");
            if n1 + n2 >= 10 {
                let one = if !flag { 0 } else { 1 };
                result.push((n1 + n2) % 10 + one);
                flag = true;
            } else if n1 + n2 == 9 {
                let value = if !flag {
                    9
                } else {
                    flag = true;
                    0
                };
                result.push(value);
            } else {
                let one = if !flag { 0 } else { 1 };
                result.push(n1 + n2 + one);
                flag = false;
            }
            (flag, result)
        },
    );
    if flag {
        result.push(1);
    }
    result
        .into_iter()
        .rev()
        .fold(String::new(), |mut result, v| {
            result.push_str(&format!("{v}"));
            result
        })
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

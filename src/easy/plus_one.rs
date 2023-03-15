pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let (flag, mut result) =
        digits
            .into_iter()
            .rev()
            .fold((false, vec![]), |(mut flag, mut result), digit| {
                if digit + 1 == 10 && !flag {
                    result.push(0);
                } else if digit + 1 != 10 && !flag {
                    result.push(digit + 1);
                    flag = true;
                } else {
                    result.push(digit);
                }
                (flag, result)
            });

    if !flag {
        result.push(1);
    }

    result.into_iter().rev().collect()
}

#[test]
fn test_plus_one() {
    assert_eq!(vec![1, 2, 4], plus_one(vec![1, 2, 3]));
    assert_eq!(vec![4, 3, 2, 2], plus_one(vec![4, 3, 2, 1]));
    assert_eq!(vec![1, 0], plus_one(vec![9]));
}

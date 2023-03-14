pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    (0..encoded.len())
        .fold((vec![first], first), |(mut result, mut fvalue), idx| {
            let value = encoded[idx] ^ fvalue;
            result.push(value);
            fvalue = value;
            (result, fvalue)
        })
        .0
}

#[test]
fn test_decode() {
    assert_eq!(decode(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
    assert_eq!(decode(vec![6, 2, 7, 3], 4), vec![4, 2, 0, 7, 4]);
}

#[test]
fn test_xor() {
    println!("1 xor 0 = 1");
    println!("1 xor 1 = {}", 1 ^ 1);
    println!("0 xor 2 = {}", 0 ^ 2);
    println!("2 xor 3 = {}", 2 ^ 3);
}

pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    fn num_to_vec(mut num: i32) -> Vec<i32> {
        let mut vec_t = vec![];
        while num >= 10 {
            vec_t.push(num % 10);
            num /= 10;
        }
        vec_t.push(num);
        vec_t.into_iter().rev().collect()
    }
    nums.into_iter().fold(vec![], |mut result, num| {
        result.append(&mut num_to_vec(num));
        result
    })
}

#[test]
fn test_separate_digits() {
    assert_eq!(
        separate_digits(vec![13, 25, 83, 77]),
        vec![1, 3, 2, 5, 8, 3, 7, 7]
    );
    assert_eq!(separate_digits(vec![7, 1, 3, 9]), vec![7, 1, 3, 9]);
    assert_eq!(
        separate_digits(vec![
            32, 43, 68, 8, 100, 84, 80, 14, 88, 42, 53, 98, 69, 64, 40, 60, 23, 99
        ]),
        vec![
            3, 2, 4, 3, 6, 8, 8, 1, 0, 0, 8, 4, 8, 0, 1, 4, 8, 8, 4, 2, 5, 3, 9, 8, 6, 9, 6, 4, 4,
            0, 6, 0, 2, 3, 9, 9
        ]
    );
}

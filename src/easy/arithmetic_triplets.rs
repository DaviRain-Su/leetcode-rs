pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    nums.iter().fold(0, |mut cnt, num| {
        if let Some(v) = nums.iter().find(|&&x| x == num + diff) {
            if nums.iter().any(|&x| x == v + diff) {
                cnt += 1;
            }
        }
        cnt
    })
}

#[test]
fn test_arithmetic_triplets() {
    assert_eq!(arithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3), 2);
    assert_eq!(arithmetic_triplets(vec![4, 5, 6, 7, 8, 9], 2), 2);
}

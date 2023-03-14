pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.into_iter().fold(0, |mut acc, account| {
        let sum: i32 = account.iter().sum();
        if sum > acc {
            acc = sum;
        }
        acc
    })
}

#[test]
fn test_maximum_wealth() {
    assert_eq!(6, maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]));
    assert_eq!(10, maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]));
    assert_eq!(
        17,
        maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]])
    );
}

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    (0..candies.len()).fold(vec![], |mut result, idx| {
        let max_kid_idx = idx;
        let max_kid_with_candie = candies[idx] + extra_candies;
        let r = (0..candies.len()).fold(true, |mut acc, jdx| {
            if max_kid_with_candie < candies[jdx] && max_kid_idx != jdx {
                acc &= false;
            } else {
                acc &= true;
            }
            acc
        });
        result.push(r);
        result
    })
}

#[test]
fn test_kid_with_candies() {
    assert_eq!(
        vec![true, true, true, false, true],
        kids_with_candies(vec![2, 3, 5, 1, 3], 3)
    );
    assert_eq!(
        vec![true, false, false, false, false],
        kids_with_candies(vec![4, 2, 1, 1, 2], 1)
    );

    assert_eq!(
        vec![true, false, true],
        kids_with_candies(vec![12, 1, 12], 10)
    );
}

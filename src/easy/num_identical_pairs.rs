use std::collections::BTreeMap;

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .fold((0, BTreeMap::new()), |(mut cnt, mut map), num| {
            if let Some(v) = map.get_mut(&num) {
                cnt += *v;
                *v += 1;
            } else {
                map.insert(num, 1);
            }
            (cnt, map)
        })
        .0
}

#[test]
fn test_num_identical_pairs() {
    assert_eq!(4, num_identical_pairs(vec![1, 2, 3, 1, 1, 3]));
    assert_eq!(6, num_identical_pairs(vec![1, 1, 1, 1]));
    assert_eq!(0, num_identical_pairs(vec![1, 2, 3]));
}

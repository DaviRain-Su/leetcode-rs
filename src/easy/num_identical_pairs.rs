// public int numIdenticalPairs(int[] A) {
//        int res = 0, count[] = new int[101];
//        for (int a: A) {
//            res += count[a]++;
//        }
//        return res;
// }
pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    use std::collections::BTreeMap;
    let mut cnt = 0;
    // for i in 0..(nums.len()) {
    //     for j in (i + 1)..(nums.len()) {
    //         if nums[i] == nums[j] {
    //             cnt += 1;
    //         }
    //     }
    // }
    let mut map = BTreeMap::new();
    for item in nums {
        if let Some(v) = map.get_mut(&item) {
            cnt += *v;
            *v += 1;
        } else {
            map.insert(item, 1);
        }
    }
    cnt
}

#[test]
fn test_num_identical_pairs() {
    assert_eq!(4, num_identical_pairs(vec![1, 2, 3, 1, 1, 3]));
    assert_eq!(6, num_identical_pairs(vec![1, 1, 1, 1]));
    assert_eq!(0, num_identical_pairs(vec![1, 2, 3]));
}

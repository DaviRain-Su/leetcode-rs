// class Solution:
//     def twoSum(self, nums: List[int], target: int) -> List[int]:

//         d = {}
//         for i, j in enumerate(nums):
//             r = target - j
//             if r in d: return [d[r], i]
//             d[j] = i
// 		# An Upvote will be encouraging

/// use for loop solution this
pub fn solution_v1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] == target - nums[j] {
                return vec![i as i32, j as i32];
            }
        }
    }

    vec![]
}

/// use hashmap solution this problem
pub fn solution_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    nums.into_iter()
        .enumerate()
        .fold((vec![], HashMap::new()), |(ret, mut map), (idx, item)| {
            let right = target - item;
            if let Some(v) = map.get(&right) {
                (vec![*v, idx as i32], map)
            } else {
                map.insert(item, idx as i32);
                (ret, map)
            }
        })
        .0
}

#[test]
fn test_two_sum() {
    assert_eq!(vec![0, 1], solution_v1(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 2], solution_v1(vec![3, 2, 4], 6));
    assert_eq!(vec![0, 1], solution_v1(vec![3, 3], 6));

    assert_eq!(vec![0, 1], solution_v2(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 2], solution_v2(vec![3, 2, 4], 6));
    assert_eq!(vec![0, 1], solution_v2(vec![3, 3], 6));
}

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
    let mut map = HashMap::new();

    for (idx, item) in nums.into_iter().enumerate() {
        let right = target - item;
        if let Some(v) = map.get(&right) {
            return vec![*v, idx as i32];
        } else {
            map.insert(item, idx as i32);
        }
    }
    vec![]
}

// cannot use n size vector to do this
// pub fn solution_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let max_num = nums.clone().into_iter().max().unwrap_or_default();
//     let mut result = vec![0; max_num as usize + 1];
//     for (i, j) in nums.iter().enumerate() {
//         println!("i is {i}, j is {j}");
//         let r = target - j;
//         println!("r is {r}");
//         match result.iter().find(|v| v == &&r) {
//             Some(_v) => {
//                 println!("_v is {_v}");
//                 return vec![result[r as usize], i as i32];
//             }
//             None => {
//                 result[*j as usize] = i as i32;
//             }
//         }
//     }
//     vec![]
// }

#[test]
fn test_two_sum() {
    assert_eq!(vec![0, 1], solution_v1(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 2], solution_v1(vec![3, 2, 4], 6));
    assert_eq!(vec![0, 1], solution_v1(vec![3, 3], 6));

    assert_eq!(vec![0, 1], solution_v2(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 2], solution_v2(vec![3, 2, 4], 6));
    assert_eq!(vec![0, 1], solution_v2(vec![3, 3], 6));
}

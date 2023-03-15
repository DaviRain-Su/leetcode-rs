use std::cmp::Ordering;

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let (mut low, mut high) = (0, nums.len());

    while low < high {
        let mid = low + (high - low) / 2;
        match nums[mid].cmp(&target) {
            Ordering::Equal => {
                return mid as i32;
            }
            Ordering::Greater => high = mid,
            Ordering::Less => low = mid + 1,
        }
    }
    low as i32
}

#[test]
fn test_search_insert() {
    assert_eq!(2, search_insert(vec![1, 3, 5, 6], 5));
    assert_eq!(1, search_insert(vec![1, 3, 5, 6], 2));
    assert_eq!(4, search_insert(vec![1, 3, 5, 6], 7));
}

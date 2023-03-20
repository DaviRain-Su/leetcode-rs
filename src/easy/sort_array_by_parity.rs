pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let mut start = 0usize;
    let mut end = nums.len() - 1;
    let mut nums = nums;
    while start < end {
        if nums[start] % 2 != 0 && nums[end] % 2 == 0 {
            nums.swap(start, end);
            start += 1;
            end -= 1;
        }
        if nums[start] % 2 == 0 {
            start += 1;
        }
        if nums[end] % 2 != 0 {
            end -= 1;
        }
    }
    nums
}

#[test]
fn test_sort_array_by_parity() {
    let ret = sort_array_by_parity(vec![3, 1, 2, 4]);
    println!("ret = {ret:?}");

    let ret = sort_array_by_parity(vec![0, 1]);
    println!("ret = {ret:?}");
}

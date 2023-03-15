pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let count = (1..nums.len()).fold(0, |mut cnt, idx| {
        if nums[idx] == nums[idx - 1] {
            cnt += 1;
        } else {
            nums[idx - cnt] = nums[idx];
        }
        cnt
    });

    (nums.len() - count) as i32
}

fn swap_range(array: &mut [i32], start: usize, end: usize) {
    let end = if (end - start) % 2 != 0 { end - 1 } else { end };
    for idx in start..end {
        let mut temp = 0;
        std::mem::swap(&mut temp, &mut array[idx]);
        std::mem::swap(&mut array[idx + 1], &mut temp);
        std::mem::swap(&mut temp, &mut array[idx]);
    }
}

#[test]
fn test_remove_duplicates() {
    let mut temp_vec = vec![1, 1, 2];
    assert_eq!(2, remove_duplicates(&mut temp_vec));
}

#[test]
fn test_swap_range() {
    let mut temp_vec = vec![1, 1, 2];
    let ret = remove_duplicates(&mut temp_vec);
    println!("temp vec = {temp_vec:?}, ret = {ret}");
    let mut temp_vec = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let ret = remove_duplicates(&mut temp_vec);
    println!("temp vec = {temp_vec:?}, ret = {ret}");
}

pub fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    let mut i = m - 1;
    let mut j = n - 1;
    let mut k = (m + n) - 1;

    while i >= 0 && j >= 0 {
        if nums1[i as usize] > nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
        k -= 1;
    }

    while j >= 0 {
        nums1[k as usize] = nums2[j as usize];
        j -= 1;
        k -= 1;
    }
}

#[test]
fn test_merge() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);

    let mut nums1 = vec![1];
    let mut nums2 = vec![];
    merge(&mut nums1, 1, &mut nums2, 0);
    assert_eq!(nums1, vec![1]);

    let mut nums1 = vec![0];
    let mut nums2 = vec![1];
    merge(&mut nums1, 0, &mut nums2, 1);
    assert_eq!(nums1, vec![1]);

    let mut nums1 = vec![2, 0];
    let mut nums2 = vec![1];
    merge(&mut nums1, 1, &mut nums2, 1);
    assert_eq!(nums1, vec![1, 2]);
}

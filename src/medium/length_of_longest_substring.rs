// 这段代码执行以下操作：
//
// 创建一个空的哈希映射char_indices，用于存储当前窗口中字符的索引。
// 初始化变量longest_substring和start，分别跟踪最长子字符串的长度和当前窗口的起始索引。
// 将输入字符串s转换为字符向量。
// 使用索引迭代字符串中的字符。
// 如果当前字符已经存在于哈希映射中，将起始索引更新为当前起始索引和哈希映射中字符索引加1的较大值。
// 使用当前最长子字符串和当前窗口长度的较大值更新最长子字符串的长度。
// 使用当前字符及其索引更新哈希映射。
// 返回最长子字符串的长度（转换为i32类型）。
// 此解决方案的时间复杂度为O(n)，其中n是输入字符串的长度，因为它一次遍历整个字符串。
pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;
    let mut char_indices = HashMap::new();
    let mut longest_substring = 0;
    let mut start = 0;
    let chars: Vec<char> = s.chars().collect();

    for (end, &c) in chars.iter().enumerate() {
        if let Some(&prev_index) = char_indices.get(&c) {
            start = std::cmp::max(start, prev_index + 1);
            println!("start is {start}, c is {c}, end is {end}");
        }
        println!("start is {start}, end is {end}");
        longest_substring = std::cmp::max(longest_substring, end - start + 1);
        println!("longest substrig is {longest_substring}");
        char_indices.insert(c, end);
        println!("char_indices is {char_indices:?}");
    }
    longest_substring as i32
}

#[test]
fn test_length_of_longest_substring() {
    assert_eq!(3, length_of_longest_substring("abcabcbb".into()));
    assert_eq!(1, length_of_longest_substring("bbbbb".into()));
    assert_eq!(3, length_of_longest_substring("pwwkew".into()));
}

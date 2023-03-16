pub mod sum_numbers;

pub mod add_two_numbers;
/// # 3. Longest Substring Without Repeating Characters
///
/// Given a string s, find the length of the longest  substring without repeating characters.
///
/// ```ts
/// initialize left_ptr = 0, right_ptr = 0, max_length = 0
/// initialize a data structure (e.g., set or hash_map) to store unique characters in the current window
///
/// while right_ptr < length of s:
///    if s[right_ptr] is not in the data structure:
///        add s[right_ptr] to the data structure
///        update max_length to be the maximum of max_length and the size of the data structure
///        increment right_ptr
///    else:
///        remove s[left_ptr] from the data structure
///        increment left_ptr
///
/// return max_length
/// ```
/// 这个伪代码实现了滑动窗口方法，通过调整窗口的左右指针来查找字符串s中最长的不包含重复字符的子串。
/// 当遇到重复字符时，窗口的左侧指针会向右移动，从而在窗口内保持不重复的字符。同时，在遍历过程中，我们会持续更新最长子串的长度。
pub mod length_of_longest_substring;

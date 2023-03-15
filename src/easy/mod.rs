/// # 1920. Build Array from Permutation
///
/// Given a zero-based permutation nums (0-indexed), build an array ans of the same length where `ans[i] = nums[nums[i]]` for each `0 <= i < nums`.length and return it.
///
/// A zero-based permutation nums is an array of distinct integers from 0 to `nums.length - 1` (inclusive).
pub mod build_array;
/// # 1773. Count Items Matching a Rule
///
/// You are given an array items, where each items[i] = [typei, colori, namei] describes the type, color, and name of the ith item. You are also given a rule represented by two strings, ruleKey and ruleValue.
///
/// The ith item is said to match the rule if one of the following is true:
///
/// - ruleKey == "type" and ruleValue == typei.
/// - ruleKey == "color" and ruleValue == colori.
/// - ruleKey == "name" and ruleValue == namei.
/// Return the number of items that match the given rule.
pub mod count_matches;
/// # 1389. Create Target Array in the Given Order
///
/// Given two arrays of integers nums and index. Your task is to create target array under the following rules:
///
/// Initially target array is empty.
/// From left to right read nums[i] and index[i], insert at index index[i] the value nums[i] in target array.
/// Repeat the previous step until there are no elements to read in nums and index.
/// Return the target array.
///
/// It is guaranteed that the insertion operations will be valid.
pub mod create_target_array;
/// # 1720. Decode XORed Array
///
/// There is a hidden integer array arr that consists of n non-negative integers.
///
/// It was encoded into another integer array encoded of length n - 1, such that `encoded[i] = arr[i] XOR arr[i + 1]`. For example, if `arr = [1,0,2,1]`, then `encoded = [1,2,3]`.
///
/// You are given the encoded array. You are also given an integer first, that is the first element of arr, i.e. `arr[0]`.
///
/// Return the original array arr. It can be proved that the answer exists and is unique.
pub mod decode;
/// # 1313. Decompress Run-Length Encoded List
///
/// We are given a list nums of integers representing a list compressed with run-length encoding.
///
/// Consider each adjacent pair of elements `[freq, val] = [nums[2*i], nums[2*i+1]]` (with i >= 0).  For each such pair, there are freq elements with value val concatenated in a sublist. Concatenate all the sublists from left to right to generate the decompressed list.
///
/// Return the decompressed list.
pub mod decompress_rl_elist;
/// # 2535. Difference Between Element Sum and Digit Sum of an Array
///
/// You are given a positive integer array nums.
///
/// The element sum is the sum of all the elements in nums.
/// The digit sum is the sum of all the digits (not necessarily distinct) that appear in nums.
/// Return the absolute difference between the element sum and digit sum of nums.
///
/// Note that the absolute difference between two integers x and y is defined as |x - y|.
pub mod difference_of_sum;
/// # 2011. Final Value of Variable After Performing Operations
///
/// There is a programming language with only four operations and one variable X:
///
/// `++X` and `X++` increments the value of the variable X by 1.
/// `--X` and `X--` decrements the value of the variable X by 1.
/// Initially, the value of X is 0.
///
/// Given an array of strings operations containing a list of operations, return the final value of X after performing all the operations.
pub mod final_value_after_operations;
/// # 1929. Concatenation of Array
///
/// Given an integer array nums of length n, you want to create an array ans of length 2n where `ans[i] == nums[i]` and `ans[i + n] == nums[i]` for `0 <= i < n` (0-indexed).
///
/// Specifically, ans is the concatenation of two nums arrays.
///
/// Return the array ans.
pub mod get_concatenation;
/// #1431. Kids With the Greatest Number of Candies
///
/// There are n kids with candies. You are given an integer array candies, where each `candies[i]` represents the number of candies the ith kid has, and an integer extraCandies, denoting the number of extra candies that you have.
///
/// Return a boolean array result of length n, where `result[i]` is true if, after giving the ith kid all the extraCandies, they will have the greatest number of candies among all the kids, or false otherwise.
///
/// Note that multiple kids can have the greatest number of candies.
///
pub mod kids_with_candies;
/// # 2574. Left and Right Sum Differences
///
/// Given a 0-indexed integer array nums, find a 0-indexed integer array answer where:
///
/// `answer.length == nums.length`.
/// `answer[i] = |leftSum[i] - rightSum[i]|`.
/// Where:
/// - `leftSum[i]` is the sum of elements to the left of the index i in the array nums. If there is no such element, `leftSum[i]` = 0.
/// - `rightSum[i]` is the sum of elements to the right of the index i in the array nums. If there is no such element, `rightSum[i]` = 0.
///
/// Return the array answer.
pub mod left_rigth_difference;
/// # 58. Length of Last Word
///
/// Given a string s consisting of words and spaces, return the length of the last word in the string.
///
/// A word is a maximal  substring consisting of non-space characters only.
pub mod length_of_last_word;
/// # 1672. Richest Customer Wealth
///
/// You are given an m x n integer grid accounts where `accounts[i][j]` is the amount of money the i​​​​​​​​​​​th​​​​ customer has in the j​​​​​​​​​​​th​​​​ bank. Return the wealth that the richest customer has.
///
/// A customer's wealth is the amount of money they have in all their bank accounts. The richest customer is the customer that has the maximum wealth.
pub mod maximum_wealth;
/// # 2114. Maximum Number of Words Found in Sentences
///
/// A sentence is a list of words that are separated by a single space with no leading or trailing spaces.
///
/// You are given an array of strings sentences, where each `sentences[i]` represents a single sentence.
///
/// Return the maximum number of words that appear in a single sentence.
pub mod most_words_found;
/// # 1512. Number of Good Pairs
///
/// Given an array of integers nums, return the number of good pairs.
///
/// A pair (i, j) is called good if `nums[i] == nums[j]` and `i < j`.
pub mod num_identical_pairs;
/// # 26. Remove Duplicates from Sorted Array
///
/// Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same.
///
/// Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.
///
/// Return k after placing the final result in the first k slots of nums.
///
/// Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.
///
pub mod remove_duplicates;
/// # 27. Remove Element
///
/// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The relative order of the elements may be changed.
///
/// Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.
///
/// Return k after placing the final result in the first k slots of nums.
///
/// Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.
///
pub mod remove_element;
pub mod restore_string;
/// # 1480. Running Sum of 1d Array
///
/// Given an array nums. We define a running sum of an array as `runningSum[i] = sum(nums[0]…nums[i])`.
///
/// Return the running sum of nums.
pub mod running_sum;
/// # 35. Search Insert Position
///
/// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
///
/// You must write an algorithm with O(log n) runtime complexity.
pub mod search_insert;
/// # 1470. Shuffle the Array
///
/// Given the array nums consisting of 2n elements in the form `[x1,x2,...,xn,y1,y2,...,yn]`.
///
/// Return the array in the form `[x1,y1,x2,y2,...,xn,yn]`.
pub mod shuffle;
/// # 1365. How Many Numbers Are Smaller Than the Current Number
///
/// Given the array nums, for each `nums[i]` find out how many numbers in the array are smaller than it. That is, for each `nums[i]` you have to count the number of valid j's such that j != i and nums[j] < nums[i].
///
/// Return the answer in an array.
pub mod smaller_numbers_than_current;
/// # 28. Find the Index of the First Occurrence in a String
///
/// Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
///
pub mod str_str;
/// #1 Two Sum
///
/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
///
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
///
/// You can return the answer in any order.
pub mod two_sum;

pub mod plus_one;

pub mod add_binary;
/// # 94. Binary Tree Inorder Traversal
///
/// Given the root of a binary tree, return the inorder traversal of its nodes' values.
///
/// ```
/// inorderTraversal(root):
///    result = []
///    stack = []
///    curr = root
///
///    while curr != null or stack is not empty:
///        while curr != null:
///            stack.push(curr)
///            curr = curr.left
///
///        node = stack.pop()
///        result.push(node.val)
///        curr = node.right
///
///    return result
///```
///
/// 该算法使用了一个栈来模拟递归遍历。我们首先将根节点压入栈中，然后遍历其左子树，将左子节点压入栈中。
/// 在遍历完左子树后，我们弹出栈顶元素并将其加入结果向量中，然后遍历其右子树，将右子节点压入栈中。这样不断地遍历栈中的元素，直到所有节点都被遍历完为止。
///
/// 注意，伪码中的 null 应该被替换为 None，算法的实现语言中可能也需要进行相应的修改。
pub mod inorder_traversal;

pub mod defang_i_paddr;

pub mod num_jewels_in_stones;

pub mod add_strings;

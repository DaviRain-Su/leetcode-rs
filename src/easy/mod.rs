/// # 67. Add Binary
///
/// Given two binary strings a and b, return their sum as a binary string.
pub mod add_binary;
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
/// # 1108. Defanging an IP Address
///
/// Given a valid (IPv4) IP address, return a defanged version of that IP address.
///
/// A defanged IP address replaces every period "." with "[.]".
pub mod defang_i_paddr;
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
/// # 94. Binary Tree Inorder Traversal
///
/// Given the root of a binary tree, return the inorder traversal of its nodes' values.
///
/// ```ts
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
/// # 1431. Kids With the Greatest Number of Candies
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
/// # 66. Plus One
///
/// You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.
///
/// Increment the large integer by one and return the resulting array of digits.
pub mod plus_one;
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
/// # 1528. Shuffle String
///
/// You are given a string s and an integer array indices of the same length. The string s will be shuffled such that the character at the ith position moves to indices[i] in the shuffled string.
///
/// Return the shuffled string.
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

/// # 415. Add Strings
///
/// Given two non-negative integers, num1 and num2 represented as string, return the sum of num1 and num2 as a string.
///
/// You must solve the problem without using any built-in library for handling large integers (such as BigInteger). You must also not convert the inputs to integers directly.
pub mod add_strings;
/// # 2367. Number of Arithmetic Triplets
///
/// You are given a 0-indexed, strictly increasing integer array nums and a positive integer diff. A triplet `(i, j, k)` is an
/// arithmetic triplet if the following conditions are met:
///
/// `i < j < k`,
/// `nums[j] - nums[i] == diff`, and
/// `nums[k] - nums[j] == diff`.
/// Return the number of unique arithmetic triplets.
pub mod arithmetic_triplets;
/// # 1662. Check If Two String Arrays are Equivalent
/// Given two string arrays word1 and word2, return true if the two arrays represent the same string, and false otherwise.
///
/// A string is represented by an array if the array elements concatenated in order forms the string.
pub mod array_strings_are_equal;
/// # 70. Climbing Stairs
///
/// You are climbing a staircase. It takes n steps to reach the top.
///
///Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
pub mod climb_stairs;
/// #  2006. Count Number of Pairs With Absolute Difference K
///
/// Given an integer array nums and an integer k, return the number of pairs (i, j) where i < j such that |nums[i] - nums[j]| == k.
///
/// The value of |x| is defined as:
///
/// x if x >= 0.
/// -x if x < 0.
pub mod count_k_difference;
/// # 1295. Find Numbers with Even Number of Digits
///
/// Given an array nums of integers, return how many of them contain an even number of digits.
pub mod find_numbers;
/// # 14. Longest Common Prefix
///
/// Write a function to find the longest common prefix string amongst an array of strings.
///
///If there is no common prefix, return an empty string "".
pub mod longest_common_prefix;
/// # 771. Jewels and Stones
///
/// You're given strings jewels representing the types of stones that are jewels, and stones representing the stones you have. Each character in stones is a type of stone you have. You want to know how many of the stones you have are also jewels.
///
/// Letters are case sensitive, so "a" is considered a different type of stone from "A".
pub mod num_jewels_in_stones;
/// # 13. Roman to Integer
///
/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
///
/// Symbol       Value
/// I             1
/// V             5
/// X             10
/// L             50
/// C             100
/// D             500
/// M             1000
/// For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
///
/// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
///
/// I can be placed before V (5) and X (10) to make 4 and 9.
/// X can be placed before L (50) and C (100) to make 40 and 90.
/// C can be placed before D (500) and M (1000) to make 400 and 900.
/// Given a roman numeral, convert it to an integer.
pub mod roman_to_int;
/// # 2418. Sort the People
///
/// You are given an array of strings names, and an array heights that consists of distinct positive integers. Both arrays are of length n.
///
/// For each index i, names[i] and heights[i] denote the name and height of the ith person.
///
/// Return names sorted in descending order by the people's heights.
pub mod sort_people;
/// # 1588. Sum of All Odd Length Subarrays
///
/// Given an array of positive integers arr, return the sum of all possible odd-length subarrays of arr.
///
/// A subarray is a contiguous subsequence of the array.
pub mod sum_odd_length_subarrays;
/// #  1816. Truncate Sentence
///
/// A sentence is a list of words that are separated by a single space with no leading or trailing spaces. Each of the words consists of only uppercase and lowercase English letters (no punctuation).
///
/// For example, "Hello World", "HELLO", and "hello world hello world" are all sentences.
/// You are given a sentence s​​​​​​ and an integer k​​​​​​. You want to truncate s​​​​​​ such that it contains only the first k​​​​​​ words. Return s​​​​​​ after truncating it.
pub mod truncate_sentence;
/// # 804. Unique Morse Code Words
///
/// International Morse Code defines a standard encoding where each letter is mapped to a series of dots and dashes, as follows:
///
/// 'a' maps to ".-",
/// 'b' maps to "-...",
/// 'c' maps to "-.-.", and so on.
/// For convenience, the full table for the 26 letters of the English alphabet is given below:
///
/// [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."]
/// Given an array of strings words where each word can be written as a concatenation of the Morse code of each letter.
///
/// For example, "cab" can be written as "-.-..--...", which is the concatenation of "-.-.", ".-", and "-...". We will call such a concatenation the transformation of a word.
/// Return the number of different transformations among all words we have.
pub mod unique_morse_representations;

pub mod max_product_difference;

pub mod delete_greatest_value;

pub mod flip_and_invert_image;

pub mod diagonal_sum;

pub mod separate_digits;

pub mod first_palindrome;

pub mod find_gcd;

pub mod merge;

pub mod sort_array_by_parity;

pub mod height_checker;

pub mod count_negatives;

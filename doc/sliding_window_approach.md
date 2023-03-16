# Sliding window approach

滑动窗口（sliding window）方法是一种在序列数据（如字符串、数组等）中找到满足特定条件的**连续子序列**的算法。滑动窗口方法的核心思想是使用两个指针（通常称为左指针和右指针）表示当前的窗口。在遍历过程中，根据问题的要求和窗口内的状态调整窗口的大小和位置。

这里是滑动窗口方法的一般步骤：

1. 初始化左指针和右指针（通常为0），并定义一个变量来跟踪满足条件的子序列。
2. 增加右指针，扩大窗口，直到满足题目要求。
3. 在满足题目要求的情况下，更新结果变量。
4. 收缩窗口，即移动左指针，直到窗口内不再满足题目要求。
5. 重复步骤2-4，直到右指针遍历完整个序列。

在寻找最长不重复子字符串的问题中，滑动窗口方法可以用来找到满足不包含重复字符的条件的子字符串。当遇到一个已经存在于当前窗口的字符时，我们将窗口的左边界移动到重复字符的下一个位置。同时，我们将窗口的右边界向右移动一位，直到遍历完整个字符串。

使用滑动窗口方法的优势在于它能够有效地在O(n) 时间内解决问题，其中n是序列的长度。这是因为左指针和右指针分别只遍历整个序列一次。此外，**滑动窗口方法通常可以很容易地扩展到处理更复杂的序列问题。**

## problem set

- LeetCode 3 - 无重复字符的最长子串（Longest Substring Without Repeating Characters）
- LeetCode 76 - 最小覆盖子串（Minimum Window Substring）
- LeetCode 209 - 长度最小的子数组（Minimum Size Subarray Sum）
- LeetCode 424 - 替换后的最长重复字符（Longest Repeating Character Replacement）
- LeetCode 438 - 找到字符串中所有字母异位词（Find All Anagrams in a String）
- LeetCode 567 - 字符串的排列（Permutation in String）
- LeetCode 713 - 乘积小于K的子数组（Subarray Product Less Than K）
- LeetCode 904 - 水果成篮（Fruit Into Baskets）
- LeetCode 930 - 和相同的二元子数组（Binary Subarrays With Sum）
- LeetCode 992 - K 个不同整数的子数组（Subarrays with K Different Integers）

### leetcode 3

我将尝试提供一个以时间为轴的图解，以帮助您理解滑动窗口的概念。在每个阶段，我们将显示窗口范围、左指针（L）和右指针（R）的位置。

假设我们有一个字符串s = "abcabcbb"，我们需要找到最长的不包含重复字符的子串。

1. 初始化：

```
L R
a b c a b c b b
```

2. 右指针向右移动，找到重复字符：

```
L     R
a b c a b c b b
```

3. 左指针向右移动，直到窗口中没有重复字符：

```
  L   R
a b c a b c b b
```

4. 右指针继续向右移动，找到重复字符：

```
  L     R
a b c a b c b b
```

5.  左指针向右移动，直到窗口中没有重复字符

```
    L   R
a b c a b c b b
```

6. 右指针继续向右移动，找到重复字符：

```
    L     R
a b c a b c b b
```


继续按照这个模式移动指针，直到右指针到达字符串的末尾。在这个例子中，最长子串的长度为3（"abc"）。

希望这种表示方式能帮助您更好地理解滑动窗口的概念。


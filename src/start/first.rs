
// https://leetcode.com/problems/count-elements-with-maximum-frequency/description/?envType=daily-question&envId=2024-03-08

// You are given an array nums consisting of positive integers.
// Return the total frequencies of elements in nums such that those elements all have the maximum frequency.
// The frequency of an element is the number of occurrences of that element in the array.
// Example 1:
// Input: nums = [1,2,2,3,1,4]
// Output: 4
// Explanation: The elements 1 and 2 have a frequency of 2 which is the maximum frequency in the array.
// So the number of elements in the array with maximum frequency is 4.
// Example 2:
// Input: nums = [1,2,3,4,5]
// Output: 5
// Explanation: All elements of the array have a frequency of 1 which is the maximum.
// So the number of elements in the array with maximum frequency is 5.
// Constraints:
//     1 <= nums.length <= 100
//     1 <= nums[i] <= 100

// first iteration.
fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    for num in nums {
        if num > max {
            max = num;
        }
    }
    max
}

fn max_frequency_elements2(nums: Vec<i32>) -> i32 {
    nums.iter().max().unwrap().to_owned()
}

// todo! add test
pub fn solution() {
    let nums = [1,2,2,3,1,4];
    let nums2 = [1,2,3,4,5];

    let result = max_frequency_elements(nums.to_vec());
    let result2 = max_frequency_elements(nums2.to_vec());

    let result3 = max_frequency_elements2(nums.to_vec());
    let result4 = max_frequency_elements2(nums2.to_vec());



    println!("Leetcode task 1");
    println!("[1,2,2,3,1,4] {} expected 4", result);
    println!("[1,2,3,4,5] {} expected 5", result2);
    println!("Leetcode task 1 later iteration:");
    println!("[1,2,2,3,1,4] {} expected 4", result3);
    println!("[1,2,3,4,5] {} expected 5", result4);
}
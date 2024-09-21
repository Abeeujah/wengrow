use std::{
    iter::Sum,
    ops::{Add, Div, Rem, Sub},
    usize,
};

// O(N) runtime
fn avg_of_even_numbers<T>(list: &[T]) -> Option<T>
where
    T: Copy + PartialEq + Div<Output = T> + From<i32> + Add<Output = T> + Rem<Output = T>,
{
    let mut count = 0;
    let mut sum = T::from(0);
    for num in list {
        if *num % T::from(2) == T::from(0) {
            count += 1;
            sum = *num + sum;
        }
    }
    match sum == T::from(0) {
        true => None,
        false => Some(sum / T::from(count)),
    }
}

// O(N²) runtime
fn word_builder(chars: &[char]) -> Vec<String> {
    let mut words = Vec::new();
    for i in chars {
        for j in chars {
            if i == j {
                continue;
            }
            words.push(format!("{}{}", i, j));
        }
    }
    words
}

// O(1) runtime
fn sample<T>(list: Vec<T>) -> Vec<T>
where
    T: Copy,
{
    let len = list.len();
    if len < 3 {
        return list;
    }
    let mid = if len % 2 == 0 { len / 2 } else { (len / 2) + 1 };
    let first = list[0];
    let mid = list[mid];
    let last = list[len - 1];
    vec![first, mid, last]
}

// O(N) runtime
fn avg_celcius<T>(far_readings: Vec<T>) -> T
where
    T: Copy + Sub<Output = T> + Div<Output = T> + From<i32> + From<f64> + From<usize> + Sum,
{
    let sum = far_readings
        .iter()
        .map(|reading| (*reading - T::from(32)) / T::from(1.8))
        .sum::<T>();

    sum / T::from(far_readings.len())
}

// O(N) runtime
fn size_clothes(clothes: Vec<&str>) -> Vec<String> {
    let mut sizes = Vec::new();
    for cloth in clothes {
        for size in 1..6 {
            sizes.push(format!("{} {} {}", cloth, "Size:", size));
        }
    }
    sizes
}

// O(N) runtime
fn count_ones(lists: &[&[i32]]) -> i32 {
    let mut count = 0;
    for list in lists {
        for num in *list {
            if *num != 1 {
                continue;
            }
            count += 1;
        }
    }
    count
}

// O(N) runtime
fn is_palindrome(word: &str) -> bool {
    let rev = word.chars().collect::<String>();
    word == rev
}

// O(N) runtime
fn is_palindrome_not_idiomatic(word: &str) -> bool {
    let mut left_index = 0;
    let mut right_index = word.len() - 1;
    let chars = word.chars().collect::<Vec<char>>();
    while left_index < right_index / 2 {
        if chars[left_index] != chars[right_index] {
            return false;
        }
        left_index += 1;
        right_index += 1;
    }
    true
}

fn hundred_sum<T>(list: &[T]) -> bool
where
    T: Copy + PartialEq + Add<Output = T> + From<i32>,
{
    let mut left_index = 0;
    let mut right_index = list.len() - 1;
    while left_index < right_index / 2 {
        if list[left_index] + list[right_index] != T::from(100) {
            return false;
        }
        left_index += 1;
        right_index += 1;
    }
    true
}

// O(N²) runtime
fn prod_nums(nums: &[i32]) -> Vec<i32> {
    let mut prods = Vec::new();
    for (i, num) in nums.iter().enumerate() {
        for j in i + 1..nums.len() {
            prods.push(num * nums[j])
        }
    }
    prods
}

// O(N * M) runtime
fn prods_nums_lists(nums: &[i32], list: &[i32]) -> Vec<i32> {
    let mut prods = Vec::new();
    for num in nums {
        for l in list {
            prods.push(num * l);
        }
    }
    prods
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_avg() {
        assert_eq!(Some(4), avg_of_even_numbers(&[4, 4]));
    }

    #[test]
    fn test_word_builder() {
        assert_eq!(
            vec!["ab", "ac", "ad", "ba", "bc", "bd", "ca", "cb", "cd", "da", "db", "dc"],
            word_builder(&['a', 'b', 'c', 'd'])
        )
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("lawal"));
    }

    #[test]
    fn test_prods() {
        assert_eq!(
            vec![2, 3, 4, 5, 6, 8, 10, 12, 15, 20],
            prod_nums(&[1, 2, 3, 4, 5])
        );
    }
}

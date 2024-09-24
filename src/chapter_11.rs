use std::ops::{Add, AddAssign, Mul, MulAssign, Rem};

// Modify an array in place, using recursion
pub fn double_array<T>(list: &mut [T], index: usize) -> Vec<T>
where
    T: Clone + Mul + MulAssign + From<i32>,
{
    if index >= list.len() {
        return list.to_vec();
    }
    list[index] *= T::from(2);
    double_array(list, index + 1)
}

// Sum all the elements in an array
pub fn sum<T>(list: &[T]) -> T
where
    T: Default + AddAssign + Copy,
{
    let mut sum = T::default();
    for i in list {
        sum += *i;
    }
    sum
}

pub fn recursive_sum<T>(list: &[T]) -> T
where
    T: Add<Output = T> + Copy,
{
    if list.len() == 1 {
        return list[0];
    }
    return list[0] + recursive_sum(&list[1..]);
}

// Reverse every character in a string
pub fn reverse_str(word: &str) -> String {
    word.chars().rev().collect()
}

pub fn recursive_reverse_str(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }
    let (first, rest) = word.split_at(1);
    recursive_reverse_str(rest) + first
}

// Count the occurences of "x" in a given string
pub fn count_x(word: &str) -> usize {
    word.chars()
        .filter(|c| c.to_ascii_lowercase() == 'x')
        .count()
}
pub fn recursive_count_x(word: &str) -> u32 {
    if word.is_empty() {
        return 0;
    }

    let (first, rest) = word.split_at(1);
    let first_char = first.chars().next().unwrap_or_default();

    match first_char.is_ascii_lowercase() && first_char == 'x' {
        true => 1 + recursive_count_x(rest),
        false => recursive_count_x(rest),
    }
}

// Staircase number of paths problem
// If a traveller can go 1, 2, 3 steps on a staircase,
// how many possible paths can they go
pub fn number_of_paths(steps: i32) -> u32 {
    if steps < 0 {
        return 0;
    }
    match steps {
        0 | 1 => 1,
        _ => number_of_paths(steps - 1) + number_of_paths(steps - 2) + number_of_paths(steps - 3),
    }
}

// Write a function that returns the Anagrams of a given string
pub fn anagrams_of(string: &str) -> Vec<String> {
    if string.len() == 1 {
        return vec![string.to_string()];
    }

    let mut collection: Vec<String> = Vec::new();
    let substring_anagrams = anagrams_of(&string[1..]);

    for substring_anagram in substring_anagrams {
        for index in 0..substring_anagram.len() + 1 {
            let mut copy = substring_anagram.clone();
            copy.insert(index, string.chars().next().unwrap());
            collection.push(copy);
        }
    }

    collection
}

// Write a function that accepts an array of strings and
// returns the total number of characters across all the strings.
pub fn char_count(words: &[&str]) -> usize {
    words.iter().map(|word| word.len()).sum()
}

pub fn recursive_char_count(words: &[&str]) -> usize {
    if words.is_empty() {
        return 0;
    }
    return words[0].len() + recursive_char_count(&words[1..]);
}

// Write a function that accepts an array of numbers and
// returns a new array containing just the even numbers.
pub fn even_only<T>(numbers: &[T]) -> Vec<T>
where
    T: From<i32> + Rem<Output = T> + PartialEq + Copy,
{
    numbers
        .iter()
        .filter(|num| **num % T::from(2) == T::from(0))
        .copied()
        .collect()
}

pub fn recursive_even_only<T>(numbers: &[T]) -> Vec<T>
where
    T: From<i32> + Rem<Output = T> + PartialEq + Copy,
{
    if numbers.is_empty() {
        return Vec::new();
    } else {
        let head = numbers[0];
        let tail = &numbers[1..];
        let mut result = recursive_even_only(tail);
        if head % T::from(2) == T::from(0) {
            result.insert(0, head);
        }
        result
    }
}

// Write a function that accepts a number and
// returns the correct number from the triangular series
pub fn trig_seq(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    return n + trig_seq(n - 1);
}

// write a function that accepts a string and returns the
// first index that contains the character "x"
pub fn x_index(word: &str) -> Option<usize> {
    word.chars().position(|c| c == 'x')
}

pub fn recursive_x_index(word: &str, index: usize) -> Option<usize> {
    if index >= word.len() {
        return None;
    }

    if word.chars().nth(index).unwrap() == 'x' {
        return Some(index);
    }

    recursive_x_index(word, index + 1)
}

pub fn shortest_paths(rows: usize, cols: usize) -> u64 {
    if rows == 0 || cols == 0 {
        return 0;
    } else if rows == 1 || cols == 1 {
        return 1;
    } else {
        return shortest_paths(rows - 1, cols) + shortest_paths(rows, cols - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_array() {
        assert_eq!(vec![2, 4, 6, 8], double_array(&mut vec![1, 2, 3, 4], 0));
    }

    #[test]
    fn test_recursive_sum() {
        assert_eq!(10, recursive_sum(&[1, 2, 3, 4]));
    }

    #[test]
    fn test_recursive_reverse_str() {
        assert_eq!("lawal", recursive_reverse_str("lawal"));
    }

    #[test]
    fn test_recursive_count_x() {
        assert_eq!(2, recursive_count_x("xxw"));
    }

    #[test]
    fn test_number_of_paths() {
        assert_eq!(4, number_of_paths(3));
    }

    #[test]
    fn test_anagrams_of() {
        assert_eq!(
            vec!["abc", "bac", "bca", "acb", "cab", "cba"],
            anagrams_of("abc")
        );
    }

    #[test]
    fn test_recursive_char_count() {
        assert_eq!(9, recursive_char_count(&["word", "up", "gee"]));
    }

    #[test]
    fn test_recursive_even_only() {
        assert_eq!(vec![2, 4], recursive_even_only(&[1, 2, 3, 4]));
    }

    #[test]
    fn test_trig_seq() {
        assert_eq!(28, trig_seq(7));
    }
}

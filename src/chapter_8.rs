use std::{
    char,
    collections::{HashMap, HashSet},
    hash::Hash,
};

// Write a function to determine if an array is a subset of a another array
// Simple Approach
// O(N * M) runtime
fn is_subset_crude<T>(set: &[T], subset: &[T]) -> bool
where
    T: PartialEq,
{
    for element in subset {
        let mut found = false;
        for el in set {
            if element == el {
                found = true;
            }
        }
        if !found {
            return found;
        }
    }
    true
}

// Idiomatic rust
// O(N) runtime
fn is_subset<T>(set: &[T], subset: &[T]) -> bool
where
    T: Eq + Hash,
{
    let set = set.iter().collect::<HashSet<&T>>();
    subset.iter().all(|item| set.contains(item))
}

// O(N) runtime
fn is_subset_one_liner<T>(set: &[T], subset: &[T]) -> bool
where
    T: PartialEq,
{
    subset.iter().all(|item| set.contains(item))
}

// Write a function that returns the intersection of two arrays
// Crude Approach
// O(N * M) runtime
fn intersection<T>(first: &[T], second: &[T]) -> HashSet<T>
where
    T: Copy + Eq + Hash,
{
    let (smaller, bigger) = if first.len() < second.len() {
        (first, second)
    } else {
        (second, first)
    };
    let mut store = HashSet::with_capacity(smaller.len());
    for item in smaller {
        if bigger.contains(item) && !store.contains(item) {
            store.insert(*item);
        }
    }
    store
}

// Idiomatic Approach
// O(N) runtime
fn intersection_idiomatic<T>(first: &[T], second: &[T]) -> HashSet<T>
where
    T: Copy + Eq + Hash,
{
    first
        .iter()
        .copied()
        .collect::<HashSet<T>>()
        .intersection(&second.iter().copied().collect::<HashSet<T>>())
        .copied()
        .collect()
}

// Write a function that returns the first duplicate character in a list of characters
fn ret_first_duplicate(word: &[char]) -> Option<char> {
    let mut set = HashSet::new();
    for c in word {
        match set.contains(c) {
            true => return Some(*c),
            false => set.insert(c),
        };
    }
    None
}

// Write a function that returns the missing letter
// from a word that contains all the alphabets
fn missing_letter(word: &str) -> Option<char> {
    let letters = 'a'..='z';
    let set = word.chars().collect::<HashSet<char>>();
    for c in letters {
        if !set.contains(&c) {
            return Some(c);
        }
    }
    None
}

// Write a function that returns the first character without a duplicate in a word
fn first_non_duplicate(word: &str) -> Option<char> {
    let mut map: HashMap<char, i32> = HashMap::new();
    let chars: Vec<char> = word.chars().collect();
    for c in &chars {
        let value = map.entry(*c).or_default();
        *value += 1;
    }
    for c in chars {
        if map[&c] == 1 {
            return Some(c);
        }
    }
    None
}

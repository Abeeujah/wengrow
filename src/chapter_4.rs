use std::{collections::HashSet, i32};

fn check_duplicate(list: &[i32]) -> bool {
    let mut set = HashSet::new();
    for i in list.iter() {
        match set.insert(i) {
            true => continue,
            false => return true,
        }
    }
    false
}

fn check_greatest_number(list: &[i32]) -> i32 {
    let mut min = i32::MIN;
    for i in list {
        if *i > min {
            min = *i;
        }
    }
    min
}

fn bubble_sort<T>(mut list: Vec<T>) -> Vec<T>
where
    T: Ord + Copy,
{
    let mut sort_iter = list.len() - 1;
    let mut sorted = false;

    while !sorted {
        sorted = true;
        for i in 0..sort_iter {
            if list[i] > list[i + 1] {
                let temp = list[i];
                list[i] = list[i + 1];
                list[i + 1] = temp;
                sorted = false;
            }
        }
        if sort_iter == 0 {
            break;
        }
        sort_iter -= 1;
    }
    list
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        assert_eq!(
            bubble_sort(vec![65, 55, 45, 35, 25, 15, 10]),
            vec![10, 15, 25, 35, 45, 55, 65]
        );
    }

    #[test]
    fn test_check_duplicate() {
        assert!(check_duplicate(&[1, 1]));
    }

    #[test]
    fn test_check_greatest_number() {
        assert_eq!(check_greatest_number(&[1, 2, 3]), 3);
    }
}

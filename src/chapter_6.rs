use std::cmp::Ordering;

fn insertion_sort<T>(mut list: Vec<T>) -> Vec<T>
where
    T: Ord + Copy,
{
    for i in 1..list.len() {
        let temp_value = list[i];
        let mut position = i;

        while position > 0 && list[position - 1] > temp_value {
            list[position] = list[position - 1];
            position -= 1;
        }
        list[position] = temp_value;
    }
    list
}

fn intersection<T>(list: Vec<T>, arr: Vec<T>) -> Vec<T>
where
    T: Ord + Copy,
{
    let mut store: Vec<T> = Vec::new();
    for i in list {
        for j in &arr {
            match i.cmp(&j) {
                Ordering::Equal => {
                    store.push(i);
                    break;
                }
                _ => continue,
            };
        }
    }
    store
}

fn contains_x(word: &str) -> bool {
    for i in word.chars() {
        if i == 'X' {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        assert_eq!(vec![1, 2, 3, 4, 7], insertion_sort(vec![4, 2, 7, 1, 3]));
        assert_eq!(vec![1, 2, 5, 7, 9], insertion_sort(vec![5, 2, 9, 1, 7]));
    }

    #[test]
    fn test_intersection() {
        assert_eq!(vec![2, 4], intersection(vec![1, 2, 3, 4], vec![2, 4, 6]));
    }

    #[test]
    fn test_contains_x() {
        assert!(!contains_x(""));
        assert!(contains_x("X"));
        assert!(!contains_x("call"));
    }
}

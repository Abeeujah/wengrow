fn selection_sort<T>(mut list: Vec<T>) -> Vec<T>
where
    T: Ord + Copy,
{
    let size = list.len();
    for i in 0..size - 1 {
        let mut smallest_index = i;
        for j in i + 1..size {
            if list[j] < list[smallest_index] {
                smallest_index = j;
            }
        }
        if smallest_index != i {
            let temp = list[i];
            list[i] = list[smallest_index];
            list[smallest_index] = temp;
        }
    }
    list
}

#[cfg(test)]
mod tests {
    use crate::chapter_5::selection_sort;

    #[test]
    fn test_selection_sort() {
        assert_eq!(
            selection_sort(vec![65, 55, 45, 35, 25, 15, 10]),
            vec![10, 15, 25, 35, 45, 55, 65]
        );
    }
}

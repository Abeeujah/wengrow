use std::cmp::Ordering;

pub fn linear_search<T>(haystack: &[T], needle: T) -> Option<usize>
where
    T: Ord,
{
    for (index, element) in haystack.iter().enumerate() {
        if *element == needle {
            return Some(index);
        }
    }
    None
}

pub fn binary_search<T>(haystack: &[T], needle: T) -> Option<usize>
where
    T: Ord,
{
    let mut lower_bound = 0;
    let mut upper_bound = haystack.len() - 1;

    while lower_bound <= upper_bound {
        let mid_point = (lower_bound + upper_bound) >> 1;
        let mid_value = &haystack[mid_point];

        match mid_value.cmp(&needle) {
            Ordering::Less => {
                lower_bound = mid_point + 1;
            }
            Ordering::Greater => {
                upper_bound = mid_point - 1;
            }
            Ordering::Equal => return Some(mid_point),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::chapter_2::{binary_search, linear_search};

    #[test]
    fn test_linear_search() {
        assert_eq!(linear_search(&[2, 4, 6, 8, 10, 12, 13], 12), Some(5));
        assert_eq!(linear_search(&[2, 4, 6, 8, 10, 12, 13], 40), None);
    }

    #[test]
    fn test_binary_search() {
        assert_ne!(binary_search(&[2, 4, 6, 8, 10, 12, 13], 12), None);
        assert_eq!(binary_search(&[2, 4, 6, 8, 10, 12, 13], 40), None);
    }
}

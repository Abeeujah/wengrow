fn is_prime(number: i32) -> bool {
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    true
}

fn is_leap_year(year: u32) -> bool {
    match year % 100 == 0 {
        true => year % 400 == 0,
        false => year % 4 == 0,
    }
}

fn sum_list<T>(list: Vec<T>) -> T
where
    T: core::iter::Sum,
{
    list.into_iter().sum()
}

fn chess_board_spaces(grains: i32) -> i32 {
    let mut chess_spaces = 1;
    let mut grains_placed = 1;

    while grains_placed < grains {
        grains_placed *= 2;
        chess_spaces += 1;
    }

    chess_spaces
}

fn copy_starts_with<'a>(words: &[&str], start: char) -> Vec<String> {
    let mut selected_words = Vec::new();
    for word in words {
        if word.starts_with(start) {
            selected_words.push(word.to_string());
        }
    }
    selected_words
}

fn list_median(list: &[i32]) -> i32 {
    let size = list.len();
    let mid_point = (size - 1) >> 1;
    match size % 2 == 0 {
        true => (list[mid_point - 1] + list[mid_point + 1]) >> 1,
        false => list[mid_point],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(17));
        assert!(!is_prime(14));
    }

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2100));
    }

    #[test]
    fn test_sum_list() {
        assert_eq!(sum_list(vec![1, 2, 3, 4]), 10);
        assert_ne!(sum_list(vec![1, 2, 3, 4]), 100);
    }

    #[test]
    fn test_chess_board_spaces() {
        assert_eq!(chess_board_spaces(8), 4);
    }

    #[test]
    fn test_list_median() {
        assert_eq!(list_median(&[1, 2, 3, 4]), 2);
        assert_eq!(list_median(&[1, 2, 3, 4, 4]), 3);
    }
}

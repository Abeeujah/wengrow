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

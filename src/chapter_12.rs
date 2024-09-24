use std::collections::HashMap;

pub fn max<T>(list: &[T]) -> Option<T>
where
    T: PartialOrd + Copy,
{
    if list.is_empty() {
        return None;
    }
    if list.len() == 1 {
        return Some(list[0]);
    }
    let max = max(&list[1..]);
    if list[0] > max.expect("Cannot be null") {
        return Some(list[0]);
    }
    return max;
}

pub fn fib(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fib(n - 2) + fib(n - 1),
    }
}

pub fn fib_memoized(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 {
        return n;
    }

    if let Some(&result) = memo.get(&n) {
        return result;
    }

    let result = fib_memoized(n - 1, memo) + fib_memoized(n - 2, memo);
    memo.insert(n, result);
    result
}

// fib(n-2) + fib(n-1)
pub fn fib_iter(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let mut store = &mut [0, 1];
    let mut a = 0;
    let mut b = 1;
    for _ in 1..n {
        let mut temp = a + b;
        a = b;
        b = temp;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_iter() {
        assert_eq!(fib_iter(13), 233);
    }
}

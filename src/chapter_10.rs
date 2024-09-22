use std::fs;

// write a countdown from 10
fn countdown(n: u8) {
    for i in 0..=n {
        println!("{i}");
    }
}

fn recursive_countdown(n: u8) {
    if n == 0 {
        return;
    }
    println!("{n}");
    recursive_countdown(n - 1);
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn recursive_factorial(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    n * factorial(n - 1)
}

fn recursive_find_directories(dir: &str) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(dir)?;
    let mut entries: Vec<_> = entries.filter_map(|entry| entry.ok()).collect();
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        match path.is_file() {
            true => println!("{}", path.file_name().unwrap().to_str().unwrap()),
            false => recursive_find_directories(path.to_str().unwrap())?,
        }
    }
    Ok(())
}

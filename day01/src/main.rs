use std::io;


fn get_number_from_line(line: &String) -> u64 {
    let first_numeric: char = match line.find(char::is_numeric) {
        None => {
            panic!("No numerics on line '{}'!", line);
        },
        Some(val) => {
            char::from_u32(line.as_bytes()[val].into()).unwrap()
        },
    };
    let last_numeric: char = match line.rfind(char::is_numeric) {
        None => {
            panic!("Somehow no numerics on line '{}'?!", line);
        },
        Some(val) => {
            char::from_u32(line.as_bytes()[val].into()).unwrap()
        },
    };

    return format!("{}{}", first_numeric, last_numeric).parse().unwrap();
}

fn main() {
    let lines = io::stdin().lines();
    let mut sum: u64 = 0;

    for line in lines {
        let line = match line {
            Ok(l) => l,
            Err(err) => {
                panic!("Error while reading stdin: {}", err);
            }
        };

        if line.is_empty() {
            continue;
        }

        let number = get_number_from_line(&line);
        println!("Line '{}', number {}", line, number);

        sum += number;
    }

    println!("Sum of all lines' numbers: {}", sum);
}

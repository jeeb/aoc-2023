use std::{io, collections::HashMap};

fn get_first_char_of_numeric_or_numeric_string(line: &String, reverse: bool) -> char {
    let possible_descriptive_strings = HashMap::from([
        ("one",   '1'),
        ("two",   '2'),
        ("three", '3'),
        ("four",  '4'),
        ("five",  '5'),
        ("six",   '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine",  '9'),
    ]);

    let mut minmaxima: Option<usize> = None;
    let mut minmaxima_char: Option<char> = None;

    let first_numeric = if reverse {
        line.rfind(char::is_numeric)
    } else {
        line.find(char::is_numeric)
    };
    for entry in possible_descriptive_strings {
        let str_loc = match if reverse {
            line.rfind(entry.0)
        } else {
            line.find(entry.0)
        } {
            None => {
                continue
            },
            Some(l) => l,
        };

        match minmaxima {
            None => {
                minmaxima = Some(str_loc);
                minmaxima_char = Some(entry.1);
            }
            Some(s) => {
                if (!reverse && str_loc >= s) ||
                   (reverse && str_loc <= s)  {
                    continue;
                }

                minmaxima = Some(str_loc);
                minmaxima_char = Some(entry.1);
            }
        };
    }

    if first_numeric.is_none() && minmaxima.is_none() {
        panic!("No numerics or textual numerics on line '{}'!", line);
    }

    match first_numeric {
        None => {
            return minmaxima_char.unwrap();
        },
        Some(l) => {
            let first_numeric_char = char::from_u32(line.as_bytes()[l].into()).unwrap();
            match minmaxima {
                None => {
                    return first_numeric_char;
                }
                Some(m) => {
                    if (!reverse && l < m) ||
                       (reverse && l > m) {
                        return first_numeric_char;
                    } else {
                        return minmaxima_char.unwrap();
                    }
                }
            }
        }
    }

}

fn get_number_from_line(line: &String) -> u64 {
    let first_numeric = get_first_char_of_numeric_or_numeric_string(line, false);
    let last_numeric = get_first_char_of_numeric_or_numeric_string(line, true);

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

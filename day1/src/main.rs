use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new(); 
    
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    
    result
}

fn numbers_in_line(line: &str) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut changed_line = line;
    let mut start_index = 0;

    let mut end = false;
    while !(end) {
        end = true;
        for word in NUMBERS_ALPHA {
            if changed_line.contains(word) {
                end = false;
                let word_index = changed_line.find(word).unwrap();
                result.push((NUMBERS_ALPHA.iter().position(|r| r == word).unwrap() + 1, start_index + word_index));
                println!("oui");
            }
        }
        for word in NUMBERS {
            if changed_line.contains(word) {
                end = false;
                let word_index = changed_line.find(word).unwrap();
                result.push((NUMBERS.iter().position(|r| r == word).unwrap() + 1, start_index + word_index));
                println!("oui");
            }
        }
        result.sort_by(|a, b| a.1.cmp(&b.1));
        println!("{:?} {}", result, changed_line);
        if changed_line.len() >= result.last().unwrap().1 + 1 {
            (_, changed_line) = changed_line.split_at(result.last().unwrap().1 + 1);
            start_index += result.last().unwrap().1 + 1;
        }
        else {
            end = true;
        }
    
    }
    result
}

const NUMBERS_ALPHA: &[&str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const NUMBERS: &[&str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn main() {
    let lines: Vec<String> = read_lines("data.txt");
    let mut sum = 0;
    
    for line in lines {
        let mut found = false;
        let mut last = 0;
        for char in line.chars() {
            if char.is_digit(10) {
                if !(found) {
                    last = char.to_digit(10).unwrap();
                    sum += 10*last;
                    found = true;
                }
                else {
                    last = char.to_digit(10).unwrap();
                }
            }
        }
        sum += last
    }

    let lines: Vec<String> = read_lines("data.txt");
    let mut sum = 0;
    
    for line in lines {
        let numbers = numbers_in_line(&line);
        let qty = (numbers[0].0)*10 + numbers.last().unwrap().0;
        sum += qty;
        println!("{} {:?} {}", line, numbers, qty);
    }
    println!("{}", sum);
}

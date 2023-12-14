use std::fs::read_to_string;

#[derive(Debug)]
struct NumberPosition {
    number: u32,
    start: (usize, usize),
    end: (usize, usize)
}

fn is_symbol(char: char) -> bool {
    match char {
        '0'..='9' => false,
        '.' => false,
        _ => true,
    }
}

fn is_number(char: char) -> bool {
    match char {
        '0'..='9' => true,
        _ => false,
    }
}

fn is_gear(char: char) -> bool {
    match char {
        '*' => true,
        _ => false,
    }
}

fn is_gear_adjacent(number: &NumberPosition, gear: (usize, usize)) -> bool {
    for x in number.start.0..=number.end.0 {
        let y = number.start.1;
        if (x <= gear.0 + 1 && x >= gear.0 - 1) && (y <= gear.1 + 1 && y >= gear.1 - 1) {
            return true;
        }
    }
    false
}

fn has_adjacent_symbol((x, y): (usize, usize), symbols: &Vec<(usize, usize)>) -> bool {
    for (x2, y2) in symbols {
        if (x <= x2 + 1 && x >= x2 - 1) && (y <= y2 + 1 && y >= y2 - 1) {
            return true;
        }
    }
    false
}

fn main() {

    let data = read_to_string("data.txt").expect("Unable to read file");
    let mut symbols = Vec::new();
    for (y, line) in data.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if is_symbol(char) {
                symbols.push((x, y));
            }
        }
    }

    let mut sum = 0;

    for (y, line) in data.lines().enumerate() {
        let mut adjacent_to_symbol = false;
        let mut current_number = 0;
        for (x, char) in line.chars().enumerate() {
            if is_number(char) {
                if has_adjacent_symbol((x, y), &symbols) {
                    adjacent_to_symbol = true;
                }
                current_number = current_number * 10 + char.to_digit(10).unwrap();
            } else {
                if adjacent_to_symbol {
                    sum += current_number;
                }
                adjacent_to_symbol = false;
                current_number = 0;
            }
        }
        if adjacent_to_symbol {
            sum += current_number;
        }
    }
    println!("{}", sum);     

    sum = 0;

    let mut gears = Vec::new();
    for (y, line) in data.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if is_gear(char) {
                gears.push((x, y));
            }
        }
    }

    let mut numbers = Vec::new();
    for (y, line) in data.lines().enumerate() {
        let mut current_number = 0;
        let mut start = (0, 0);
        for (x, char) in line.chars().enumerate() {
            if is_number(char) {
                if current_number == 0 {
                    start = (x, y);
                }
                current_number = current_number * 10 + char.to_digit(10).unwrap();
            } else {
                if current_number != 0 {
                    let end = (x - 1, y);
                    numbers.push(NumberPosition {number: current_number, start: start, end: end});
                }
                current_number = 0;
            }
        }
        if current_number != 0 {
            let end = (line.len(), y);
            numbers.push(NumberPosition {number: current_number, start: start, end: end});
        }
    }

    for gear in gears {
        let mut number_iterator = numbers.iter();
        let mut number = number_iterator.next().unwrap();
        let mut current_y = number.start.1;
        let mut part_numbers_count = 0;
        let mut gear_ratio = 1;
        while current_y < gear.1 -1 {
            number = number_iterator.next().unwrap();
            current_y = number.start.1;
        }
        while current_y < gear.1 + 2 {
            let mut number_coords = Vec::new();
            for x in number.start.0..=number.end.0 {
                number_coords.push((x, number.start.1));
            }

            if is_gear_adjacent(number, gear) {
                part_numbers_count += 1;
                println!("{} {}", number.number, part_numbers_count);
                if part_numbers_count <= 2 {
                    gear_ratio *= number.number;
                    println!("{} {}", number.number, gear_ratio);
                }
            }
            
            
            number = if let Some(number) = number_iterator.next() {
                current_y = number.start.1;
                number
            } else {
                current_y = gear.1 + 2;
                &NumberPosition {number: 0, start: (0, 0), end: (0, 0)}
            };
        } 
        if part_numbers_count == 2 {
            sum += gear_ratio;
            println!("final : {} {}", gear_ratio, number.number)
        }
    }
    println!("{}", sum);
}

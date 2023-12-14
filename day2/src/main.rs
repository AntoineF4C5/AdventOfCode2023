use std::fs::read_to_string; 

fn main() {
    let data = read_to_string("data.txt").unwrap();
    let mut sum = 0;
    for game in data.lines() {
        let mut sets: Vec<&str> = Vec::new();
        let raw_game: &str;
        raw_game = game.split(": ").collect::<Vec<&str>>()[1];
        for set in raw_game.split("; ") {
            sets.push(set);
        }

        let mut min_red: i32 = 0;
        let mut min_green: i32 = 0;
        let mut min_blue: i32 = 0;

        for set in sets.iter() {
            let marbles = set.split(", ").collect::<Vec<&str>>();
            let mut set_results: Vec<Vec<&str>> = Vec::new();
            for marble in marbles.iter() {
                set_results.push(marble.split(" ").collect::<Vec<&str>>());
            }
            for result in set_results.iter() {
                match result[1] {
                    "red" => {
                        if result[0].parse::<i32>().unwrap() > min_red {
                            min_red = result[0].parse::<i32>().unwrap();
                        }
                    },
                    "green" => {
                        if result[0].parse::<i32>().unwrap() > min_green {
                            min_green = result[0].parse::<i32>().unwrap();
                        }
                    },
                    "blue" => {
                        if result[0].parse::<i32>().unwrap() > min_blue {
                            min_blue = result[0].parse::<i32>().unwrap();
                        }
                    },
                    _ => {
                        println!("Error: Invalid color");
                    }
                }
            }
        }
        sum += min_red*min_green*min_blue;
        println!("{}", sum)
    }

}

use std::{fs::read_to_string};

struct RaceScore {
    time: u64,
    distance: u64
}

impl RaceScore {
    fn new(time: u64, distance: u64) -> RaceScore {
        RaceScore { time, distance }
    }

    // distance = time_held * time_travelling
    // time = time_held + time_travelling
    // time_held ( time - time_held ) = distance
    // time_held = (time +- sqrt(time² - 4*distance))/2

    // basicly returns the integer BELOW integer needed to have better distance (cuz truncated)
    fn calculate_time_held(&self) -> u64 {
        (self.time as f64 - f64::sqrt((self.time.pow(2)-4*self.distance) as f64)).trunc() as u64/2
    }

    fn count_winning_ways(&self) -> u64 {
        self.time - self.calculate_time_held()*2 -1
    }
}

struct ScoreList {
    list: Vec<RaceScore>
}

impl ScoreList {
    fn new(score_vector: Vec<RaceScore>) -> ScoreList {
        let mut score_list = Vec::new();

        for race_score in score_vector {
            score_list.push(race_score );
        }

        ScoreList {list: score_list}
    }

    fn count_total_ways(self) -> u64 {
        let mut total = 1;
        for score in self.list {
            total *= score.count_winning_ways()
        }
        total
    }
}

fn part_one(data: &String) -> u64 {
    let (_,times_data) = data.lines().nth(0).unwrap().split_once(": ").unwrap();
    let (_, distance_data) = data.lines().nth(1).unwrap().split_once(": ").unwrap();
  
    let times = times_data.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let distances = distance_data.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let mut distance_it = distances.iter();
    let mut score_vec = Vec::new();
    for time in times {
        score_vec.push(RaceScore::new(time, *distance_it.next().unwrap()));
    }

    let score_list = ScoreList::new(score_vec);
    score_list.count_total_ways()
}

// the goal is to reverse first the order of digits to be added, to then add them to the number by getting the unit digit each time 
// ex : number = 123, x = 456
// reversed_x = 654
// step 0: number = 123, x = 654
// step 1: number = 1234, x = 65
// step 2: number = 12345, x = 6
// step 3: number = 123456, x = 0
fn add_digits_left(number: u64, x: u64) -> u64 {
    let mut result = number;
    let mut x = x*10+1; // to tackle cases where x ends by 0: if x =120, reversed_x would equal 021, which is 21, and gives 12 when reversed back
    let mut reversed_x = 0;


    while x > 0 {
        let digit = x % 10;
        x = x / 10;
        reversed_x = reversed_x* 10 + digit;
    }
    while reversed_x > 0 {
        let digit = reversed_x % 10;
        reversed_x = reversed_x /10;
        result = result*10 + digit
    }
    result/10   // we get rid of the extra digit we added at the beginning
}        

fn part_two(data: &String) -> u64 {
    let (_,times_data) = data.lines().nth(0).unwrap().split_once(": ").unwrap();
    let (_, distance_data) = data.lines().nth(1).unwrap().split_once(": ").unwrap();
    
    let mut time = 0;
    let mut distance = 0;

    for recovered_time in times_data.split_whitespace().map(|x| x.parse::<u64>().unwrap()) {
        time = add_digits_left(time, recovered_time);
    }
    for recovered_distance in distance_data.split_whitespace().map(|x| x.parse::<u64>().unwrap()) {
        distance = add_digits_left(distance, recovered_distance);
    }

    let race_score = RaceScore::new(time, distance);
    race_score.count_winning_ways()
}

fn main() {
    let data = &read_to_string("data.txt").unwrap();

    let part_one_result = part_one(data);
    println!("Part one result: {}", part_one_result);
    
    let part_two_result = part_two(data);
    println!("Part two result: {}", part_two_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_time_held_good() {
        let race_score = RaceScore::new(7, 9);
        assert_eq!(race_score.calculate_time_held(), 1);
    }

    #[test]
    fn is_counting_ways_good() {
        let race_score = RaceScore::new(7, 9);
        assert_eq!(race_score.count_winning_ways(), 4);

        let race_score = RaceScore::new(15, 40);
        assert_eq!(race_score.count_winning_ways(), 8);

        let race_score = RaceScore::new(30, 200);
        assert_eq!(race_score.count_winning_ways(), 9);

        let race_score = RaceScore::new(71530, 940200);
        assert_eq!(race_score.count_winning_ways(), 71503);
    }

    #[test]
    fn is_total_accurate() {
        let mut scores_vec = Vec::new();
        scores_vec.push(RaceScore::new(7, 9));
        scores_vec.push(RaceScore::new(15, 40));
        scores_vec.push(RaceScore::new(30, 200));

        let race_list = ScoreList::new(scores_vec);
        assert_eq!(race_list.count_total_ways(), 288);
    }

    #[test]
    fn is_adding_digits_good() {
        assert_eq!(add_digits_left(0, 0), 0);
        assert_eq!(add_digits_left(0, 1), 1);
        assert_eq!(add_digits_left(0, 12), 12);
        assert_eq!(add_digits_left(1, 1), 11);
        assert_eq!(add_digits_left(11, 1), 111);
        assert_eq!(add_digits_left(12, 12), 1212);
    }
}
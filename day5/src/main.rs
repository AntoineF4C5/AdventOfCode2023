use std::fs::read_to_string;

struct InterfaceMap {
    name: String,
    destination_map: Vec<(u64, u64, u64)>
}

impl InterfaceMap {
    fn new(name: String) -> InterfaceMap {
        InterfaceMap {
            name: name,
            destination_map: Vec::new()
        }
    }

    fn add_range(&mut self, destination_start: u64, source_start: u64, range_length: u64) {
        self.destination_map.push((destination_start, source_start, range_length));
    }

    fn sort(&mut self) {
        self.destination_map.sort_by(|(_, a, _), (_, b, _)| a.cmp(&b));
    }

    fn find_destination(&self, source: u64) -> u64 {
        for (destination_start, source_start, range_length) in &self.destination_map {
            if source >= *source_start && source < *source_start + *range_length {
                return destination_start + (source - source_start);
            }
        }
        return source;
    }

    fn find_destination_with_range(&self, source: (u64, u64)) -> Vec<(u64, u64)> {
        let mut current_range = source;
        let mut result = Vec::new();
        for (destination_start, source_start, range_length) in &self.destination_map {
            if is_a_overlapping_b(current_range, (*source_start, *range_length)) {
                if current_range.0 <= *source_start { 
                    let range_before_start = *source_start - current_range.0;
                    result.push((current_range.0, range_before_start));
                    current_range = (*source_start, current_range.1 - range_before_start);
                }
                if current_range.0 + current_range.1 <= *source_start + *range_length {
                    result.push((*destination_start + (current_range.0 - source_start), current_range.1));
                    return result;
                } else {
                    let consumed_range = *source_start + *range_length - current_range.0;

                    result.push((*destination_start + (current_range.0 - source_start), consumed_range));
                    current_range = (*source_start + *range_length, current_range.1 - consumed_range);
                }
                println!();
            } else if current_range.0 <= *source_start {
                result.push(current_range);
                return result;
            }
        }
        result.push(current_range);
        result
    }
}

struct InterfaceList {
    interfaces: Vec<InterfaceMap>
}

impl InterfaceList {
    fn new() -> InterfaceList {
        InterfaceList {
            interfaces: Vec::new()
        }
    }

    fn add_interface(&mut self, interface: InterfaceMap) {
        self.interfaces.push(interface);
    }
}

fn is_a_overlapping_b(a: (u64, u64), b: (u64, u64)) -> bool {
    if a.0 + a.1 <= b.0 || b.0 + b.1 <= a.0 {
        return false;
    }
    return true;
}

fn sort_ranges(ranges: &mut Vec<(u64, u64)>) {
    ranges.sort_by(|(a, _), (b, _)| a.cmp(&b));
}

fn merge_ranges(ranges: &mut Vec<(u64, u64)>) {
    let mut i = 0;
    while i < ranges.len() - 1 {
        if ranges[i].0 + ranges[i].1 == ranges[i + 1].0 {
            let new_range = (ranges[i].0, ranges[i].1 + ranges[i + 1].1);
            ranges.remove(i);
            ranges.remove(i);
            ranges.insert(i, new_range);
        } else if ranges[i].0 + ranges[i].1 > ranges[i + 1].0 {
            println!("ERRRRRRROOOOOOOOOOOOOOR SHOULDNT HAPPEN");
            return;
        } else {
            i += 1;
        }
    }
}

fn fill_interfaces(maps: &mut InterfaceList, data: &mut std::str::Split<'_, &str>) { 
    while let Some(interface_string) = data.next() {
        let mut name_splitter = interface_string.splitn(2, " map:\n");
        let name = name_splitter.next().unwrap().to_string();
        let mut interface = InterfaceMap::new(name);

        for line in name_splitter.next().unwrap().lines() {
            let mut content = line.split_whitespace();
            let destination_start = content.next().unwrap().parse::<u64>().unwrap();
            let source_start = content.next().unwrap().parse::<u64>().unwrap();
            let range_length = content.next().unwrap().parse::<u64>().unwrap();
    
            interface.add_range(destination_start, source_start, range_length);
        }
        interface.sort();
        maps.add_interface(interface);
    }
}

fn part_one(data: String) {
    let mut data = data.split("\n\n");
    let seeds = data.next().unwrap().split(": ").nth(1).unwrap();
    let seeds = seeds.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let mut maps = InterfaceList::new();

    fill_interfaces(&mut maps, &mut data);

    let mut destinations = Vec::new();
    for seed in seeds {
        let mut checkpoint = seed;
        for interface in &maps.interfaces {
            checkpoint = interface.find_destination(checkpoint);
        }
        destinations.push(checkpoint);
    }
    if let Some(result) = destinations.iter().min() {
        println!("Part one: {}", result);
        return;
    }
    println!("Part one: No result");
}

fn part_two(data: String) {
    let mut data = data.split("\n\n");
    let seeds_string = data.next().unwrap().split(": ").nth(1).unwrap();
    
    let mut seeds: Vec<(u64, u64)> = Vec::new();
    let mut seed_start: u64 = 0;
    for (i, number) in seeds_string.split_whitespace().enumerate() {
        match i % 2 {
            0 => seed_start = number.parse::<u64>().unwrap(),
            1 => seeds.push((seed_start, number.parse::<u64>().unwrap())),
            _ => panic!("This should never happen")
        }       
    }

    let mut maps = InterfaceList::new();
    fill_interfaces(&mut maps, &mut data);

    let mut checkpoint = seeds.clone();
    println!("entry :\n{:?}", checkpoint);
    sort_ranges(&mut checkpoint);
    println!();
    for interface in &maps.interfaces {
        println!("interface : {:?}", interface.destination_map);
        let mut new_checkpoint = Vec::new();
        for range in &checkpoint {
            new_checkpoint.append(&mut interface.find_destination_with_range(*range));
        }
        sort_ranges(&mut new_checkpoint);
        checkpoint = new_checkpoint;
        println!("entry :\n{:?}", checkpoint);
        println!();
    }
    println!("{:?}", checkpoint);
    
    let mut seed_ranges_sum = 0;
    for seed_range in seeds {
        seed_ranges_sum += seed_range.1;
    }

    let mut total_ranges_sum = 0;
    for range in checkpoint {
        total_ranges_sum += range.1;
    }

    println!("Part two: {}", seed_ranges_sum as i64 - total_ranges_sum as i64)
    
}

fn main() {

    let data = read_to_string("data.txt").expect("Unable to read file");

    part_one(data);

    let data = read_to_string("data.txt").expect("Unable to read file");

    part_two(data);

}

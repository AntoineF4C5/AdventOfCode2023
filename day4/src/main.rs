use std::fs::read_to_string;

const WINNING: usize = 10;
const OBTAINED_NUMBERS: usize = 25;
const NUMBER_OF_CARDS: usize = 198;

#[derive(Debug)]

#[derive(Copy, Clone, Default)]
struct ScratchCard {
    id: u32,
    winning_numbers: [u32; WINNING],
    obtained_numbers: [u32; OBTAINED_NUMBERS]
}

impl std::fmt::Display for ScratchCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}, winning: {:?}, obtained: {:?}",self.id, self.winning_numbers, self.obtained_numbers)
    }
}

impl ScratchCard {
    fn new(id: u32, line_string: &str) -> ScratchCard {
        let mut winning_numbers: [u32; WINNING] = [0; WINNING];
        let mut obtained_numbers: [u32; OBTAINED_NUMBERS] = [0; OBTAINED_NUMBERS];
        
        let winning_numbers_string = line_string.split('|').nth(0).unwrap().split(':').nth(1).unwrap();
        let obtained_numbers_string = line_string.split('|').nth(1).unwrap();
        
        for (i, number) in winning_numbers_string.split_whitespace().enumerate() {
            winning_numbers[i] = number.parse::<u32>().unwrap();
        }
        
        for (i, number) in obtained_numbers_string.split_whitespace().enumerate() {
            obtained_numbers[i] = number.parse::<u32>().unwrap();
        }
        
        ScratchCard {
            id,
            winning_numbers,
            obtained_numbers
        }
    }
    
    // computes value of a card according to part 1 of the problem
    fn get_value(&self) -> u32 {
        let mut value: Option<u32> = None;
        for obtained_number in self.obtained_numbers.iter() {
            if self.winning_numbers.contains(obtained_number) {
                if let Some(v) = value {
                    value = Some(v*2);
                } else {
                    value = Some(1);
                }
            }
        }
        value.unwrap_or(0)
    }

    fn get_score(&self) -> u32 {
        let mut score: u32 = 0;
        for obtained_number in self.obtained_numbers.iter() {
            if self.winning_numbers.contains(obtained_number) {
                score += 1;
            }
        }
        score
    }
}

#[derive(Copy, Clone)]
struct CardDeck {
    cards: [ScratchCard; NUMBER_OF_CARDS]
}

impl CardDeck {
    fn init() -> CardDeck {
        let mut cards: [ScratchCard; NUMBER_OF_CARDS] = [ScratchCard::default(); NUMBER_OF_CARDS];
        let data = read_to_string("data.txt").expect("Unable to read file");
        for (i, line) in data.lines().enumerate() {
            let scratch_card = ScratchCard::new((i + 1) as u32, line);
            println!("{:?}", scratch_card);
            cards[i] = scratch_card;
        }
        CardDeck {
            cards
        }
    }

    fn get_total_cards(self) -> u32 {
        let mut total = 0;
        let mut quantities = [1; NUMBER_OF_CARDS];
        for (id, card) in self.cards.iter().enumerate() {
            total += quantities[id];
            let score = card.get_score();
            println!("Card {:?} has score {}", card, score );
            for i in 0..score {
                quantities[id + i as usize + 1] += quantities[id];
            }
        }
        total
    }
}

fn main() {

    let data = read_to_string("data.txt").expect("Unable to read file");
    let mut sum = 0;


    for (i, line) in data.lines().enumerate() {
        let scratch_card = ScratchCard::new((i + 1) as u32, line);
        sum += scratch_card.get_value();
    }

    println!("Total value: {}", sum);

    let card_deck = CardDeck::init();

    println!("Total cards: {}", card_deck.get_total_cards());

}

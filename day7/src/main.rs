use std::fs::read_to_string;
use std::cmp::{Ordering, PartialOrd, Ord, PartialEq, Eq};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack, 
    Queen,
    King,
    Ace
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum CardVariant {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace
}

/* 
Generic trait for initializing and finding the hand type
*/
trait CardTrait<T> {
    fn cards_in_vec(cards_string: String) -> Vec<T>;
    fn sort_cards_in_vectors(cards: &Vec<T>) -> Vec<u32>;
}


/* 
Specific logic for initializing and finding the hand type with Card hand
*/
impl CardTrait<Card> for Card {
    fn cards_in_vec(cards_string: String) -> Vec<Card> {
        let mut cards = Vec::new();
        for char in cards_string.chars() {
            let card = match char {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!("Invalid card")
            };
            cards.push(card);
        }
        cards
    }

    fn sort_cards_in_vectors(cards: &Vec<Card>) -> Vec<u32> {
        let mut considered_cards = Vec::new();
        let mut qty_of_cards = Vec::new();
        for card in cards {
            if let Some(index) = considered_cards.iter().position(|item| item == card) {
                qty_of_cards[index] += 1;
            } else {
                considered_cards.push(*card);
                qty_of_cards.push(1);
            }
        }
        qty_of_cards
    }
}

/* 
Specific logic for initializing and finding the hand type with CardVariant hand
*/
impl CardTrait<CardVariant> for CardVariant {
    fn cards_in_vec(cards_string: String) -> Vec<CardVariant> {
        let mut cards = Vec::new();
        for char in cards_string.chars() {
            let card = match char {
                '2' => CardVariant::Two,
                '3' => CardVariant::Three,
                '4' => CardVariant::Four,
                '5' => CardVariant::Five,
                '6' => CardVariant::Six,
                '7' => CardVariant::Seven,
                '8' => CardVariant::Eight,
                '9' => CardVariant::Nine,
                'T' => CardVariant::Ten,
                'J' => CardVariant::Joker,
                'Q' => CardVariant::Queen,
                'K' => CardVariant::King,
                'A' => CardVariant::Ace,
                _ => panic!("Invalid card")
            };
            cards.push(card);
        }
        cards
    }

    fn sort_cards_in_vectors(cards: &Vec<CardVariant>) -> Vec<u32> {
        let mut considered_cards = Vec::new();
        let mut qty_of_cards = Vec::new();
        let mut joker_qty = 0;
        for card in cards {
            if card == &CardVariant::Joker {
                joker_qty += 1;
            } else if let Some(index) = considered_cards.iter().position(|item| item == card) {
                qty_of_cards[index] += 1;
            } else {
                considered_cards.push(*card);
                qty_of_cards.push(1);
            }
        }
        if joker_qty == 0 {
            return qty_of_cards;
        }
        if joker_qty == 5 { 
            qty_of_cards.push(5);
            return qty_of_cards;
        }
        let most_present_card_index = qty_of_cards.iter().position(|x| x == qty_of_cards.iter().max().unwrap()).unwrap();
        qty_of_cards[most_present_card_index] += joker_qty;
    
        qty_of_cards
    }
}

/* 
Generic hand struct
*/
#[derive(PartialEq, Eq, Debug)]
struct Hand<T: Clone + Eq + Ord> {
    cards: Vec<T>,
    bid: u32,
    hand_type: HandType
}

/*
Impl custom comparison rules
*/
impl<T: PartialEq + Clone + Ord> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Hand<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq + Clone + Ord> Ord for Hand<T> {
    fn cmp(&self, other: &Hand<T>) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        } else {
            let self_cards = self.cards.clone();
            let other_cards = other.cards.clone();
            for (a, b) in self_cards.iter().zip(other_cards.iter()) {
                if a != b {
                    return a.cmp(b);
                }
            }
            return Ordering::Equal;
        }
    }
}

/*
Generic way of initializing a hand, depending of chosen variant
*/
impl<T: CardTrait<T> + Clone + Ord> Hand<T> {
    fn new (cards_string: String, bid: u32) -> Hand<T> {
        let cards = T::cards_in_vec(cards_string);
        let qty_of_cards = T::sort_cards_in_vectors(&cards);
        let hand_type = match qty_of_cards.len() {
            1 => HandType::FiveOfAKind,
            2 => if qty_of_cards.contains(&4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            },
            3 => if qty_of_cards.contains(&3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            },
            4 => HandType::OnePair,
            _ => HandType::HighCard
        };

        Hand { cards, bid, hand_type }
    }
}

fn part_one(data: &String) -> u64 {
    let mut hands = Vec::new();
    for (cards, bid) in data.lines().map(|x| x.split_once(" ").unwrap()) {
        let hand = Hand::<Card>::new(cards.to_string(), bid.parse::<u32>().unwrap());
        hands.push(hand);
    }
    hands.sort();

    let mut total: u64 = 0;
    for (index, hand) in hands.iter().enumerate() {
        total += (index + 1) as u64 * hand.bid as u64;
    }
    total
}

fn part_two(data: &String) -> u64 {
    let mut hands = Vec::new();
    for (cards, bid) in data.lines().map(|x| x.split_once(" ").unwrap()) {
        let hand = Hand::<CardVariant>::new(cards.to_string(), bid.parse::<u32>().unwrap());
        hands.push(hand);
    }
    hands.sort();

    let mut total: u64 = 0;
    for (index, hand) in hands.iter().enumerate() {
        total += (index + 1) as u64 * hand.bid as u64;
    }
    total
}

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("Part one answer: {}", part_one(&data));
    println!("Part two answer: {}", part_two(&data));
}   

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_enum_ordering() {
        assert!(HandType::HighCard < HandType::OnePair);
        assert!(HandType::OnePair < HandType::TwoPair);
        assert!(HandType::TwoPair < HandType::ThreeOfAKind);
        assert!(HandType::ThreeOfAKind < HandType::FullHouse);
        assert!(HandType::FullHouse < HandType::FourOfAKind);
        assert!(HandType::FourOfAKind < HandType::FiveOfAKind);
    }

    #[test]
    fn test_card_enum_ordering() {
        assert!(Card::Two < Card::Three);
        assert!(Card::Three < Card::Four);
        assert!(Card::Four < Card::Five);
        assert!(Card::Five < Card::Six);
        assert!(Card::Six < Card::Seven);
        assert!(Card::Seven < Card::Eight);
        assert!(Card::Eight < Card::Nine);
        assert!(Card::Nine < Card::Ten);
        assert!(Card::Ten < Card::Jack);
        assert!(Card::Jack < Card::Queen);
        assert!(Card::Queen < Card::King);
        assert!(Card::King < Card::Ace);
    }

    #[test]
    fn test_card_initializing() {
        assert_eq!(Card::cards_in_vec(String::from("2")), vec![Card::Two]);
        assert_eq!(Card::cards_in_vec(String::from("34")), vec![Card::Three, Card::Four]);
        assert_eq!(Card::cards_in_vec(String::from("32T3K")), vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]);
        assert_ne!(Card::cards_in_vec(String::from("32T3K")), vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::Queen]);
        assert_ne!(Card::cards_in_vec(String::from("32T3K")), vec![Card::Three, Card::Two, Card::Ten, Card::King, Card::Three]);
    }

    #[test]
    fn test_sort_cards_in_vectors() {
        let cards = Card::cards_in_vec(String::from("32T3K"));
        let qty_of_cards = Card::sort_cards_in_vectors(&cards);
        assert_eq!(qty_of_cards, vec![2, 1, 1, 1]);

        let cards = Card::cards_in_vec(String::from("33333"));
        let packed_hand = Card::sort_cards_in_vectors(&cards);
        assert_eq!(packed_hand, vec![5]);
    }

    #[test]
    fn test_hand_type() {
        let cards =String::from("32T3K");
        let hand = Hand::<Card>::new(cards, 0);
        assert_eq!(hand.hand_type, HandType::OnePair);

        let cards =String::from("33333");
        let hand = Hand::<Card>::new(cards, 0);
        assert_eq!(hand.hand_type, HandType::FiveOfAKind);

        let cards = String::from("33344");
        let hand = Hand::<Card>::new(cards, 0);
        assert_eq!(hand.hand_type, HandType::FullHouse);

        let cards = String::from("3334K");
        let hand = Hand::<Card>::new(cards, 0);
        assert_eq!(hand.hand_type, HandType::ThreeOfAKind);

        let cards =String::from("33344");
        let hand = Hand::<Card>::new(cards, 0);
        assert_eq!(hand.hand_type, HandType::FullHouse);

        let cards = String::from("33344");
        let hand = Hand::<Card>::new(cards, 0);
        assert_eq!(hand.hand_type, HandType::FullHouse);

        let cards = String::from("33344");
        let hand = Hand::<Card>::new(cards, 0);
        assert_eq!(hand.hand_type, HandType::FullHouse);

        let cards = String::from("33344");
        let hand = Hand::<Card>::new(cards, 0);
        assert_eq!(hand.hand_type, HandType::FullHouse);

        let cards = String::from("33344");
        let hand = Hand::<Card>::new(cards, 0);
        assert_eq!(hand.hand_type, HandType::FullHouse);

        let cards = String::from("33344");
        let hand = Hand::<Card>::new(cards, 0);
        assert_eq!(hand.hand_type, HandType::FullHouse);
    }

    #[test]
    fn test_hand_ordering() {
        let hand1 = Hand::<Card>::new(String::from("32T3K"), 0);
        let hand2 = Hand::<Card>::new(String::from("33333"), 0);
        assert!(hand1 < hand2);

        let hand1 = Hand::<Card>::new(String::from("33333"), 0);
        let hand2 = Hand::<Card>::new(String::from("JJJJJ"), 0);
        assert!(hand1 < hand2);

        let hand1 = Hand::<Card>::new(String::from("77888"), 0);
        let hand2 = Hand::<Card>::new(String::from("77788"), 0);
        assert!(hand1 > hand2);
    }

    #[test]
    fn test_hand_vector_sorting() {
        let mut hands = Vec::new();
        hands.push(Hand::<Card>::new(String::from("32T3K"), 0));
        hands.push(Hand::<Card>::new(String::from("33333"), 0));
        hands.push(Hand::<Card>::new(String::from("77888"), 0));
        hands.push(Hand::<Card>::new(String::from("77788"), 0));
        hands.sort();
        assert_eq!(hands[0].hand_type, HandType::OnePair);
        assert_eq!(hands[1].hand_type, HandType::FullHouse);
        assert_eq!(hands[2].hand_type, HandType::FullHouse);
        assert_eq!(hands[3].hand_type, HandType::FiveOfAKind);
        assert_eq!(hands[1].cards, vec![Card::Seven, Card::Seven, Card::Seven, Card::Eight, Card::Eight]);
        assert_eq!(hands[2].cards, vec![Card::Seven, Card::Seven, Card::Eight, Card::Eight, Card::Eight]);
    }

    #[test]
    fn test_variant_card_init() {
        assert_eq!(CardVariant::cards_in_vec(String::from("J")), vec![CardVariant::Joker]);
        assert_eq!(CardVariant::cards_in_vec(String::from("34")), vec![CardVariant::Three, CardVariant::Four]);
        assert_eq!(CardVariant::cards_in_vec(String::from("32T3K")), vec![CardVariant::Three, CardVariant::Two, CardVariant::Ten, CardVariant::Three, CardVariant::King]);
        assert_ne!(CardVariant::cards_in_vec(String::from("32J3K")), vec![CardVariant::Three, CardVariant::Two, CardVariant::Joker, CardVariant::Three, CardVariant::Queen]);
        assert_ne!(CardVariant::cards_in_vec(String::from("32T3K")), vec![CardVariant::Three, CardVariant::Two, CardVariant::Ten, CardVariant::King, CardVariant::Three]);
    }

    #[test]
    fn test_variant_ordering() {
        let hand1 = Hand::<CardVariant>::new(String::from("JJJJJ"), 0);
        let hand2 = Hand::<CardVariant>::new(String::from("KKKKK"), 0);
        assert!(hand1 < hand2);

        let hand1 = Hand::<CardVariant>::new(String::from("JKK2A"), 0);
        let hand2 = Hand::<CardVariant>::new(String::from("KK2A3"), 0);
        assert!(hand1 > hand2);

        let hand1 = Hand::<CardVariant>::new(String::from("2JJJ3"), 0);
        let hand2 = Hand::<CardVariant>::new(String::from("22JJ3"), 0);
        assert!(hand1 < hand2);
    }
}
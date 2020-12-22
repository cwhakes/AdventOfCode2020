use std::collections::{HashSet, VecDeque};
use std::cmp::{Ord, Ordering};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let (deck1, deck2) = process_input(&buf);
    
    let answer = get_answer(&deck1, &deck2);
    let answer2 = get_answer2(&deck1, &deck2);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> (Deck, Deck) {
    let mut iter = input.split("\n\n");

    let deck1 = iter.next().unwrap().strip_prefix("Player 1:\n").unwrap().lines().map(|s| s.parse().unwrap()).collect();
    let deck2 = iter.next().unwrap().strip_prefix("Player 2:\n").unwrap().lines().map(|s| s.parse().unwrap()).collect();

    (Deck(deck1), Deck(deck2))
}

fn get_answer(deck1: &Deck, deck2: &Deck) -> usize {
    let (mut deck1, mut deck2) = (deck1.clone(), deck2.clone());

    while !deck1.0.is_empty() && !deck2.0.is_empty() {
        deck1.combat_round(&mut deck2);
    }

    deck1.score() + deck2.score()
}

fn get_answer2(deck1: &Deck, deck2: &Deck) -> usize {
    let (mut deck1, mut deck2) = (deck1.clone(), deck2.clone());
    
    deck1.recursive_combat(&mut deck2);

    deck1.score() + deck2.score()
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Deck(VecDeque<usize>);

impl Deck {
    fn combat_round(&mut self, other: &mut Deck) {
        let self_card = self.0.pop_front().unwrap();
        let other_card = other.0.pop_front().unwrap();

        let winner;
        let winning_card;
        let losing_card;

        match self_card.cmp(&other_card) {
            Ordering::Less => {
                winner = other;
                winning_card = other_card;
                losing_card = self_card;
            }
            Ordering::Greater => {
                winner = self;
                winning_card = self_card;
                losing_card = other_card;
            }
            Ordering::Equal => panic!()
        }

        winner.0.push_back(winning_card);
        winner.0.push_back(losing_card);
    }

    fn recursive_combat(&mut self, other: &mut Deck) -> Winner {
        let mut previous_rounds: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();

        while !self.0.is_empty() && !other.0.is_empty() {
            let deck_fingerprint = (self.0.iter().cloned().collect(), other.0.iter().cloned().collect());
            if !previous_rounds.insert(deck_fingerprint) {return Winner::Player1}

            let self_card = self.0.pop_front().unwrap();
            let other_card = other.0.pop_front().unwrap();

            let winner = if self.0.len() >= self_card && other.0.len() >= other_card {
                self.subdeck(self_card).recursive_combat(&mut other.subdeck(other_card))
            } else {
                match self_card.cmp(&other_card) {
                    Ordering::Less => Winner::Player2,
                    Ordering::Greater => Winner::Player1,
                    Ordering::Equal => panic!()
                }
            };

            let winning_deck;
            let winning_card;
            let losing_card;

            match winner {
                Winner::Player1 => {
                    winning_deck = &mut *self;
                    winning_card = self_card;
                    losing_card = other_card;
                }
                Winner::Player2 => {
                    winning_deck = &mut *other;
                    winning_card = other_card;
                    losing_card = self_card;
                }
            }

            winning_deck.0.push_back(winning_card);
            winning_deck.0.push_back(losing_card);
        }

        if self.0.is_empty() {
            Winner::Player2
        } else {
            Winner::Player1
        }
    }

    fn score(&self) -> usize {
        let len = self.0.len();
        
        let mut count = 0;
        for (index, card) in self.0.iter().enumerate() {
            count += (len - index) * card;
        }
        count
    }

    fn subdeck(&self, num: usize) -> Self {
        Deck(self.0.iter().take(num).cloned().collect())
    }
}

enum Winner {
    Player1,
    Player2,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer() {
        let (deck1, deck2) = process_input(INPUT);
        assert_eq!(306, get_answer(&deck1, &deck2));
    }

    #[test]
    fn test_infinite() {
        let (deck1, deck2) = process_input(INFINITE);
        let _ = get_answer2(&deck1, &deck2);
    }

    #[test]
    fn test_answer2() {
        let (deck1, deck2) = process_input(INPUT);
        assert_eq!(291, get_answer2(&deck1, &deck2));
    }

    const INPUT: &'static str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

const INFINITE: &'static str = "\
Player 1:
43
19

Player 2:
2
29
14";

}

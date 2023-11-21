use rand::seq::SliceRandom;
use std::io;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Card {
    Number(Color, u8),
    Skip(Color),
    Reverse(Color),
    DrawTwo(Color),
    Wild,
}

enum PlayerChoice {
    Draw,
    Drop,
}

fn main() {
    let mut deck = create_deck();
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    let mut player_hand = deal_hand(&mut deck);
    let mut computer_hand = deal_hand(&mut deck);
    let mut discard_pile = vec![deck.pop().unwrap()];

    loop {
        display_game_state(&player_hand, &discard_pile);

        match ask_player() {
            PlayerChoice::Draw => draw_card(&mut deck, &mut player_hand),
            PlayerChoice::Drop => play_card(&mut player_hand, &mut discard_pile),
        }

        computer_turn(&mut deck, &mut computer_hand, &mut discard_pile);

        if player_hand.is_empty() {
            println!("Congratulations! You won!");
            break;
        } else if computer_hand.is_empty() {
            println!("Sorry, you lost. The computer won!");
            break;
        } else if deck.is_empty() {
            println!("The deck is empty. The game is a draw.");
            break;
        }
    }
}

fn display_game_state(player_hand: &[Card], discard_pile: &[Card]) {
    println!("Your hand:");
    for card in player_hand {
        println!("  {:?}", card);
    }
    println!("Top card: {:?}", discard_pile.last().unwrap());
}

fn ask_player() -> PlayerChoice {
    loop {
        println!("Enter 'draw' to draw a card or 'drop' to play a card:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "draw" => return PlayerChoice::Draw,
            "drop" => return PlayerChoice::Drop,
            _ => println!("Invalid input. Try again."),
        }
    }
}

fn draw_card(deck: &mut Vec<Card>, hand: &mut Vec<Card>) {
    if let Some(card) = deck.pop() {
        println!("You drew a card: {:?}", card);
        hand.push(card);
    } else {
        println!("The deck is empty!");
    }
}

fn play_card(hand: &mut Vec<Card>, discard_pile: &mut Vec<Card>) {
    if let Some(index) = get_valid_card_index(hand, discard_pile.last().unwrap()) {
        let played_card = hand.remove(index);
        println!("You played: {:?}", played_card);
        discard_pile.push(played_card);
    } else {
        println!("No valid cards to play. You drew a card.");
        draw_card(hand, discard_pile);
    }
}

fn get_valid_card_index(hand: &[Card], top_card: &Card) -> Option<usize> {
    hand.iter().position(|card| can_play_card(card, top_card))
}

fn can_play_card(new_card: &Card, top_card: &Card) -> bool {
    match (new_card, top_card) {
        // Number can be played if it has the same color and number
        (Card::Number(color1, number1), Card::Number(color2, number2)) => color1 == color2 && number1 == number2,
        
        // Skip, Reverse, and DrawTwo can be played if they have the same color
        (Card::Skip(color1), Card::Skip(color2))
        | (Card::Reverse(color1), Card::Reverse(color2))
        | (Card::DrawTwo(color1), Card::DrawTwo(color2)) => color1 == color2,

        // Wild cards can be played on any card
        (Card::Wild, _) | (_, Card::Wild) => true,

        // Any other combination is not allowed
        _ => false,
    }
}

fn computer_turn(deck: &mut Vec<Card>, computer_hand: &mut Vec<Card>, discard_pile: &mut Vec<Card>) {
    if let Some(index) = get_valid_card_index(computer_hand, discard_pile.last().unwrap()) {
        let played_card = computer_hand.remove(index);
        println!("Computer played: {:?}", played_card);
        discard_pile.push(played_card);
    } else {
        if let Some(card) = deck.pop() {
            println!("Computer drew a card: {:?}", card);
            computer_hand.push(card);
        } else {
            println!("The deck is empty!");
        }
    }
}

fn create_deck() -> Vec<Card> {
    let mut deck = Vec::new();

    for color in &[Color::Red, Color::Blue, Color::Green, Color::Yellow] {
        for number in 1..=9 {
            deck.push(Card::Number(color.clone(), number));
        }

        deck.push(Card::Skip(color.clone()));
        deck.push(Card::Reverse(color.clone()));
        deck.push(Card::DrawTwo(color.clone()));
    }

    deck
}

fn deal_hand(deck: &mut Vec<Card>) -> Vec<Card> {
    (0..7).filter_map(|_| deck.pop()).collect()
}
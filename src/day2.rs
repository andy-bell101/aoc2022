use std::collections::HashMap;

use crate::utils::get_file_contents;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum GameState {
    Lose,
    Draw,
    Win,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Hands {
    Rock,
    Paper,
    Scissors,
}

lazy_static! {
    static ref SCORES_FOR_GAME_STATE: HashMap<GameState, u64> = {
        return HashMap::from([
            (GameState::Lose, 0),
            (GameState::Draw, 3),
            (GameState::Win, 6),
        ]);
    };
    static ref SCORES_FOR_HAND: HashMap<Hands, u64> = {
        return HashMap::from([(Hands::Rock, 1), (Hands::Paper, 2), (Hands::Scissors, 3)]);
    };
    static ref WINS_AGAINST: HashMap<Hands, Hands> = {
        return HashMap::from([
            (Hands::Rock, Hands::Scissors),
            (Hands::Paper, Hands::Rock),
            (Hands::Scissors, Hands::Paper),
        ]);
    };
    static ref LOSES_AGAINST: HashMap<Hands, Hands> = {
        return HashMap::from([
            (Hands::Rock, Hands::Paper),
            (Hands::Paper, Hands::Scissors),
            (Hands::Scissors, Hands::Rock),
        ]);
    };
}

fn parse_chars_to_hands(c: &char) -> Hands {
    return match c {
        'A' | 'X' => Hands::Rock,
        'B' | 'Y' => Hands::Paper,
        'C' | 'Z' => Hands::Scissors,
        _ => panic!("invalid input!"),
    };
}

fn parse_chars_to_game_state(c: &char) -> GameState {
    return match c {
        'X' => GameState::Lose,
        'Y' => GameState::Draw,
        'Z' => GameState::Win,
        _ => panic!("invalid input!"),
    }
}

fn score_game(opponent_hand: &Hands, my_hand: &Hands) -> u64 {
    let game_state = if opponent_hand == my_hand {
        GameState::Draw
    } else if WINS_AGAINST[my_hand] == *opponent_hand {
        GameState::Win
    } else {
        GameState::Lose
    };
    return SCORES_FOR_GAME_STATE[&game_state] + SCORES_FOR_HAND[my_hand];
}

pub fn part_1(file_name: &str) -> String {
    let contents = get_file_contents(file_name);
    let games: Vec<(Hands, Hands)> = Vec::from_iter(contents.lines().map(|line| {
        (
            parse_chars_to_hands(&line.chars().nth(0).expect("not enough chars on line")),
            parse_chars_to_hands(&line.chars().nth(2).expect("not enough chars on line")),
        )
    }));
    return games
        .iter()
        .map(|(h1, h2)| score_game(h1, h2))
        .sum::<u64>()
        .to_string();
}

fn hand_to_pick_for_desired_game_state(opponent_hand: &Hands, game_state: &GameState) -> Hands {
    return match game_state {
        GameState::Draw => opponent_hand.clone(),
        GameState::Win => LOSES_AGAINST[opponent_hand],
        GameState::Lose => WINS_AGAINST[opponent_hand],
    };
}

pub fn part_2(file_name: &str) -> String {
    let contents = get_file_contents(file_name);
    let games: Vec<(Hands, GameState)> = Vec::from_iter(contents.lines().map(|line| {
        (
            parse_chars_to_hands(&line.chars().nth(0).expect("not enough chars on line")),
            parse_chars_to_game_state(&line.chars().nth(2).expect("not enough chars on line")),
        )
    }));
    let hands = games.iter().map(|(h, g)| (h, hand_to_pick_for_desired_game_state(h, g)));
    return hands
        .map(|(h1, h2)| score_game(h1, &h2))
        .sum::<u64>()
        .to_string();
}

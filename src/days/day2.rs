use super::get_contents;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn get_score_2_plays(which: usize, path: String) -> i32 {
    let chars: Vec<char> = get_contents(path).replace(" ", "").replace("\n", "").chars().collect();
    let mut player1_score = 0;
    let mut player2_score = 0;
    let letter_to_play = HashMap::from([
        ('A', String::from("Rock")),
        ('X', String::from("Rock")),
        ('B', String::from("Paper")),
        ('Y', String::from("Paper")),
        ('C', String::from("Scissors")),
        ('Z', String::from("Scissors")),
    ]);
    let score_by_play = HashMap::from([
        ("Rock".to_string(), 1),
        ("Paper".to_string(), 2),
        ("Scissors".to_string(), 3),
    ]);

    let mut play1: String;
    let mut play2: String;
    for i in (0..chars.len()).step_by(2) {
        let j = i + 1;
        play1 = String::from(letter_to_play.get(&chars[i]).unwrap());
        play2 = String::from(letter_to_play.get(&chars[j]).unwrap());

        if play1 == play2 {
            player1_score += 3;
            player2_score += 3;
        } else if play1 == "Paper" && play2 == "Rock" {
            player1_score += 6;
        } else if play1 == "Scissors" && play2 == "Paper" {
            player1_score += 6;
        } else if play1 == "Rock" && play2 == "Scissors" {
            player1_score += 6;
        } else {
            player2_score += 6;
        }

        player1_score += score_by_play.get(&play1).unwrap();
        player2_score += score_by_play.get(&play2).unwrap();
    }

    let mut scores = Vec::new();
    scores.push(player1_score);
    scores.push(player2_score);
    scores[which - 1]
}

#[allow(dead_code)]
pub fn get_score_by_outcome(which: usize, path: String) -> i32 {
    let chars: Vec<char> = get_contents(path).replace(" ", "").replace("\n", "").chars().collect();
    let mut player1_score = 0;
    let mut player2_score = 0;
    let mut scores: Vec<i32> = Vec::new();
    let play_to_lose = HashMap::from([
        ('C', 'B'),
        ('B', 'A'),
        ('A', 'C'),
    ]);
    let play_to_win = HashMap::from([
        ('A', 'B'),
        ('B', 'C'),
        ('C', 'A'),
    ]);
    let score_by_play = HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3),
    ]);

    for i in (0..chars.len()).step_by(2) {
        let j = i + 1;

        if chars[j] == 'X' {
            let what_to_play = play_to_lose.get(&chars[i]).unwrap();

            player1_score += 6 + score_by_play.get(&chars[i]).unwrap();
            player2_score += score_by_play.get(&what_to_play).unwrap();
        } else if chars[j] == 'Z' {
            let what_to_play = play_to_win.get(&chars[i]).unwrap();

            player1_score += score_by_play.get(&chars[i]).unwrap();
            player2_score += 6 + score_by_play.get(&what_to_play).unwrap();
        } else {
            player1_score += 3 + score_by_play.get(&chars[i]).unwrap();
            player2_score += 3 + score_by_play.get(&chars[i]).unwrap();
        }

    }

    scores.push(player1_score);
    scores.push(player2_score);
    scores[which - 1]
}

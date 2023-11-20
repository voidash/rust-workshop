use tour::imp_math::*;
use tour::imp_rockpaper::*;
use tour::imp_string::*;


#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(4, 5), 20);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(10, 5), 5);
}

// rock scissors paper

#[test]
fn test_rock_vs_scissors() {
    assert_eq!(play_round(Choice::Rock, Choice::Scissors), "Player 1 wins");
}

#[test]
fn test_scissors_vs_rock() {
    assert_eq!(play_round(Choice::Scissors, Choice::Rock), "Player 2 wins");
}

#[test]
fn test_paper_vs_rock() {
    assert_eq!(play_round(Choice::Paper, Choice::Rock), "Player 1 wins");
}

#[test]
fn test_rock_vs_paper() {
    assert_eq!(play_round(Choice::Rock, Choice::Paper), "Player 2 wins");
}

#[test]
fn test_scissors_vs_paper() {
    assert_eq!(play_round(Choice::Scissors, Choice::Paper), "Player 1 wins");
}

#[test]
fn test_paper_vs_scissors() {
    assert_eq!(play_round(Choice::Paper, Choice::Scissors), "Player 2 wins");
}


// String manipulation
#[test]
fn test_reverse_words_single() {
    assert_eq!(reverse_words("rust"), "tsur");
}

#[test]
fn test_count_vowels_and_consonants() {
    assert_eq!(count_vowels_consonants("hello"), (2,3));
}

#[test]
fn test_ceasar_cypher() {
    assert_eq!(ceasar_cipher("nepal",1), "ofqbm");
}


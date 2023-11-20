// Rock paper Scissors
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

pub fn play_round(player1: Choice, player2: Choice) -> String {
    use Choice::*;

    let p1_win = String::from("Player 1 wins");
    let p2_win = String::from("Player 2 wins");

    match (player1, player2) {
        //for player 1 win conditions
            //rock wins Scissors
        (Rock, Scissors) => p1_win,
            //scissors win paper
        // TODO
            //paper wins rock
        // TODO

        //for player 1 the lose conditions are
            // Scissors loses with Rock
        (Scissors, Rock) => p2_win,
            // Paper loses with Scissors
        // TODO
            // Rock loses with Paper
        // TODO
        _ => String::from("It's a tie")
    }
}



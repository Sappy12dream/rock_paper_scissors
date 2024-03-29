use std::io;
use rand::Rng;

#[derive(Debug, PartialEq)]
enum Choices {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Choices {
    fn from_str(choice: &str) -> Option<Choices> {
        match choice {
            "r" => Some(Choices::ROCK),
            "s" => Some(Choices::SCISSORS),
            "p" => Some(Choices::PAPER),
            _ => None,
        }
    }

    fn to_emoticon(&self) -> &'static str {
        match self {
            Choices::ROCK => "🗿",
            Choices::PAPER => "📃",
            Choices::SCISSORS => "✂️",
        }
    }
}

fn main() {
  println!("Welcome to Rock, Paper, Scissors!");
  loop{
    println!("Please choose 🌑 (r)ock, 📃 (p)aper, or ✂️ (s)cissors, or (q)uit:");

    let mut player_choice= String::new();

    io::stdin().read_line(& mut player_choice ).expect("Failed to read line");
let player_choice = player_choice.trim().to_lowercase();

    if player_choice == "q"{
      println!("Thanks for playing!");
    };

    let player_choice = match Choices::from_str(& player_choice){
      Some(choice)=> choice,
      None => {
        println!("Invalid choice. Please try again.");
        continue;
      },
      
    };
    let computer_choice = get_computer_choice();

    println!("You chose: {}", player_choice.to_emoticon());

    println!("Computer chose: {}", computer_choice.to_emoticon());
    
    match (player_choice, computer_choice) {
            (Choices::ROCK, Choices::SCISSORS)
            | (Choices::SCISSORS, Choices::PAPER)
            | (Choices::PAPER, Choices::ROCK) => println!("You win! 🎉"),
            (Choices::ROCK, Choices::PAPER)
            | (Choices::SCISSORS, Choices::ROCK)
            | (Choices::PAPER, Choices::SCISSORS) => println!("Computer wins! 😔"),
            _ => println!("It's a tie! 😐"),
    }
  }
}

fn get_computer_choice()->Choices{
  match rand::thread_rng().gen_range(0..=2){
    0=>Choices::ROCK,
    1=>Choices::PAPER,
    _=>Choices::SCISSORS,
  }
}
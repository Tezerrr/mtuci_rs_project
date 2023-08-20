use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use chrono::NaiveDateTime;
use regex::Regex;

struct Flashcard {
    question: String,
    answer: String,
}

struct FlashcardDeck {
    cards: Vec<Flashcard>,
}

impl FlashcardDeck {
    fn new() -> Self {
        FlashcardDeck { cards: Vec::new() }
    }

    fn add_card(&mut self, card: Flashcard) {
        self.cards.push(card);
    }

     fn load_from_txt(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut cards = Vec::new();
        let contents = std::fs::read_to_string(filename)?;

        let re = Regex::new(r"Created: (.+)\n   Question: (.+)\n   Answer: (.+)")?;

        for capture in re.captures_iter(&contents) {
            if let (Some(created), Some(question), Some(answer)) = (
                capture.get(1),
                capture.get(2),
                capture.get(3),
            ) {
                let created = NaiveDateTime::parse_from_str(created.as_str(), "%Y-%m-%d %H:%M:%S%.f %z")?;
                cards.push(Flashcard {
                    question: question.as_str().trim().to_string(),
                    answer: answer.as_str().trim().to_string(),
                });
            }
        }

        Ok(FlashcardDeck { cards })
    }

    fn study(&mut self) {
            let mut current_card = 0;

            while current_card < self.cards.len() {
                let card = &self.cards[current_card];
                println!("Question: {}", card.question);
                println!("Press Enter to reveal the answer...");

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                println!("Answer: {}", card.answer);
                println!("Did you get it right? (y/n)");

                input.clear();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let user_response = input.trim().to_lowercase();

                if user_response == "y" || user_response == "yes" {
                    println!("Card {} removed from deck.", current_card + 1);
                    self.cards.remove(current_card);
                } else {
                    current_card += 1;
                }
            }
            println!("All cards have been studied!");
        }

     fn save_to_txt(&self, filename: &str) -> io::Result<()> {
        let path = Path::new(filename);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        for (index, card) in self.cards.iter().enumerate() {
            let card_info = format!(
                "{}. Created: {}\n   Question: {}\n   Answer: {}\n\n",
                index + 1,
                chrono::Local::now(),
                card.question,
                card.answer
            );
            file.write_all(card_info.as_bytes())?;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut deck = FlashcardDeck::load_from_txt("flashcards.txt")?;

    loop {
        println!("Flashcard App Menu:");
        println!("1. Add a new flashcard");
        println!("2. Study flashcards");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                println!("Enter question:");
                let mut question = String::new();
                io::stdin().read_line(&mut question)?;

                println!("Enter answer:");
                let mut answer = String::new();
                io::stdin().read_line(&mut answer)?;

                let card = Flashcard {
                    question: question.trim().to_string(),
                    answer: answer.trim().to_string(),
                };

                deck.add_card(card);
                if let Err(err) = deck.save_to_txt("flashcards.txt") {
                    eprintln!("Failed to save flashcards: {}", err);
                } else {
                    println!("Flashcard added and saved!");
                }
            }
            "2" => {
                deck.study();
                if let Err(err) = deck.save_to_txt("flashcards.txt") {
                    eprintln!("Failed to save flashcards: {}", err);
                } else {
                    println!("Flashcards updated and saved!");
                }
            }

            "3" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please select a valid option.");
            }
        }
    }

    Ok(())
}
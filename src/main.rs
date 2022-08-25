use std::*;

///Wordle
/// Rules of the game:
/// 1) You can on&ly play one puzzle a day (a run, for this version)
/// 2)You have to guess a 5 letter world
/// 3) You have 6 attempts to guess the world
/// 4)Every time you guess you are told which of the letters you guess is in the target word
/// 5)You are also told if the letter was in the right place

fn main() {
    let decoration = "**********************************";
    println!("{} WORDLE {}",decoration,decoration);
    println!("Please enter a 5 letter word: ");
    input();
}

//Here, we take the word and return
//You win if you guess it or
//ask for another try
fn input(){
    let mut guess_times = 6;
    while guess_times > 0{
     //Here is where we take input
     let mut guess: String = String::new();
     let target_word: String = String::from("hello");
     io::stdin().read_line(&mut guess).unwrap(); //We use this crate for the input
     let guess = guess.trim().to_string(); //and save in the variable
     
        //Is the word 5 letters?
        if guess.len() == 5{
            if guess == target_word{
                println!("You win! The word was {}", target_word);
                break;
            }
            else{
                println!("Please try again!");
                guess_times -=1;
            }  
        }
        else{
            println!("It has to be a 5 letter word.");
            guess_times -=1;
            continue;
        }
        
    }

}


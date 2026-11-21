/*
Copyright (c) 2026 @kforkernel
All rights reserved.

Licensed under the kforkernel Use License (KUL) v1.0.

See LICENSE.md for terms and conditions.
*/

use std::io;
use colored::Colorize;

fn main() {

    // displaying options in terminal //15_
    let words = "_______________
|lollipop      |
---------------|
|chocolate     |
---------------|
|jellybean     |
---------------|
|marshmallow   |
---------------|
|bubblegum     |
---------------|
|peppermint    |
---------------|
|licorice      |
---------------|
|caramel       |
---------------|
|cottoncan     |
---------------|
|toffee        |
---------------|
|brownie       |
---------------|
|cupcake       |
---------------|
|cookie        |
---------------|
|doughnut      |
---------------|
|pancake       |
---------------|
|waffle        |
---------------|
|cheesecake    |
---------------|
|icecream      |
---------------|
|milkshake     |
---------------|
|pudding       |
|______________|".truecolor(255, 0, 0).bold();



    // creating the correct answer word and the wordlist
    let mut guess_count = 0;

    let rand_num: usize = rand::random_range(0..20);

    let word_list: Vec<&str> = vec![
        "lollipop",
        "chocolate",
        "jellybean",
        "marshmallow",
        "bubblegum",
        "peppermint",
        "licorice",
        "caramel",
        "cottoncandy",
        "toffee",
        "brownie",
        "cupcake",
        "cookie",
        "doughnut",
        "pancake",
        "waffle",
        "cheesecake",
        "icecream",
        "milkshake",
        "pudding",
    ];
    
    let correct_answer = word_list[rand_num];



    // creating hints and length variables
    let hint_len = correct_answer.len();



    // making sure first guess no hint
    let mut no_hint = 0;



    loop{
        // the hint loop
        if no_hint != 0{
            // creating random number from 0 to max length of word
            let hint: usize = rand::random_range(0..hint_len);

            // picking a random char of the word
            let char_hint = correct_answer.chars().nth(hint);

            // printing the hint
            println!("{} {}", "Hint:".truecolor(50, 205, 50).bold(), char_hint.unwrap().to_string().truecolor(255, 255, 0));
        }
        no_hint += 1;



        // creating fresh string and getting user input and displaying word options
        let mut guess = String::new();

        println!("{}\n{}\n\n", "You can choose from".truecolor(106, 27, 154).bold(), &words);

        println!("{}", "Input your guess!".cyan().bold());

        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        
        print!("\n");



        // cleaning user input and printing guess and incramenting guess count
        let guess = guess.trim();
        println!("{} {}", "You guessed:".truecolor(0, 32, 194).bold(), guess.truecolor(255, 255, 0));
        guess_count += 1;



        // checking if answer correct
        if guess == correct_answer{
            break;
        }
        if guess_count == 5{
            println!("You lose the answer was {correct_answer}");
            std::process::exit(0);        
        }
        else{
        println!("{}", "Wrong answer, try again!".truecolor(255, 0, 102).bold());        
        continue;
        }
    }

    // success!!!
    // combine variables into a single string slice
    let success_message = format!("Congratulations the correct answer was {}, you guessed it in {} guesses!", correct_answer, no_hint);

    // Loop through every character and apply color pattern
    for (i, c) in success_message.chars().enumerate() {
        // looping colours
        let color_step = i % 6;
        let colored_char = match color_step {
            0 => c.to_string().truecolor(255, 0, 0),       //  red
            1 => c.to_string().truecolor(255, 127, 0),     //  orange
            2 => c.to_string().truecolor(255, 255, 0),     //  yellow
            3 => c.to_string().truecolor(0, 255, 0),       //  lime green
            4 => c.to_string().truecolor(0, 150, 255),     //  sky blue
            _ => c.to_string().truecolor(180, 0, 255),     //  purple
        };
        // print each character side by side
        print!("{}", colored_char);
    }
    println!();
}

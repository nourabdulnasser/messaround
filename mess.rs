

/* Addition to main.rs
} else if mode == "-names" {
       /* let amount_of_names = args[2].parse::<usize>().unwrap();
        let amount_of_seconds_per_names = args[3].parse::<usize>().unwrap();
        names::memory_names(amount_of_names,amount_of_seconds_per_names);
        */
        names::memory_names();
    }
*/

// --------- names.rs

use rand::Rng;
use std::{thread};
use super::input_wrapper;
use terminal::{Clear, Action};



pub fn memory_cards(length: usize, time: usize){


    let terminal = terminal::stdout();
    let ranks = vec!["A","K","Q","J","2","3","4","5","6","7","8","9","10"];
    let suits = vec!["c","d","h","s"];
    //let suits = vec!["♣","♦","♥","♠"];

    // random suits and ranks

    let random_card = || {

        let mut rng = rand::thread_rng();

        let rand_index_rank = rng.gen_range(0..ranks.len());
        let rand_index_suit = rng.gen_range(0..suits.len());

        let rank = ranks[rand_index_rank];
        let suit = suits[rand_index_suit];


        // creates a random card with a rank and suit

        return rank.to_owned()+suit;

    };

    // generating cards

    let mut cards_line: String = "".to_string();

    for  card in 0..length{
        // random_card();
        // let random_card_str = &random_card(); // to push random strings into vector
        cards_line = cards_line + &random_card();
        // cards_vec.push(random_card_str);
        if card < length-1 {
            cards_line = cards_line + ",  ";
        }

        // "looks,  like,  this"

    }




    // wait for {time} seconds

    let mem_time_u64 = (length*time).try_into().unwrap();
    let mem_dur = std::time::Duration::from_secs(mem_time_u64);

    println!("Memorise this:\n{:?}\nYou have {} seconds.", cards_line, length*time);

    thread::sleep(mem_dur);

    terminal.act(Action::ClearTerminal(Clear::All)).map_err(|err| println!("{:?}", err)).ok();



    // user prompt

    print!("Type in what you memorised!\nTo exit, enter: exit\n");



    // comparing input and cards_line_vec

    let cards_line_split = cards_line.split(",  ");
    let cards_line_vec: Vec<&str> = cards_line_split.collect();

    // println!("{:?}", cards_line_vec); // test

    let mut counter = 0;

    for _item in &cards_line_vec {
        let user_input = input_wrapper::get_input();
        if user_input.to_uppercase() == cards_line_vec[counter].to_uppercase(){
            println!("Bravo! You got {} right!\n", cards_line_vec[counter]);
        } else if user_input == "exit" {
           break;
        } else {
            println!("Nay :( The right answer is {}.\n", cards_line_vec[counter]);
        };

        counter = counter +1;

    }

    println!("end");

}

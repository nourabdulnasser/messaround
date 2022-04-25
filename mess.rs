use std::env;
use rand::Rng;
use std::{thread, time};
use std::io::{stdin,stdout,Write};
//use std::process::Command;


pub fn cards_memory(length: usize, time: usize){

    //use random_string::generate;

    let ranks = vec!["A","K","Q","J","2","3","4","5","6","7","8","9","10"];
    let suits = vec!["c","d","h","s"]; 
    //let suits = vec!["♣","♦","♥","♠"];
    
    // random suits and ranks

    let random_card = || {

        let mut rng = rand::thread_rng();

        let rand_index_rank = rng.gen_range(0..ranks.len()-1);
        let rand_index_suit = rng.gen_range(0..suits.len()-1);
    
        let rank = ranks[rand_index_rank];
        let suit = suits[rand_index_suit];

        //println!("{}", rank.to_owned()+suit);

        return rank.to_owned()+suit;

        // looks
        // like
        // this

    };

    // generating cards
    
    let mut cards_line: String = "".to_string();
    let mut cards_vec = vec![];

    for  card in 0..length{
        //random_card();
        let mut random_card_str = &random_card();
        cards_line = cards_line + random_card_str;
        cards_vec.push(random_card_str);
        if card < length-1 {
            cards_line = cards_line + ",  ";
        }

        // "looks,  like,  this" 

    }

    println!("{:?}", cards_vec);

   // println!("Memorise this:\n{{ {} }}\nYou have {} seconds.", cards_line, time);

    // wait for time seconds

    let mem_time_u64 = (length*time).try_into().unwrap(); 
    
    let mem_time = std::time::Duration::from_secs(mem_time_u64);

    println!("Memorise this:\n{{ {} }}\nYou have {} seconds.", cards_line, length*time);

    let now = time::Instant::now();

    thread::sleep(mem_time);

    print!("\x1B[2J\x1B[1;1H");  // not as clean as I'd like it to be but will do for now.

    
    
    
    //println!("what");
    
    
    //std::process::Command::new("cls").status().unwrap();


    // user prompt

   /* let mut s=String::new();
    print!("Type in what you memorised!");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    println!("You typed: {}",s);*/




    //println!("")
    


    
    //let suit = "CDHS";
    //println!("{}", generate(length, suit));

}



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
use super::api_call;
use std::{thread};
use terminal::{Clear, Action};
//use regex //1.4.5

pub fn memory_names(length: usize, time: usize){
    let mut random_names: &str = "";
    let mut names_vec: Vec<String> = vec![];  // vector of clean random names
    

    // function for getting a random name
    fn random_name() -> String{
        let mut api_name_vec: Vec<String> = vec![]; // vector of names extracted from api function
        let api_name: String = api_call::api_get_request("https://random-data-api.com/api/name/random_name");
        let api_delimit = regex::Regex::new(r",|:").unwrap(); // delimit characters for easier iteration
        let api_delimit_vec: Vec<&str> = api_delimit.split(&api_name).collect(); // vector of all elements in api
        let mut name = "";
        let mut counter = 0;

        // trying to convert elements of api_delimit_vec into String
        for element in &api_delimit_vec{
            if element == &"\"first_name\""
            {
                name = &api_delimit_vec[counter+1];  // using let to bind
            }
            counter = counter +1 ;
        } 
        // println!("{:?}",api_name_vec); // test
        return name.to_string().replace("\"","");
    }

    let mut random_names_vec: Vec<String> = vec![];

    // loop to get {number_of_names} random names
    for number in 0..length
    {
        random_names_vec.push(random_name());
        println!("{:?}", random_names_vec); // test
    }

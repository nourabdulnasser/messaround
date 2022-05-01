

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
// regex = "1.4.5"

pub fn memory_names(){
    let mut random_names: &str = "";
    let mut names_vec: Vec<&str> = vec![];  // vector of clean random naames
    let mut counter = 0;

    // function for getting random instance through api and putting it in a vector
    fn get_api_vec() -> Vec<String>{
        let mut api_name_vec: Vec<&str> = vec![]; // vector of names extracted from api function
        let api_name: String = api_call::api_get_request("https://random-data-api.com/api/name/random_name");
        let api_delimit = regex::Regex::new(r",|:").unwrap(); // delimit characters for easier iteration
        let api_delimit_vec: Vec<&str> = api_delimit.split(&api_name).collect(); // vector of all elements in api
    
        // trying to convert elements of api_delimit_vec into String
        for element in api_delimit_vec{
            let jk = element.to_string();
            api_name_vec.push(jk);
        } 
        println!("{:?}",api_name_vec); // test
        return api_name_vec;
    }

    // get_api_vec(); // test

    let number_of_names: usize;

    // loop to get {number_of_names} random names
    for number in 0..number_of_names
    {
        for element in &get_api_vec()
        {
          if get_api_vec()[counter] == "\"first_name\""
            {
                let random_name = get_api_vec()[counter+1];  // using let to bind
                // random_name.replace("\"","");
                names_vec.push(&random_name);
            }
            counter = counter + 1;
        }

    }
    println!("{:?}", names_vec); // test
}


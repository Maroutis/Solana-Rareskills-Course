use anchor_lang::prelude::*;

declare_id!("8jY4ccw4cGe5X7LNtktdtTmFkR2MpZcqYKpndupAtRyo");

// *** CONSTANT DECLARED HERE ***
const MEANING_OF_LIFE_AND_EXISTENCE: u64 = 42;

#[program]
pub mod tryrust {
    use super::*;

    use std::collections::HashMap;

    pub fn initialize(ctx: Context<Initialize>, key: String, value: String, name: String, age: u64) -> Result<()> {
        for i in (0..10).step_by(2) {
            // do something...
                
            msg!("{}", i);
        }
        // Declare a dynamic array-like structure using Vec
        let mut dynamic_array: Vec<u32> = Vec::new();

        // Add elements to the dynamic array
        dynamic_array.push(10);
        dynamic_array.push(20);
        dynamic_array.push(30);

        // Accessing elements of the dynamic array
        let first_element = dynamic_array[0];
        let third_element = dynamic_array[2];

        // Rest of your program's logic
        msg!("Third element = {}", third_element);

                // Initialize the mapping
        let mut my_map = HashMap::new();

        // Add a key-value pair to the mapping
        my_map.insert(key.to_string(), value.to_string());// Have to do a clone here because key ownership will be given to the hashmap, doing to_string creates a copy of key and given to the map

        // Log the value corresponding to a key from the mapping
        msg!("My name is {}", my_map[&key]);


        // Defining a struct in Solana
        struct Person {
            my_name: String,
            my_age: u64,
        }

        // Creating an instance of the struct
        let mut person1: Person = Person {
            my_name: name,
            my_age: age,
        };

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        // Accessing and modifying struct fields
        person1.my_name = "Bob".to_string();
        person1.my_age = 18;

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        msg!(&format!("Answer to the ultimate question: {}", MEANING_OF_LIFE_AND_EXISTENCE)); // new line here

        let mut dynamic_array: Vec<u32> = Vec::from([1,2,3,4,5,6]);
       let len = dynamic_array.len(); // this has type usize
       
       let another_var: u64 = 5; // this has type u64

       let len_plus_another_var = len as u64 + another_var;

       msg!("The result is {}", len_plus_another_var);

        Ok(())
    }

//     pub fn age_checker(ctx: Context<Initialize>, age: u64) -> Result<()> {
//     if age >= 18 {
//         msg!("You are 18 years old or above");
//     } else {
//         msg!("You are below 18 years old");
//     }
//     Ok(())
//  }

    // pub fn age_checker(ctx: Context<Initialize>, age: u64) -> Result<()> {
    //     let result = if age >= 18 {"You are 18 years old or above"} else { "You are below 18 years old" }; // add ; because assigning a varibale # expression
    //     msg!("{:?}", result);
    //     Ok(())
    // }

    pub fn age_checker(ctx: Context<Initialize>, age: u64) -> Result<()> {
    match age {
        1 => {
            // Code block executed if age equals 1
            msg!("The age is 1");
        },
        2 | 3 => {
            // Code block executed if age equals 2 or 3
            msg!("The age is either 2 or 3");
        },
        4..=6 => {
            // Code block executed if age is in the 
            // range 4 to 6 (inclusive)
            msg!("The age is between 4 and 6");
        },
        _ => {
            // Code block executed for any other age
            msg!("The age is something else");
        }
    }
    Ok(())
}





}

#[derive(Accounts)]
pub struct Initialize {}

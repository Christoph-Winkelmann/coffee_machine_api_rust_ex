/*
Purpose:
    An API for a simple coffe machine.
    It should:
    - have a variable to measure coffee beans and water level
    - Provide a function to make a cup of coffee, using beans and water
    - Check after each cup, if there are enough beans and water for the next cup
    - provide warnings and messages for certain situations
    - provide functions for refilling supplies (beans, water)
*/

use std::io;

const BEANS_MAX: u32 = 100;
const WATER_MAX: u32 = 1_250;

fn main() {
    let mut running: bool = true;
    let mut beans_state = BEANS_MAX;
    let mut water_state = WATER_MAX;

    while running {
        let needs_beans: bool = if beans_state < 15 { true } else { false };
        let needs_water: bool = if water_state < 240 { true } else { false };
        let needs_refill: bool = needs_beans || needs_water;
        // display beans and water level
        println!("Beans: {beans_state} g\nWater: {water_state} ml\n");

        // ask user for action
        println!("What can I do for you?");
        if needs_refill {
            println!(
                r"

2. Refill
3. Exit
"
            )
        } else {
            println!(
                r"
1. Make Coffee
2. Refill
3. Exit
"
            );
        }
        // determine choice and call corresponding functions
        let choice = get_input();
        if choice == "1" {
            if needs_beans {
                println!("Not enough beans. Please refill the machine.");
                continue;
            }
            if needs_water {
                println!("Not enough water. Please refill the water tank.");
                continue;
            }
            beans_state -= 15;
            water_state -= 240;
            println!("Here you go.");
        } else if choice == "2" {
            if needs_beans {
                beans_state = BEANS_MAX;
                println!("Refill successfull");
                continue;
            }
            if needs_water {
                water_state = WATER_MAX;
                println!("Refill successfull");
                continue;
            }
            println!("Refill not required.")
        } else if choice == "3" {
            running = false;
        } else {
            println!("Invalid input.");
            continue;
        }
    }
    println!("Good-bye...")
}

// fn check_needs() -> String {}

fn get_input() -> String {
    loop {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");
        let input: String = match input.trim().parse() {
            Ok(str) => str,
            Err(_) => continue,
        };
        return input;
    }
}

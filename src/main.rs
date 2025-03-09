/*
Purpose:
    An API for a simple coffe machine.
    It should:
    - have a variable to measure coffee beans and water level
    - Provide a function to make a cup of coffee, consuming beans and water
    - Check if there are enough beans and water for the next cup
    - Provide warnings and messages for certain situations
    - Provide a function for refilling supplies (beans, water)

Updated Context:
    - Restructuring the old code using what I have learned from the Rust Book
        Chapters 4 - 6 (https://rust-book.cs.brown.edu/)

Addendum (After finishing this version of the code):
    - This is more verbose than the old version
    - But it works very well while relying on less loose variables
    - I didnt spend any thoughts on accesability modifiers (public and private) as
        that is a lesson in Rust that I havent dealt with yet.
*/

use std::io;

struct CoffeeMachine {
    beans_max: u32,
    water_max: u32,
    curr_beans: u32,
    curr_water: u32,
    status: Status,
}

impl CoffeeMachine {
    fn refill(&mut self) {
        self.curr_beans = self.beans_max;
        self.curr_water = self.water_max;
    }

    fn make_coffee(&mut self) {
        self.curr_beans -= 15;
        self.curr_water -= 240;
        println!("Here you go.");
    }

    fn set_status(&mut self) {
        if self.curr_beans < 15 || self.curr_water < 240 {
            self.status = Status::NeedsRefill;
        } else {
            self.status = Status::Operational;
        }
    }

    fn display_menu(&self) {
        match self.status {
            Status::Operational => {
                let output = println!(
                    r"
    1. Make Coffee
    2. Refill
    3. Exit
    ",
                );
                output
            }
            Status::NeedsRefill => {
                let output = println!(
                    r"

    2. Refill
    3. Exit
    ",
                );
                output
            }
        }
    }

    fn display_fill_state(&self) {
        println!(
            "Beans: {} gramms | Water: {} milliliters",
            self.curr_beans, self.curr_water
        );
    }
}

enum Status {
    Operational,
    NeedsRefill,
}

//////
// Main Program
/////

fn main() {
    let mut program_running: bool = true;

    let mut coffee_machine1 = CoffeeMachine {
        beans_max: 100,
        water_max: 1_250,
        curr_beans: 0,
        curr_water: 0,
        status: Status::NeedsRefill,
    };

    while program_running {
        coffee_machine1.set_status();
        coffee_machine1.display_fill_state();
        coffee_machine1.display_menu();

        // get input
        let input = get_input();

        // process input
        match input {
            1 => match coffee_machine1.status {
                Status::NeedsRefill => invalid_input(),
                Status::Operational => coffee_machine1.make_coffee(),
            },
            2 => coffee_machine1.refill(),
            3 => program_running = false,
            _ => (),
        }
    }
    println!("Goodbye!")
}

//////
//////
//////

// fn check_needs() -> String {}

fn get_input() -> u8 {
    loop {
        println!("Welcome, what can I do for you?");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");
        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                invalid_input();
                continue;
            }
        };
        if input > 3 || input < 1 {
            invalid_input();
            continue;
        } else {
            return input;
        }
    }
}

fn invalid_input() {
    println!("Invalid input. Type the corresponding number.");
}

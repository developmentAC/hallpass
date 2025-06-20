// I was going to break up this code into multiple files, but I ran out of time. I will take care of that later.

use colored::*;
use rand::Rng;
use rand::seq::SliceRandom;
use std::io::{self, Write};

// Terminal color and style constants for output formatting
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const BOLD: &str = "\x1b[1m";

// Player struct holds all player stats and state
#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    str_: u8, // Strength
    int_: u8, // Intelligence
    cha: u8,  // Charisma
    money: i32,
    grades: i32,
    popularity: i32,
    attention: i32,
    warnings: u8,
    expelled: bool,
    relationship_charlotte: i32,
    reputation: i32,
}

#[derive(Debug)]
pub struct PlayerStats {
    pub money: i32,
    pub popularity: i32,
    pub grades: i32,
}

// Bully struct for adversaries
#[derive(Debug)]
struct Bully {
    name: &'static str,
    str_: u8,
    // int_: u8, //never used?
    // cha: u8, //never used?
}

// ShopItem struct for items in the shop
#[derive(Debug)]
pub struct ShopItem {
    pub name: &'static str,
    pub description: &'static str,
    pub cost: i32,
    pub popularity_change: i32,
    pub grades_change: i32,
    pub money_change: i32,
}

impl ShopItem {
    pub fn apply(&self, player: &mut Player) {
        player.money -= self.cost;
        player.popularity += self.popularity_change;
        player.grades += self.grades_change;
        player.money += self.money_change;
    }
}

// Main game loop and menu
fn main() {
    // Print game title
    println!(
        "{}",
        "Welcome to Hall Pass!"
            .bold()
            .underline()
            .bright_yellow()
            .on_green()
    );
    // Character creation at game start
    let mut player = character_creation();

    println!("Welcome {PLAYER_NAME}!", PLAYER_NAME = player.name);
    println!(
        "You are a student at Hall Pass High, trying to survive the school year while maintaining your grades, popularity, and relationships."
    );

    // List of bullies in the game
    let bullies = vec![
        Bully {
            name: "Chad",
            str_: 8,
            // int_: 3,
            // cha: 4,
        },
        Bully {
            name: "Vicky",
            str_: 6,
            // int_: 5,
            // cha: 7,
        },
        Bully {
            name: "Spike",
            str_: 9,
            // int_: 2,
            // cha: 3,
        },
    ];

    let mut day = 1;
    loop {
        println!("\n{BOLD}{YELLOW}--- --- Day {} --- ---{RESET}", day);

        // Game over conditions
        if player.expelled || player.money <= 0 || player.popularity <= -10 {
            println!("{RED}Game Over! You have lost.{RESET}");
            break;
        }
        // Win conditions
        if player.popularity >= 30 && player.money >= 100 && player.grades >= 80 {
            println!(
                "{GREEN}Congratulations! You are the most popular student and have won Hall Pass!{RESET}"
            );
            break;
        }
        if player.relationship_charlotte >= 30 {
            println!("{MAGENTA}You have won Charlotte's heart! You win!{RESET}");
            break;
        }

        // Show current player status
        show_status(&player);

        // // Random event with 20% chance
        // if rand::thread_rng().gen_bool(0.2) {
        //     random_event(&mut player);
        // }

        // Random event with 95% chance
        if rand::thread_rng().gen_bool(0.95) {
            println!("\n  ⚡️  {BOLD}{CYAN}A random event occurs!{RESET}");
            random_event(&mut player);
        }

        // Main menu options
        println!("{CYAN}\n Choose your action:{RESET}");
        println!("{YELLOW} 1. Dodge Bullies{RESET}");
        println!("{YELLOW} 2. Milk Heist{RESET}");
        println!("{YELLOW} 3. Cheat on a test{RESET}");
        println!("{YELLOW} 4. Flirt with Charlotte{RESET}");
        println!("{YELLOW} 5. Visit Item Shop{RESET}");
        println!("{YELLOW} 6. Skip Day{RESET}");
        println!("{YELLOW} 7. Confront Bullies (Fight or Distract){RESET}");
        println!("{YELLOW} 8. Confront Principal (Fight or Distract){RESET}");

        // Get player menu choice
        let choice = get_input("Enter choice (1-8): ");
        match choice.trim() {
            "1" => dodge_bullies(&mut player, &bullies),
            "2" => {
                milk_heist(&mut player);
                bullies_or_principal_event(&mut player, &bullies, "milk_heist");
            }
            "3" => {
                test_cheating(&mut player);
                bullies_or_principal_event(&mut player, &bullies, "test_cheating");
            }
            "4" => charlotte_flirt(&mut player),
            "5" => item_shop(&mut player),
            "6" => {
                println!("{BLUE}You chill for the day...{RESET}");
                println!(
                    "{RED}Be careful! Skipping a day draws attention! The principal or bullies will come looking for you!{RESET}"
                );
                if rand::thread_rng().gen_bool(0.5) {
                    // principal_confrontation(&mut player);
                    confront_principal(&mut player);
                } else {
                    // dodge_bullies(&mut player, &bullies);
                    confront_bullies(&mut player, &bullies);
                }
            }
            "7" => confront_bullies(&mut player, &bullies),
            "8" => confront_principal(&mut player),
            _ => println!("{RED}Invalid choice.{RESET}"),
        }

        // Principal's attention and warning system
        if player.attention >= 10 {
            player.warnings += 1;
            player.attention = 0;
            println!(
                "  {RED}Principal: This is your warning #{}!{RESET}",
                player.warnings
            );
            if player.warnings >= 3 {
                player.expelled = true;
                println!(" {RED}Oh no!! You have been expelled!{RESET}");
            }
        }

        day += 1;
    }
}

// Character creation with persona selection
fn character_creation() -> Player {
    println!("\n--- --- Character Creation --- ---");
    let name = get_input("Enter your name: ");
    println!(" Choose your persona:");
    println!("  1. Jock (STR 8, INT 4, CHA 4)");
    println!("  2. Nerd (STR 3, INT 9, CHA 4)");
    println!("  3. Charmer (STR 4, INT 4, CHA 8)");
    println!("  4. Average (STR 5, INT 5, CHA 5)");
    loop {
        let choice = get_input("Enter 1-4: ");
        let (str_, int_, cha) = match choice.trim() {
            "1" => (8, 4, 4), // values are for creating initial player stats
            "2" => (3, 9, 4),
            "3" => (4, 4, 8),
            "4" => (5, 5, 5),
            _ => {
                println!("Invalid choice.");
                continue;
            }
        };
        let money = 10 + str_ as i32 + int_ as i32 + cha as i32;
        return Player {
            name,
            str_,
            int_,
            cha,
            money,
            grades: 70,
            popularity: 5,// initial popularity
            // popularity: 80, //used to test the game
            attention: 0,
            warnings: 0,
            expelled: false,
            relationship_charlotte: 0,
            reputation: 0,
        };
    }
}

// Display current player stats
fn show_status(player: &Player) {
    // Print stats with color
    println!(
        "  {} {}\n  {} {}\n  {} {}\n  {} {}",
        "Money:".bold().bright_blue().on_bright_green(),
        player
            .money
            .to_string()
            .bold()
            .bright_blue()
            .on_bright_green(),
        "Popularity:".bold().bright_yellow().on_bright_blue(),
        player
            .popularity
            .to_string()
            .bold()
            .bright_yellow()
            .on_bright_blue(),
        "Grades:".bold().bright_yellow().on_bright_purple(),
        player
            .grades
            .to_string()
            .bold()
            .bright_yellow()
            .on_bright_purple(),
        "Charlotte's Interest in you:"
            .bold()
            .bright_cyan()
            .on_bright_magenta(),
        player
            .relationship_charlotte
            .to_string()
            .bold()
            .bright_cyan()
            .on_bright_magenta(),
        //player.relationship_charlotte
    );
}

// // Display current player stats
// (alternative version with different colors)
// fn show_status(player: &Player) {
//     // Print stats with color
//     println!(
//         "{} {}  {} {}  {} {}",
//         "Money:".bold().bright_yellow().on_bright_blue(),
//         player.money.to_string().green().bold().on_bright_magenta(),
//         "Popularity:".bold().yellow(),
//         player.popularity.to_string().yellow().bold(),
//         "Grades:".bold().cyan(),
//         player.grades.to_string().cyan().bold()
//     );
// }

// Event: Try to dodge a random bully
fn dodge_bullies(player: &mut Player, bullies: &[Bully]) {
    let bully = &bullies[rand::thread_rng().gen_range(0..bullies.len())];
    println!("{RED}You encounter bully {}!{RESET}", bully.name);
    let success_chance = player.str_ as i32 * 10 - bully.str_ as i32 * 5 + 50;
    let roll = rand::thread_rng().gen_range(1..=100);
    if roll <= success_chance {
        let pop_gain = rand::thread_rng().gen_range(2..=5);
        let rep_gain = rand::thread_rng().gen_range(1..=3);
        println!("{GREEN}You successfully dodge and impress your peers!{RESET}");
        player.popularity += pop_gain;
        player.reputation += rep_gain;
    } else {
        let money_loss = rand::thread_rng().gen_range(3..=7);
        let attn_gain = rand::thread_rng().gen_range(1..=4);
        let pop_loss = rand::thread_rng().gen_range(1..=3);
        println!("{RED}You get roughed up and lose some money.{RESET}");
        player.money -= money_loss;
        player.attention += attn_gain;
        player.popularity -= pop_loss;
    }
}

// Event: Attempt a milk heist in the cafeteria
fn milk_heist(player: &mut Player) {
    println!("You attempt a milk heist in the cafeteria...");
    let success_chance = player.int_ as i32 * 8 + player.str_ as i32 * 4 + 30;
    let roll = rand::thread_rng().gen_range(1..=100);
    if roll <= success_chance {
        let money_gain = rand::thread_rng().gen_range(8..=15);
        let attn_gain = rand::thread_rng().gen_range(1..=4);
        let rep_gain = rand::thread_rng().gen_range(1..=2);
        // println!("Success! You sell the milk on the black market.");
        println!(" {GREEN}Success! You sell the milk on the black market.{RESET}");
        player.money += money_gain;
        player.attention += attn_gain;
        player.reputation += rep_gain;
    } else {
        let money_loss = rand::thread_rng().gen_range(2..=5);
        let attn_gain = rand::thread_rng().gen_range(3..=6);
        let pop_loss = rand::thread_rng().gen_range(1..=2);
        // println!("Caught by the lunch lady! You lose money and gain attention.");
        println!("{RED}Caught by the lunch lady! You lose money and gain attention.{RESET}");
        player.money -= money_loss;
        player.attention += attn_gain;
        player.popularity -= pop_loss;
    }
}

// Event: Attempt to cheat on a test
fn test_cheating(player: &mut Player) {
    println!("You try to cheat on a test...");
    let success_chance = player.int_ as i32 * 10 + player.cha as i32 * 5;
    let roll = rand::thread_rng().gen_range(1..=100);
    if roll <= success_chance {
        let grades_gain = rand::thread_rng().gen_range(3..=7);
        let pop_gain = rand::thread_rng().gen_range(1..=3);
        let rep_gain = rand::thread_rng().gen_range(1..=2);
        println!("You ace the test! Grades and popularity up.");
        player.grades += grades_gain;
        player.popularity += pop_gain;
        player.reputation += rep_gain;
    } else {
        let grades_loss = rand::thread_rng().gen_range(3..=7);
        let money_loss = rand::thread_rng().gen_range(3..=7);
        let attn_gain = rand::thread_rng().gen_range(4..=7);
        let pop_loss = rand::thread_rng().gen_range(1..=3);
        println!("You get caught! Grades and money down, attention up.");
        player.grades -= grades_loss;
        player.money -= money_loss;
        player.attention += attn_gain;
        player.popularity -= pop_loss;
    }
}

// Event: Attempt to flirt with Charlotte
fn charlotte_flirt(player: &mut Player) {
    if player.popularity < 20 {
        println!(
            "\n{}",
            "Sadly, you do not have enough popularity to try for Charlotte's heart!"
                .bold()
                .bright_cyan()
                .on_bright_magenta()
        ); // was bright_magenta()
        return;
    }
    println!("You try to flirt with Charlotte...");
    let success_chance = player.cha as i32 * 12 + player.popularity + 20;
    let roll = rand::thread_rng().gen_range(1..=100);
    if roll <= success_chance {
        let rel_gain = rand::thread_rng().gen_range(1..=3);
        let pop_gain = rand::thread_rng().gen_range(1..=2);
        let rep_gain = rand::thread_rng().gen_range(1..=2);
        println!("Charlotte is impressed! Relationship up.");
        player.relationship_charlotte += rel_gain;
        player.popularity += pop_gain;
        player.reputation += rep_gain;
    } else {
        let rel_loss = rand::thread_rng().gen_range(1..=2);
        let pop_loss = rand::thread_rng().gen_range(1..=2);
        println!("Charlotte is not impressed. Ouch.");
        player.relationship_charlotte -= rel_loss;
        player.popularity -= pop_loss;
    }
}

// Event: Visit the item shop (stub for expansion)
fn item_shop(player: &mut Player) {
    println!("The item shop is under construction.");
    let items = get_shop_items();
    println!("Available items:");
    for item in &items {
        println!(
            "{} - {} (Cost: ${})",
            item.name.bright_yellow(),
            item.description.bright_white(),
            item.cost
        );
    }

    let choice = get_input("Enter the name of the item to buy, or 'exit' to leave: ");
    if choice.trim().to_lowercase() == "exit" {
        return;
    }

    // Find the item
    let item = items.iter().find(|i| i.name == choice.trim());
    match item {
        Some(i) => {
            // Check if player has enough money
            if player.money >= i.cost {
                // Apply the item's effects
                i.apply(player);
                println!(
                    "You bought {}! {}",
                    i.name.bright_yellow(),
                    "Enjoy your new item!".green()
                );
            } else {
                println!("{RED}You don't have enough money!{RESET}");
            }
        }
        None => println!("{RED}Item not found.{RESET}"),
    }
}

// Event: Random event placeholder (expand as needed)
fn random_event(player: &mut Player) {
    let event = get_random_event();
    // Print event description
    println!("\n{}\n", event.description.bold().bright_white().on_blue());

    // Print choices with color
    for (i, choice) in event.choices.iter().enumerate() {
        println!(
            "{} {}",
            format!("[{}]", i + 1).bold().bright_yellow().on_red(),
            choice.description.bright_white().on_green()
        );
    }

    // Prompt for input
    println!("{}", "Enter your choice:".bold().bright_cyan().on_yellow());

    let choice_num = get_input("Choose an option: ");
    if let Ok(index) = choice_num.parse::<usize>() {
        if index > 0 && index <= event.choices.len() {
            // Map Player to PlayerStats
            let mut stats = PlayerStats {
                money: player.money,
                popularity: player.popularity,
                grades: player.grades,
            };
            let choice = &event.choices[index - 1];
            let mut rng = rand::thread_rng();
            let outcome_message = (choice.effect)(&mut stats, &mut rng);

            // After a choice is made
            println!(
                "\n {} {}",
                " You chose:".bold().green(),
                choice.description.green().bold()
            );

            // Print outcome message with color based on result
            if outcome_message.contains("gained") || outcome_message.contains("won") {
                println!(
                    "  {} {}",
                    " 🍔 Outcome:".bold().green(),
                    outcome_message.green().bold()
                );
            } else if outcome_message.contains("lost") || outcome_message.contains("caught") {
                println!(
                    "{} {}",
                    "🍔 Outcome:".bold().red(),
                    outcome_message.red().bold()
                );
            } else {
                println!(
                    "{} {}",
                    "🍔 Outcome:".bold().cyan(),
                    outcome_message.cyan().bold()
                );
            }

            // Write back changes
            player.money = stats.money;
            player.popularity = stats.popularity;
            player.grades = stats.grades;
        } else {
            println!("Invalid choice.");
        }
    } else {
        println!("Invalid input.");
    }
// begin of report player stats
    println!(
        "\n{}",
        "  Current Player Stats:".bold().bright_blue().on_bright_white()
    );
    show_status(player);
// end of report player stats

}

// Event: Bullies or principal may show up after certain actions (stub)
fn bullies_or_principal_event(_player: &mut Player, _bullies: &[Bully], _context: &str) {
    // Implement as needed

// choose whether it is the principal or bullies that confront the player
    if rand::thread_rng().gen_bool(0.5) {
        // println!("{RED}The principal confronts you!{RESET}");
        confront_principal(_player);
    } else {
        // println!("{RED}You are confronted by bullies!{RESET}");
        confront_bullies(_player, _bullies);
    }


}

// Event: Principal confrontation after skipping day (stub)
// fn principal_confrontation(_player: &mut Player) {
// TODO
//     println!("The principal confronts you! (Not yet implemented)");
// }

// Utility: Get input from the user
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim_end().to_string()
}

// Confront bullies: fight or distract to escape
fn confront_bullies(player: &mut Player, bullies: &[Bully]) {
    let bully = &bullies[rand::thread_rng().gen_range(0..bullies.len())];
    println!("  {RED}You are confronted by bully {}!{RESET}", bully.name);

    loop {
        println!(" Do you want to:");
        println!("  1. Fight");
        println!("  2. Throw a paper airplane to distract and escape");
        let choice = get_input("Choose (1 or 2): ");
        match choice.trim() {
            "1" => {
                let fight_chance = player.str_ as i32 * 10 + player.reputation * 2 + 30;
                let roll = rand::thread_rng().gen_range(1..=100);
                if roll <= fight_chance {
                    let pop_gain = rand::thread_rng().gen_range(3..=7);
                    println!("{GREEN}You win the fight! Your popularity soars ({POP_GAIN}) point!{RESET}",POP_GAIN = pop_gain);
                    player.popularity += pop_gain;
                    break;
                } else {
                    let damage = rand::thread_rng().gen_range(6..=12);
                    let attn_gain = rand::thread_rng().gen_range(2..=5);
                    println!("  {RED}You lose the fight and get roughed up!{RESET}");
                    player.money -= damage;
                    player.attention += attn_gain;
                    // Optionally allow to continue fighting or escape
                    println!("  Do you want to keep fighting (1) or try to escape (2)?");
                    let next = get_input("  Choose (1 or 2): ");
                    if next.trim() == "2" {
                        continue; // Go back to the loop for distraction option
                    }
                }
            }
            "2" => {
                let distract_chance = player.int_ as i32 * 8 + player.cha as i32 * 5 + 20;
                let roll = rand::thread_rng().gen_range(1..=100);
                if roll <= distract_chance {
                    println!(". {CYAN}Your distraction works! You escape the bullies.{RESET}");
                    break;
                } else {
                    let pop_loss = rand::thread_rng().gen_range(1..=3);
                    println!("  {RED}Your distraction fails. The bullies catch you!{RESET}");
                    player.popularity -= pop_loss;
                    // Optionally, loop back for another choice
                }
            }
            _ => println!("  Invalid choice."),
        }
    }
}

// Confront principal: fight or distract to escape
fn confront_principal(player: &mut Player) {
    println!(". {RED}You are confronted by the principal!{RESET}");

    loop {
        println!(" Do you want to:");
        println!("  1. Fight");
        println!("  2. Throw a paper airplane to distract and escape");
        let choice = get_input("Choose (1 or 2): ");
        match choice.trim() {
            "1" => {
                let fight_chance = player.str_ as i32 * 8 + player.reputation * 2 + 20;
                let roll = rand::thread_rng().gen_range(1..=100);
                if roll <= fight_chance {
                    let money_gain = rand::thread_rng().gen_range(10..=20);
                    let popularity_gain = rand::thread_rng().gen_range(5..=10);
                    println!("{GREEN}You somehow win! You find some confiscated cash ({MONEY_GRAIN}) and earn some popularity ({POP_GAIN}) with your peers!{RESET}",POP_GAIN = popularity_gain, MONEY_GRAIN = money_gain);
                    player.money += money_gain;
                    player.popularity += popularity_gain;
                    break;
                } else {
                    let attn_gain = rand::thread_rng().gen_range(4..=8);
                    let warning = 1;
                    println!(
                        "{RED}You lose! The principal gives you a warning and extra attention!{RESET}"
                    );
                    player.attention += attn_gain;
                    player.warnings += warning;
                    if player.warnings >= 3 {
                        player.expelled = true;
                        println!("{RED}You have been expelled!{RESET}");
                    }
                    // Optionally allow to continue fighting or escape
                    println!("Do you want to keep fighting (1) or try to escape (2)?");
                    let next = get_input("Choose (1 or 2): ");
                    if next.trim() == "2" {
                        continue; // Go back to the loop for distraction option
                    }
                }
            }
            "2" => {
                let distract_chance = player.int_ as i32 * 10 + player.cha as i32 * 5 + 25;
                let roll = rand::thread_rng().gen_range(1..=100);
                if roll <= distract_chance {
                    println!("{CYAN}Your distraction works! You escape the principal.{RESET}");
                    break;
                } else {
                    let attn_gain = rand::thread_rng().gen_range(2..=5);
                    println!("{RED}Your distraction fails. The principal is not amused!{RESET}");
                    player.attention += attn_gain;
                    // Optionally, loop back for another choice
                }
            }
            _ => println!("Invalid choice."),
        }
    }
}

#[derive(Clone)]
pub struct Event {
    pub description: &'static str,
    pub choices: Vec<Choice>,
}

#[derive(Clone)]
pub struct Choice {
    pub description: &'static str,
    pub effect: fn(&mut PlayerStats, &mut rand::rngs::ThreadRng) -> String,
}

// Example event list with random increments/decrements
pub fn get_random_event() -> Event {
    // use rand::seq::SliceRandom; //defined above

    let events = vec![
        Event {
            description: "Sudden pop quiz in history class and you did not study! Should you cheat and risk getting caught or take your chances?",
            choices: vec![
                Choice {
                    description: "Cheat on the quiz",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.5) {
                            let gain = rng.gen_range(1..=3);
                            stats.grades += gain;
                            format!("You cheated successfully and gained {} grades!", gain)

                            // let msg = format!("You cheated successfully and gained {} grades!", gain);
                            // println!("{GREEN}{}{RESET}", msg);
                        } else {
                            let loss = rng.gen_range(1..=2);
                            stats.popularity -= loss;
                            format!("You got caught cheating and lost {} popularity!", loss)
                        }
                    },
                },
                Choice {
                    description: "Take your chances",
                    effect: |stats, rng| {
                        let loss = rng.gen_range(1..=2);
                        stats.grades -= loss;
                        format!("You didn't study and lost {} grades.", loss)
                    },
                },
            ], // <-- CLOSE choices vec!
        }, // <-- CLOSE Event
        Event {
            description: "Everyone wants extra milk at lunch today! Should you complete a milk heist to sell during lunch at 3 dollars a carton?",
            choices: vec![
                Choice {
                    description: "Attempt the milk heist",
                    effect: |stats, rng| {
                        let milk_sold = rng.gen_range(1..=4);
                        let money_gained = milk_sold * 3;
                        let pop_gain = rng.gen_range(1..=2);
                        stats.money += money_gained;
                        stats.popularity += pop_gain;
                        format!(
                            "You sold {} cartons and gained ${} and {} popularity!",
                            milk_sold, money_gained, pop_gain
                        )
                    },
                },
                Choice {
                    description: "Ignore the milk craze",
                    effect: |_stats, _rng| {
                        "You ignored the milk craze. Nothing happened.".to_string()
                    },
                },
            ],
        },
        Event {
            description: "Bullies are playing basketball in the gym and look tired! You could easily fight them and win some easy popularity points.",
            choices: vec![
                Choice {
                    description: "Fight the bullies",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.6) {
                            let pop_gain = rng.gen_range(2..=5);
                            let money_gain = rng.gen_range(1..=3);
                            stats.popularity += pop_gain;
                            stats.money += money_gain;
                            format!(
                                "You won the fight! Gained {} popularity and ${}.",
                                pop_gain, money_gain
                            )
                        } else {
                            let pop_loss = rng.gen_range(1..=3);
                            let money_loss = rng.gen_range(1..=2);
                            stats.popularity -= pop_loss;
                            stats.money -= money_loss;
                            format!(
                                "You lost the fight! Lost {} popularity and ${}.",
                                pop_loss, money_loss
                            )
                        }
                    },
                },
                Choice {
                    description: "Avoid the bullies",
                    effect: |_stats, _rng| "You avoided the bullies. Nothing happened.".to_string(),
                },
            ],
        },
        Event {
            description: "The Principal looks hung over from last night's faculty party. You could easily attack and fight the principal to win some easy money.",
            choices: vec![
                Choice {
                    description: "Fight the principal",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.4) {
                            let money_gain = rng.gen_range(3..=8);
                            let pop_gain = rng.gen_range(1..=2);
                            stats.money += money_gain;
                            stats.popularity += pop_gain;
                            format!(
                                "You beat the principal! Gained ${} and {} popularity.",
                                money_gain, pop_gain
                            )
                        } else {
                            let money_loss = rng.gen_range(2..=4);
                            let pop_loss = rng.gen_range(1..=3);
                            stats.money -= money_loss;
                            stats.popularity -= pop_loss;
                            format!(
                                "You lost to the principal! Lost ${} and {} popularity.",
                                money_loss, pop_loss
                            )
                        }
                    },
                },
                Choice {
                    description: "Leave the principal alone",
                    effect: |_stats, _rng| {
                        "You left the principal alone. Nothing happened.".to_string()
                    },
                },
            ],
        },
        //fire drill
        Event {
            description: "A surprise fire drill interrupts your math test! Should you sneak out to the vending machines for snacks, or follow the rules and line up outside?",
            choices: vec![
                Choice {
                    description: "Sneak out for snacks",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.4) {
                            let money_gain = rng.gen_range(3..=8);
                            let pop_gain = rng.gen_range(1..=2);
                            stats.money -= money_gain;
                            stats.popularity += pop_gain;
                            format!(
                                "You bought snacks for yourself and the class. Spent ${} and gained {} popularity.",
                                money_gain, pop_gain
                            )
                        } else {
                            let money_loss = rng.gen_range(2..=4);
                            let pop_loss = rng.gen_range(1..=3);
                            stats.money -= money_loss;
                            stats.popularity -= pop_loss;
                            format!(
                                "You got caught by the teacher and had to pay her off! Lost ${} and {} popularity.",
                                money_loss, pop_loss
                            )
                        }
                    },
                },
                Choice {
                    description: "Follow the rules",
                    effect: |_stats, _rng| {
                        "Class resumes and all is back to normal. Nothing happened.".to_string()
                    },
                },
            ],
        },
        Event {
            description: "The school's outdoor speakers start playing embarrassing songs from your past middle school years, like 'Baby Shark' or 'The Chicken Dance'. Do you try to escape through the auditorium doors, or attempt a choreographed dance routine to distract everyone?",
            choices: vec![
                Choice {
                    description: "Escape through the auditorium doors",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.5) {
                            let pop_loss = rng.gen_range(1..=3);
                            stats.popularity -= pop_loss;
                            format!(
                                "You trip on the way out and everyone laughs! Lost {} popularity.",
                                pop_loss
                            )
                        } else {
                            let pop_gain = rng.gen_range(1..=2);
                            stats.popularity += pop_gain;
                            format!(
                                "You slip out unnoticed and feel relieved. Gained {} popularity for your stealth!",
                                pop_gain
                            )
                        }
                    },
                },
                Choice {
                    description: "Attempt a choreographed dance routine",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.6) {
                            let pop_gain = rng.gen_range(2..=5);
                            stats.popularity += pop_gain;
                            format!("Your dance goes viral! Gained {} popularity.", pop_gain)
                        } else {
                            let pop_loss = rng.gen_range(1..=2);
                            stats.popularity -= pop_loss;
                            format!(
                                "You forget the moves and everyone cringes. Lost {} popularity.",
                                pop_loss
                            )
                        }
                    },
                },
            ],
        },
        Event {
            description: "Your homework folder gets mixed up with a rival student's and they end up returning it with their own assignments inside. Do you laugh off the mistake and start working on their math problems, or frantically try to sort out the mess before your teacher sees?",
            choices: vec![
                Choice {
                    description: "Laugh off the mistake and do their math problems",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.5) {
                            let grades_gain = rng.gen_range(1..=3);
                            stats.grades += grades_gain;
                            format!(
                                "You ace their math problems! Gained {} grades.",
                                grades_gain
                            )
                        } else {
                            let grades_loss = rng.gen_range(1..=2);
                            stats.grades -= grades_loss;
                            format!("Their math is way harder! Lost {} grades.", grades_loss)
                        }
                    },
                },
                Choice {
                    description: "Frantically sort out the mess",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.5) {
                            let pop_gain = rng.gen_range(1..=2);
                            stats.popularity += pop_gain;
                            format!("You fix it just in time! Gained {} popularity.", pop_gain)
                        } else {
                            let pop_loss = rng.gen_range(1..=2);
                            stats.popularity -= pop_loss;
                            format!(
                                "The teacher catches you panicking. Lost {} popularity.",
                                pop_loss
                            )
                        }
                    },
                },
            ],
        },
        Event {
            description: "The school cafeteria starts serving pizza with an unusual topping, like Brussels sprouts or durian fruit. Do you politely decline the offer and order a salad, or take it as a dare and become an unlikely champion of exotic cuisine?",
            choices: vec![
                Choice {
                    description: "Politely decline and order a salad",
                    effect: |stats, rng| {
                        let pop_loss = rng.gen_range(1..=2);
                        stats.popularity -= pop_loss;
                        format!(
                            "Everyone thinks you're boring. Lost {} popularity.",
                            pop_loss
                        )
                    },
                },
                Choice {
                    description: "Take the dare and eat the pizza",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.6) {
                            let pop_gain = rng.gen_range(2..=4);
                            stats.popularity += pop_gain;
                            format!(
                                "You become a legend for your bravery! Gained {} popularity.",
                                pop_gain
                            )
                        } else {
                            let money_loss = rng.gen_range(1..=3);
                            stats.money -= money_loss;
                            format!(
                                "You get sick and have to buy medicine. Lost ${}.",
                                money_loss
                            )
                        }
                    },
                },
            ],
        },
        Event {
            description: "A popular student's prized possession, like their favorite video game console or collectible figurine, goes missing during lunch. Do you secretly search for it with your friend, or pretend to be outraged and storm into the principal's office?",
            choices: vec![
                Choice {
                    description: "Secretly search for it with your friend",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.5) {
                            let pop_gain = rng.gen_range(2..=4);
                            stats.popularity += pop_gain;
                            format!(
                                "You find the item and become a hero! Gained {} popularity.",
                                pop_gain
                            )
                        } else {
                            let pop_loss = rng.gen_range(1..=2);
                            stats.popularity -= pop_loss;
                            format!("You get caught snooping. Lost {} popularity.", pop_loss)
                        }
                    },
                },
                Choice {
                    description: "Storm into the principal's office",
                    effect: |stats, rng| {
                        let attn_gain = rng.gen_range(1..=3);
                        stats.grades -= attn_gain;
                        format!(
                            "The principal is not amused. Lost {} grades for missing class.",
                            attn_gain
                        )
                    },
                },
            ],
        },
        Event {
            description: "The school's gym team gets mistakenly scheduled to play a basketball tournament against an opposing school that's actually much better than them. Do you try to psych yourself up by listening to motivational speeches on YouTube, or quietly wonder if it was all just a cruel joke?",
            choices: vec![
                Choice {
                    description: "Listen to motivational speeches",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.5) {
                            let pop_gain = rng.gen_range(1..=3);
                            stats.popularity += pop_gain;
                            format!("Your team is inspired! Gained {} popularity.", pop_gain)
                        } else {
                            let grades_loss = rng.gen_range(1..=2);
                            stats.grades -= grades_loss;
                            format!(
                                "You spend too much time online and forget homework. Lost {} grades.",
                                grades_loss
                            )
                        }
                    },
                },
                Choice {
                    description: "Wonder if it's a cruel joke",
                    effect: |stats, rng| {
                        let pop_loss = rng.gen_range(1..=2);
                        stats.popularity -= pop_loss;
                        format!(
                            "Your lack of spirit is contagious. Lost {} popularity.",
                            pop_loss
                        )
                    },
                },
            ],
        },
        Event {
            description: "A group of seniors plan a surprise party for the school's new teacher, complete with balloons and a cake. However, they accidentally invite the wrong person – the janitor – who starts crying because he thought it was his birthday. Do you try to maintain the charade or reveal the truth?",
            choices: vec![
                Choice {
                    description: "Maintain the charade",
                    effect: |stats, rng| {
                        let pop_gain = rng.gen_range(1..=3);
                        stats.popularity += pop_gain;
                        format!(
                            "The janitor is thrilled! Gained {} popularity for kindness.",
                            pop_gain
                        )
                    },
                },
                Choice {
                    description: "Reveal the truth",
                    effect: |stats, rng| {
                        let pop_loss = rng.gen_range(1..=2);
                        stats.popularity -= pop_loss;
                        format!("The janitor is embarrassed. Lost {} popularity.", pop_loss)
                    },
                },
            ],
        },
        Event {
            description: "The school's social media account gets hacked and a series of ridiculous posts go live, like 'We're having pizza on Fridays forever!' or 'Our school mascot is secretly a superhero!'. Do you quickly come up with an apology video to salvage the situation, or post your own cheeky memes?",
            choices: vec![
                Choice {
                    description: "Make an apology video",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.5) {
                            let pop_gain = rng.gen_range(1..=3);
                            stats.popularity += pop_gain;
                            format!(
                                "Your video is well received! Gained {} popularity.",
                                pop_gain
                            )
                        } else {
                            let pop_loss = rng.gen_range(1..=2);
                            stats.popularity -= pop_loss;
                            format!(
                                "No one believes your apology. Lost {} popularity.",
                                pop_loss
                            )
                        }
                    },
                },
                Choice {
                    description: "Post your own cheeky memes",
                    effect: |stats, rng| {
                        let pop_gain = rng.gen_range(1..=3);
                        stats.popularity += pop_gain;
                        format!("Your memes go viral! Gained {} popularity.", pop_gain)
                    },
                },
            ],
        },
        Event {
            description: "A teacher announces that there will be a surprise pop quiz during class, but they conveniently forget to reveal the subject. Do you spend the entire class trying to guess what it is and frantically look through notes, or pretend to fall asleep with confidence?",
            choices: vec![
                Choice {
                    description: "Frantically look through notes",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.5) {
                            let grades_gain = rng.gen_range(1..=3);
                            stats.grades += grades_gain;
                            format!(
                                "You guessed right and aced the quiz! Gained {} grades.",
                                grades_gain
                            )
                        } else {
                            let grades_loss = rng.gen_range(1..=2);
                            stats.grades -= grades_loss;
                            format!(
                                "You studied the wrong subject. Lost {} grades.",
                                grades_loss
                            )
                        }
                    },
                },
                Choice {
                    description: "Pretend to fall asleep",
                    effect: |stats, rng| {
                        let pop_gain = rng.gen_range(1..=2);
                        stats.popularity += pop_gain;
                        format!(
                            "Everyone admires your confidence. Gained {} popularity.",
                            pop_gain
                        )
                    },
                },
                //
                Choice {
                    description: "Cheat on the quiz (which may hurt your popularity!)",
                    effect: |stats, rng| {
                        let pop_gain = rng.gen_range(-2..=2);
                        stats.popularity += pop_gain;
                        format!(
                            "You sneakily copy answers from members of the course. They might have noticed! Gained {} popularity.",
                            pop_gain
                        )
                    },
                },
                //
            ],
        },
        Event {
            description: "The school's annual talent show takes place at night, but due to a technical issue, the stage lights start flashing in sync with 'Who Let the Dogs Out?'. Do you laugh off the mishap and enjoy the impromptu dance party, or try to politely ask the tech team if they can just turn it off?",
            choices: vec![
                Choice {
                    description: "Laugh off the mishap and dance",
                    effect: |stats, rng| {
                        let pop_gain = rng.gen_range(2..=4);
                        stats.popularity += pop_gain;
                        format!("You start the party! Gained {} popularity.", pop_gain)
                    },
                },
                Choice {
                    description: "Ask the tech team to turn it off",
                    effect: |stats, rng| {
                        let pop_loss = rng.gen_range(1..=2);
                        stats.popularity -= pop_loss;
                        format!(
                            "Everyone boos you for being a buzzkill. Lost {} popularity.",
                            pop_loss
                        )
                    },
                },
            ],
        },
        Event {
            description: "A school tradition goes horribly wrong, like trying to break a record for most people simultaneously doing a choreographed dance routine. Instead, everyone ends up in a tangled heap on the floor and laughing hysterically. Do you grab your phone to take a viral video shot, or get caught up in the commotion and join in?",
            choices: vec![
                Choice {
                    description: "Take a viral video shot",
                    effect: |stats, rng| {
                        let money_gain = rng.gen_range(2..=5);
                        let popularity_gain = rng.gen_range(1..=3);
                        stats.popularity += popularity_gain;
                        stats.money += money_gain;
                        format!(
                            "Your video goes viral! Gained ${} from friends  and {} popularity points",
                            money_gain, stats.popularity
                        )
                    },
                },
                Choice {
                    description: "Join in the commotion",
                    effect: |stats, rng| {
                        let pop_gain = rng.gen_range(1..=3);
                        stats.popularity += pop_gain;
                        format!("You have a blast! Gained {} popularity.", pop_gain)
                    },
                },
            ],
        },
        Event {
            description: "The principal announces a surprise 'Dress Like Your Favorite Teacher' day, but you forgot to prepare. Do you quickly throw on a lab coat and glasses to impersonate the science teacher, or just wear your regular clothes and hope no one notices?",
            choices: vec![
                Choice {
                    description: "Impersonate the science teacher",
                    effect: |stats, rng| {
                        let pop_gain = rng.gen_range(1..=3);
                        stats.popularity += pop_gain;
                        format!(
                            "Everyone loves your last-minute costume! Gained {} popularity.",
                            pop_gain
                        )
                    },
                },
                Choice {
                    description: "Wear regular clothes",
                    effect: |stats, rng| {
                        let pop_loss = rng.gen_range(1..=2);
                        stats.popularity -= pop_loss;
                        format!(
                            "You stand out for the wrong reasons. Lost {} popularity.",
                            pop_loss
                        )
                    },
                },
            ],
        },
        Event {
            description: "The school WiFi goes down during computer class! Do you offer to 'fix' it for a fee, or use the time to nap?",
            choices: vec![
                Choice {
                    description: "Offer to fix it for a fee",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.5) {
                            let money_gain = rng.gen_range(2..=6);
                            stats.money += money_gain;
                            format!("You fix the WiFi and get paid! Gained ${}.", money_gain)
                        } else {
                            let pop_loss = rng.gen_range(1..=2);
                            stats.popularity -= pop_loss;
                            format!("You make it worse! Lost {} popularity.", pop_loss)
                        }
                    },
                },
                Choice {
                    description: "Use the time to nap",
                    effect: |stats, rng| {
                        let grades_loss = rng.gen_range(1..=2);
                        stats.grades -= grades_loss;
                        format!("You miss an important lesson. Lost {} grades.", grades_loss)
                    },
                },
            ],
        },
        Event {
            description: "The school mascot's costume is missing! Do you volunteer to wear it for the pep rally, or try to sell it to a rival school?",
            choices: vec![
                Choice {
                    description: "Volunteer to wear the costume",
                    effect: |stats, rng| {
                        let pop_gain = rng.gen_range(2..=4);
                        stats.popularity += pop_gain;
                        format!(
                            "You become the star of the pep rally! Gained {} popularity.",
                            pop_gain
                        )
                    },
                },
                Choice {
                    description: "Try to sell it to a rival school",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.4) {
                            let money_gain = rng.gen_range(5..=10);
                            stats.money += money_gain;
                            format!("You make a sneaky deal! Gained ${}.", money_gain)
                        } else {
                            let pop_loss = rng.gen_range(2..=4);
                            stats.popularity -= pop_loss;
                            format!("You get caught! Lost {} popularity.", pop_loss)
                        }
                    },
                },
            ],
        },
        Event {
            description: "A rumor spreads that a famous pop star is visiting the school. Do you skip class to search for them, or stay put and avoid trouble?",
            choices: vec![
                Choice {
                    description: "Skip class to search",
                    effect: |stats, rng| {
                        if rng.gen_bool(0.3) {
                            let pop_gain = rng.gen_range(3..=6);
                            stats.popularity += pop_gain;
                            format!(
                                "You actually meet the pop star! Gained {} popularity.",
                                pop_gain
                            )
                        } else {
                            let grades_loss = rng.gen_range(2..=4);
                            stats.grades -= grades_loss;
                            format!("You get caught skipping. Lost {} grades.", grades_loss)
                        }
                    },
                },
                Choice {
                    description: "Stay put and avoid trouble",
                    effect: |stats, rng| {
                        let pop_loss = rng.gen_range(1..=2);
                        stats.popularity -= pop_loss;
                        format!(
                            "Everyone else has fun without you. Lost {} popularity.",
                            pop_loss
                        )
                    },
                },
            ],
        },
    ]; // OBC CLOSE events vec!

    events.choose(&mut rand::thread_rng()).unwrap().clone()
}

// Shop items for the item shop
pub fn get_shop_items() -> Vec<ShopItem> {
    vec![
        ShopItem {
            name: "Designer Deodorant",
            description: "Smell like you showered, even if you didn’t.",
            cost: 10,
            popularity_change: 5,
            grades_change: 0,
            money_change: 0,
        },
        ShopItem {
            name: "Homework Subscription Service",
            description: "Why do your own work when you can rent someone else’s brain?",
            cost: 30,
            popularity_change: -2,
            grades_change: 8,
            money_change: 0,
        },
        ShopItem {
            name: "Limited Edition Meme T-Shirt",
            description: "Because nothing says ‘cool’ like a joke that’s already old.",
            cost: 15,
            popularity_change: 4,
            grades_change: -1,
            money_change: 0,
        },
        ShopItem {
            name: "Energy Drink 6-Pack",
            description: "Stay awake in class, or at least vibrate through it.",
            cost: 12,
            popularity_change: 0,
            grades_change: 3,
            money_change: 0,
        },
        ShopItem {
            name: "Fake Designer Backpack",
            description: "Looks expensive, falls apart instantly.",
            cost: 20,
            popularity_change: 3,
            grades_change: 0,
            money_change: 0,
        },
        ShopItem {
            name: "Influencer Starter Kit",
            description: "Ring light, selfie stick, and a dream.",
            cost: 25,
            popularity_change: 2,
            grades_change: 0,
            money_change: 5, // Potential to make money
        },
        ShopItem {
            name: "Mystery Meat Cafeteria Coupon",
            description: "Eat it on a dare, become a legend.",
            cost: 5,
            popularity_change: 6,
            grades_change: -2,
            money_change: 0,
        },
        ShopItem {
            name: "Study Buddy Plush Toy",
            description: "Because you need someone to blame for your bad grades.",
            cost: 8,
            popularity_change: 2,
            grades_change: 2,
            money_change: 0,
        },
        ShopItem {
            name: "Hall Pass Forgery Kit",
            description: "For when you need to be anywhere but class.",
            cost: 18,
            popularity_change: -3,
            grades_change: -4,
            money_change: 10, // Sell passes
        },
        ShopItem {
            name: "Cool Kid Sunglasses",
            description: "Worn indoors for maximum irony.",
            cost: 14,
            popularity_change: 5,
            grades_change: -1,
            money_change: 0,
        },
        // Negative impact items
        ShopItem {
            name: "Suspicious Energy Pills",
            description: "Guaranteed to keep you up... or knock you out.",
            cost: 7,
            popularity_change: -2,
            grades_change: -3,
            money_change: 0,
        },
        ShopItem {
            name: "Unlicensed Mixtape",
            description: "Play it loud, lose friends fast.",
            cost: 3,
            popularity_change: -5,
            grades_change: 0,
            money_change: 0,
        },
        ShopItem {
            name: "Counterfeit Lunch Ticket",
            description: "Risk it for a free meal.",
            cost: 2,
            popularity_change: -3,
            grades_change: -2,
            money_change: 5,
        },
        ShopItem {
            name: "Cheesy Pickup Line Book",
            description: "For when you want to be remembered... for the wrong reasons.",
            cost: 6,
            popularity_change: -4,
            grades_change: 0,
            money_change: 0,
        },
    ]
}

// use std::io::{self, Write};

pub fn shop_menu(player: &mut Player) {
    let items = get_shop_items();

    println!("\n--- School Shop ---");
    for (i, item) in items.iter().enumerate() {
        println!(
            "{}. {} ({} coins)\n   {}\n",
            i + 1,
            item.name,
            item.cost,
            item.description
        );
    }
    println!("0. Exit shop");

    loop {
        print!("Select an item to purchase (by number): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if trimmed == "0" {
            println!("Leaving the shop.\n");
            break;
        }

        match trimmed.parse::<usize>() {
            Ok(num) if num >= 1 && num <= items.len() => {
                let item = &items[num - 1];
                // Check if player has enough money
                if player.money >= item.cost {
                    // Apply the item's effects
                    item.apply(player);
                    println!(
                        "You bought {}! (Popularity: {}, Grades: {}, Money: {})\n",
                        item.name, player.popularity, player.grades, player.money
                    );
                } else {
                    println!("Not enough money for {}!\n", item.name);
                }
            }
            _ => println!("Invalid selection. Try again.\n"),
        }
    }
}

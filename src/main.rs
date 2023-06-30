use colored::Colorize;
use std::io;

struct Character {
    name: String,
    gender: bool,
    experience: u32,
    level: u32,
}

impl Character {
    fn new(name: String) -> Character {
        Character {
            name,
            gender: true,
            experience: 0,
            level: 0,
        }
    }
}

fn main() {
    // ⫷⫸⋙↪⇾⇢⇨→⁜†▶◄►◁▷○◜◝◢◣◧◨◯
    println!(
        "{}\n{}\n{}",
        "(!) Green ooze drips from the ceiling (!)".red(),
        "★ Settings ★".magenta(),
        "<Deserted Village>".yellow(),
    );
    /*
    println!(
        "{}",
        "You're playing Miningcow's awesome text adventure game!".green()
    );

    if !check_if_player_allowed() {
        println!(
            "{}",
            "Sorry, you cannot play the game at this time.".yellow()
        );
        return;
    }

    let mut player = character_creation();
    println!(
        "{} {}!",
        "Welcome to the ill mind of Miningcow,".green(),
        player.name.green()
    );

    loop {
        println!("input: ");
        let input = get_input();

        println!("You inputted: {input}. There is no game yet :P CTRL + C to quit.");
    }
    */
}

fn check_if_player_allowed() -> bool {
    loop {
        println!(
            "Please enter your real life gender. {} or {}",
            "[M]".bright_blue(),
            "[F]".bright_magenta()
        );

        let gender = get_input();

        if gender == String::from("f") {
            break false;
        } else if gender == String::from("m") {
            break true;
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_lowercase()
}

fn character_creation() -> Character {
    println!("{}", "<Character Creation>".green().bold());

    println!("Name: ");
    let name = get_input();

    Character::new(name)
}

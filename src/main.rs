use std::collections::HashMap;
use std::io::{self, Write};

mod modules 
{
    pub mod pet;
    pub mod food;
    pub mod games;
}

use modules::pet::{pets_list, is_dead, Pet, PetKind};
use modules::food::{initialize_food, get_food_for_pet};
use modules::games::{initialize_games, get_games, GameEffect};


fn create_new_pet() -> Pet 
{
    println!("Available pet kinds:");

    for kind in &[
        PetKind::Axalotl,
        PetKind::Cat,
        PetKind::Dog,
        PetKind::Monkey,
        PetKind::Parrot,
        PetKind::Rabbit,
        PetKind::Squirrel,
    ] {
        println!(" - {}", kind.to_string());
    }

    print!("Enter your pet's name: ");
    io::stdout().flush().unwrap();
    let mut pet_name = String::new();
    io::stdin()
        .read_line(&mut pet_name)
        .expect("Failed to read input");
    let pet_name = pet_name.trim().to_string();

    print!("Enter your pet's kind: ");
    io::stdout().flush().unwrap();
    let mut kind_str = String::new();
    io::stdin()
        .read_line(&mut kind_str)
        .expect("Failed to read input");
    let kind_str = kind_str.trim();

    let pet_kind = match PetKind::from_str(kind_str) 
    {
        Some(k) => k,
        None => 
        {
            println!("Invalid pet kind, defaulting to cat.");
            PetKind::Cat
        }
    };
    //
    let new_pet = Pet::new(pet_name, pet_kind);
    println!("\nYour pet was created!");
    new_pet.display_stats();

    if let Err(e) = new_pet.save_pet_to_file() 
    {
        println!("Warning: Failed to save pet to file: {}", e);
    }

    new_pet
}

fn load_existing_pet() -> Option<Pet> 
{
    println!("\nSaved pets list:");
    if let Ok(names) = pets_list() 
    {
        if names.is_empty() 
        {
            println!("No pets found.\n");
            return None;
        }

        for name in &names 
        {
            println!(" - {}", name);
        }
        print!("Enter the name of the pet to load: ");
        io::stdout().flush().unwrap();

        let mut pet_name = String::new();
        io::stdin().read_line(&mut pet_name).expect("Failed to read input\n");
        let pet_name = pet_name.trim();

        match Pet::load_from_name(pet_name) 
        {
            Ok(pet) => 
            {
                println!("Pet loaded successfully!\n");
                pet.display_stats();
                Some(pet)
            }
            Err(e) => 
            {
                println!("Could not load pet: {}\n", e);
                None
            }
        }
    } 
    else 
    {
        println!("Failed to read saved pets directory.\n");
        None
    }
}

fn select_pet() -> Option<Pet> 
{
    loop 
    {
        println!("\n=== PET SELECTION ===\n");
        println!("1) Create New Pet\n");
        println!("2) Load Existing Pet\n");
        println!("3) Go Back to Main Menu\n");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut selection = String::new();
        io::stdin().read_line(&mut selection).expect("Failed to read input\n");
        let selection = selection.trim();

        match selection 
        {
            "1" => 
            {
                let new_pet = create_new_pet();
                return Some(new_pet);
            }
            "2" => 
            {
                let loaded = load_existing_pet();
                if loaded.is_some() 
                {
                    return loaded;
                }
            }
            "3" => 
            {
                return None;
            }
            _ => 
            {
                println!("Invalid choice, please try again.\n");
            }
        }
    }
}

fn feed_pet(pet: &mut Pet, food_map: &HashMap<String, (u8, u8)>) 
{
    println!("\nFoods available for {}:", pet.kind().to_string());
    let valid_foods = get_food_for_pet(pet.kind());

    for f in &valid_foods 
    {
        if let Some(&(sat, en)) = food_map.get(f) 
        {
            println!(" - {} (Satiation +{}, Energy +{})", f, sat, en);
        } 
        else 
        {
            println!(" - {}", f);
        }
    }

    print!("Enter the name of the food: ");
    io::stdout().flush().unwrap();

    let mut food_choice = String::new();
    io::stdin().read_line(&mut food_choice).expect("Failed to read input");
    let food_choice = food_choice.trim();

    let success = pet.feed(food_choice, food_map);
    if !success 
    {
        println!("Feeding was unsuccessful.");
    }
}

fn play_with_pet(pet: &mut Pet, games_map: &HashMap<String, GameEffect>) -> bool 
{
    println!("\nGames available for {}:", pet.kind().to_string());
    let valid_games = get_games(pet.kind());

    for g in &valid_games 
    {
        if let Some(game) = games_map.get(g) 
        {
            println!(
                " - {} (Happiness +{}, Energy -{}, Satiation -{})",
                g, game.happiness_increase, game.energy_cost, game.satiation_cost
            );
        } 
        else 
        {
            println!(" - {}", g);
        }
    }

    print!("Enter the name of the game: ");
    io::stdout().flush().unwrap();

    let mut game_choice = String::new();
    io::stdin().read_line(&mut game_choice).expect("Failed to read input");
    let game_choice = game_choice.trim();

    let success = pet.play(game_choice, games_map);
    
    success
}

fn daily_happiness_check(pet: &mut Pet, played_today: bool) -> bool 
{
    if !played_today 
    {
        println!("{} hasn't played yet today, happiness decreased by 1.", pet.name());
        pet.set_happiness(-1);
    }
    true 
}
    

fn game_engine(pet: &mut Pet, food_map: &HashMap<String, (u8, u8)>, games_map: &HashMap<String, GameEffect>) 
{
    let mut played_today = false;

    loop 
    {
        println!("=====================");
        println!("What would you like to do?");
        println!("1) Feed");
        println!("2) Play");
        println!("3) Put to Sleep");
        println!("4) Leave Pet Alone");
        println!("5) Save & Exit");
        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Failed to read input");
        let action = action.trim();

        match action 
        {
            "1" => 
            {
                feed_pet(pet, food_map);
                played_today = daily_happiness_check(pet, played_today);
            }
            "2" => 
            {
                if play_with_pet(pet, games_map) 
                {
                    played_today = true;
                }
                played_today = daily_happiness_check(pet, played_today);
            }
            "3" => 
            {
                if pet.check_energy() 
                {
                    pet.sleep();
                    println!("{} is sleeping!", pet.name());
                    played_today = false;
                } 
                else 
                {
                    println!("Energy is over 5, can't put {} to sleep right now!", pet.name());
                }
            }
            "4" => {
                println!("How many hours will you be gone?");
                let mut hours = String::new();
                io::stdin()
                    .read_line(&mut hours)
                    .expect("Failed to read input");
                let hours: u8 = match hours.trim().parse() 
                {
                    Ok(h) => h,
                    Err(_) => {
                        println!("Invalid input. Returning to menu.");
                        continue;
                    }
                };

                println!("Would you like to leave food for {}? (yes/no)", pet.name());
                let mut leave_food = String::new();
                io::stdin()
                    .read_line(&mut leave_food)
                    .expect("Failed to read input");
                let leave_food = leave_food.trim().to_lowercase();

                if leave_food == "yes" 
                {
                    pet.leave_food_while_gone(food_map);
                }

                pet.leave_pet_alone(hours);
            }
            "5" | "exit" => 
            {
                println!("Saving pet...");
                if let Err(e) = pet.save_pet_to_file() 
                {
                    println!("Warning: Could not save pet: {}", e);
                }
                println!("Exiting to main menu...\n");
                break;
            }
            _ => 
            {
                println!("Invalid choice. Please try again.");
            }
        };

        pet.check_satiation();
        pet.check_energy();

        pet.is_health_zero();
        if pet.lives() == 0 
        {
            if is_dead(&pet) 
            {
                println!("Returning to main menu...");
                break;
            }
        }
        
        pet.display_stats();

        if is_dead(pet) 
        {
            println!("Returning to main menu...");
            break;
        }
    }
}

fn main() 
{
    let food_map = initialize_food();
    let games_map = initialize_games();

    loop 
    {
        println!("==================== TAMAGOTCHI GAME ====================");
        print!("Choose an option:\n");
        println!("1) Play");
        println!("2) Exit");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice 
        {
            "1" => 
            {
                if let Some(mut pet) = select_pet() 
                {
                    game_engine(&mut pet, &food_map, &games_map);
                }
            }
            "2" | "exit" => 
            {
                println!("Goodbye!");
                break;
            }
            _ => 
            {
                println!("Invalid choice. Please try again.\n");
            }
        }
    }
}
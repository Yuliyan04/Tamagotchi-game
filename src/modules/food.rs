use std::collections::HashMap;
use crate::modules::pet::PetKind;

//Food types created as hash tables where we have the name of the food and by the name we have its 
//satiation and energy charecteristics

//Foods names:

const FISH: &str = "Fish";
const OCTOPUS: &str = "Octopus";
const SHRIMP: &str = "Shrimp";

const MILK: &str = "Milk";
const STEAK: &str = "Steak";
const CHICKEN: &str = "Chicken";
const WHISKAS: &str = "Whiskas";
const SCOOBY_SNACKS: &str = "Scooby snacks";
const BONE: &str = "Bone";
    
const CARROT: &str = "Carrot";
const LETTUCE: &str = "Lettuce";
const SPINACH: &str = "Spinach";
const SEEDS: &str = "Seeds";

const BERRIES: &str = "Berries";
const RASPBERRIES: &str = "Raspberries";
const BANANA: &str = "Banana";

const CAKE: &str = "Cake";
const BISCUITS: &str = "Biscuits";
const POPCORN: &str = "Popcorn";

const NUTS: &str = "Nuts";
const PISTACHIO: &str = "Pistachio";

pub fn initialize_food() -> HashMap<String, (u8, u8)> 
{
    let mut food_map = HashMap::new();

    let foods = vec![
        (FISH, (3, 1)),   // (Satiation, Energy)
        (OCTOPUS, (2, 1)),
        (SHRIMP, (2, 1)),
        (MILK, (1, 1)),
        (STEAK, (3, 1)),
        (CHICKEN, (3, 2)),
        (WHISKAS, (4, 2)),
        (SCOOBY_SNACKS, (1, 1)),
        (BONE, (2, 1)),
        (CARROT, (3, 2)),
        (LETTUCE, (1, 1)),
        (SPINACH, (2, 1)),
        (SEEDS, (2, 1)),
        (BERRIES, (1, 1)),
        (RASPBERRIES, (1, 1)),
        (BANANA, (4, 2)),
        (CAKE, (3, 2)),
        (BISCUITS, (2, 1)),
        (POPCORN, (2, 1)),
        (NUTS, (3, 2)),
        (PISTACHIO, (2, 2)),
    ];

    for (name, stats) in foods 
    {
        food_map.insert(name.to_string(), stats);
    }

    food_map
}


pub fn get_food_for_pet(pet_kind: &PetKind) -> Vec<String> 
{
    match pet_kind 
    {
        PetKind::Axalotl => vec![
            "Fish".to_string(),
            "Octopus".to_string(),
            "Shrimp".to_string(),
            "Popcorn".to_string(),
            "Biscuits".to_string(),
            "Berries".to_string(),
        ],
        PetKind::Cat => vec![
            "Milk".to_string(),
            "Steak".to_string(),
            "Chicken".to_string(),
            "Whiskas".to_string(),
            "Cake".to_string(),
            "Biscuits".to_string(),
        ],
        PetKind::Dog => vec![
            "Scooby snacks".to_string(),
            "Bone".to_string(),
            "Steak".to_string(),
            "Chicken".to_string(),
            "Biscuits".to_string(),
            "Cake".to_string(),
        ],
        PetKind::Rabbit => vec![
            "Carrot".to_string(),
            "Lettuce".to_string(),
            "Spinach".to_string(),
            "Cake".to_string(),
            "Biscuits".to_string(),
            "Berries".to_string(),
        ],
        PetKind::Parrot => vec![
            "Seeds".to_string(),
            "Berries".to_string(),
            "Raspberries".to_string(),
            "Banana".to_string(),
            "Cake".to_string(),
            "Popcorn".to_string(),
        ],
        PetKind::Monkey => vec![
            "Banana".to_string(),
            "Biscuits".to_string(),
            "Raspberries".to_string(),
            "Popcorn".to_string(),
            "Cake".to_string(),
            "Pistachio".to_string(),
        ],
        PetKind::Squirrel => vec![
            "Nuts".to_string(),
            "Pistachio".to_string(),
            "Berries".to_string(),
            "Raspberries".to_string(),
            "Seeds".to_string(),
            "Biscuits".to_string(),
        ],
    }
}
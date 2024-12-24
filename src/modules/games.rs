use std::collections::HashMap;
use crate::modules::pet::PetKind;

#[derive(Debug, Clone)]

pub struct GameEffect 
{
    pub happiness_increase: u8, 
    pub energy_cost: u8,       
    pub satiation_cost: u8,
}

//Games:

const FETCH: &str = "Fetch";
const CHASE_THE_LASER: &str = "Chase the Laser";
const CLIMBING_TREES: &str = "Climbing Trees";
const PLAYING_WITH_BALL: &str = "Playing with Ball";
const SWINGING: &str = "Swinging";
const FLYING_RACES: &str = "Flying Races";
const HIDE_AND_SEEK: &str = "Hide and Seek";
const SNIFFING_TRAILS: &str = "Sniffing Trails";
const SPLASHING_WATER: &str = "Splashing Water";


pub fn initialize_games() -> HashMap<String, GameEffect> 
{
    let mut games_map = HashMap::new();

    let games = vec![
        (
            FETCH,
            GameEffect 
            {
                happiness_increase: 4,
                energy_cost: 4,
                satiation_cost: 2,
            },
        ),
        (
            CHASE_THE_LASER,
            GameEffect 
            {
                happiness_increase: 5,
                energy_cost: 5,
                satiation_cost: 3,
            },
        ),
        (
            CLIMBING_TREES,
            GameEffect 
            {
                happiness_increase: 6,
                energy_cost: 6,
                satiation_cost: 4,
            },
        ),
        (
            PLAYING_WITH_BALL,
            GameEffect 
            {
                happiness_increase: 4,
                energy_cost: 4,
                satiation_cost: 2,
            },
        ),
        (
            SWINGING,
            GameEffect 
            {
                happiness_increase: 5,
                energy_cost: 5,
                satiation_cost: 3,
            },
        ),
        (
            FLYING_RACES,
            GameEffect 
            {
                happiness_increase: 6,
                energy_cost: 5,
                satiation_cost: 3,
            },
        ),
        (
            HIDE_AND_SEEK,
            GameEffect 
            {
                happiness_increase: 4,
                energy_cost: 4,
                satiation_cost: 2,
            },
        ),
        (
            SNIFFING_TRAILS,
            GameEffect 
            {
                happiness_increase: 3,
                energy_cost: 2,
                satiation_cost: 1,
            },
        ),
        (
            SPLASHING_WATER,
            GameEffect 
            {
                happiness_increase: 5,
                energy_cost: 4,
                satiation_cost: 2,
            },
        ),
    ];

    for (name, effect) in games 
    {
        games_map.insert(name.to_string(), effect);
    }

    games_map
}

pub fn get_games(pet_kind: &PetKind) -> Vec<String> 
{
    match pet_kind 
    {
        PetKind::Axalotl => vec![
            "Splashing Water".to_string(),
            "Hide and Seek".to_string(),
            "Playing with Ball".to_string(),
        ],
        PetKind::Cat => vec![
            "Chase the Laser".to_string(),
            "Climbing Trees".to_string(),
            "Playing with Ball".to_string(),
        ],
        PetKind::Dog => vec![
            "Fetch".to_string(),
            "Sniffing Trails".to_string(),
            "Playing with Ball".to_string(),
        ],
        PetKind::Rabbit => vec![
            "Splashing Water".to_string(),
            "Hide and Seek".to_string(),
            "Playing with Ball".to_string(),
        ],
        PetKind::Parrot => vec![
            "Flying Races".to_string(),
            "Hide and Seek".to_string(),
            "Playing with Ball".to_string(),
        ],
        PetKind::Monkey => vec![
            "Swinging".to_string(),
            "Climbing Trees".to_string(),
            "Hide and Seek".to_string(),
        ],
        PetKind::Squirrel => vec![
            "Climbing Trees".to_string(),
            "Hide and Seek".to_string(),
            "Playing with Ball".to_string(),
        ],
    }
}
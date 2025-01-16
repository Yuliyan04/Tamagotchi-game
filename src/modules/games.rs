use std::collections::HashMap;
use crate::modules::pet::PetKind;

#[derive(Debug)]

pub struct GameEffect 
{
    pub happiness_increase: u8, 
    pub energy_cost: u8,       
    pub satiation_cost: u8,
}

//Games:

const FETCH: &str = "Fetch";
const CHASE_THE_LASER: &str = "Chase the laser";
const CLIMBING_TREES: &str = "Climbing trees";
const PLAYING_WITH_BALL: &str = "Playing with ball";
const SWINGING: &str = "Swinging";
const FLYING_RACES: &str = "Flying races";
const HIDE_AND_SEEK: &str = "Hide and seek";
const SNIFFING_TRAILS: &str = "Sniffing trails";
const SPLASHING_WATER: &str = "Splashing water";


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
            SPLASHING_WATER.to_string(),
            HIDE_AND_SEEK.to_string(),
            PLAYING_WITH_BALL.to_string(),
        ],
        PetKind::Cat => vec![
            CHASE_THE_LASER.to_string(),
            CLIMBING_TREES.to_string(),
            PLAYING_WITH_BALL.to_string(),
        ],
        PetKind::Dog => vec![
            FETCH.to_string(),
            SNIFFING_TRAILS.to_string(),
            PLAYING_WITH_BALL.to_string(),
        ],
        PetKind::Rabbit => vec![
            SPLASHING_WATER.to_string(),
            HIDE_AND_SEEK.to_string(),
            PLAYING_WITH_BALL.to_string(),
        ],
        PetKind::Parrot => vec![
            FLYING_RACES.to_string(),
            HIDE_AND_SEEK.to_string(),
            PLAYING_WITH_BALL.to_string(),
        ],
        PetKind::Monkey => vec![
            SWINGING.to_string(),
            CLIMBING_TREES.to_string(),
            HIDE_AND_SEEK.to_string(),
        ],
        PetKind::Squirrel => vec![
            CLIMBING_TREES.to_string(),
            HIDE_AND_SEEK.to_string(),
            PLAYING_WITH_BALL.to_string(),
        ],
    }
}
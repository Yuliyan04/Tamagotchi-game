
use tamagotchi::modules::pet::{Pet, PetKind};
use tamagotchi::modules::food::initialize_food;
use tamagotchi::modules::games::initialize_games;


#[test]
fn test_feed_pet() 
{
    let mut pet = Pet::new("TestPet".to_string(), PetKind::Cat);
    let food_map = initialize_food();

    assert_eq!(pet.satiation(), 10);
    assert_eq!(pet.energy(), 10);

    pet.feed("Milk", &food_map);

    assert_eq!(pet.satiation(), 11); // Milk adds +1 satiation
    assert_eq!(pet.energy(), 11);    // Milk adds +1 energy
}

#[test]
fn test_play_with_pet() 
{
    let mut pet = Pet::new("TestPet".to_string(), PetKind::Dog);
    let games_map = initialize_games();

    assert_eq!(pet.happiness(), 10);
    assert_eq!(pet.energy(), 10);
    assert_eq!(pet.satiation(), 10);

    pet.play("Fetch", &games_map);

    assert_eq!(pet.happiness(), 14); // Fetch adds +4 happiness
    assert_eq!(pet.energy(), 6);     // Fetch costs -4 energy
    assert_eq!(pet.satiation(), 8);  // Fetch costs -2 satiation
}

#[test]
fn test_leave_pet_alone() 
{
    let mut pet = Pet::new("TestPet".to_string(), PetKind::Rabbit);

    assert_eq!(pet.satiation(), 10);
    assert_eq!(pet.energy(), 10);
    assert_eq!(pet.happiness(), 10);

    pet.leave_pet_alone(3);

    assert_eq!(pet.satiation(), 7); // Satiation -3
    assert_eq!(pet.energy(), 7);    // Energy -3
    assert_eq!(pet.happiness(), 7); // Happiness -3
}

#[test]
fn test_health_and_life_loss() 
{
    let mut pet = Pet::new("TestPet".to_string(), PetKind::Parrot);

    pet.set_energy(-10); 
    pet.set_satiation(-10);
    pet.check_satiation(); 
    pet.check_energy();    

    assert_eq!(pet.health(), 8); 

    pet.set_health(-10); 
    pet.is_health_zero();

    assert_eq!(pet.lives(), 0); 
    assert_eq!(pet.health(), 10); 
}
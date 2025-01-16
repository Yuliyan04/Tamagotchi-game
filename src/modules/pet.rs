use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::collections::HashMap;
use crate::modules::games::GameEffect;
use crate::modules::food::get_food_for_pet;

#[derive(Serialize, Deserialize, Clone)]
pub enum PetKind
{
    Axalotl,
    Cat,
    Dog,
    Monkey,
    Parrot,
    Rabbit,
    Squirrel,
}

//Converting enum to str and opposite
impl PetKind
{
    pub fn from_str(s: &str) -> Option<Self> 
    {
        match s.to_lowercase().as_str() 
        {
            "axalotl" => Some(PetKind::Axalotl),
            "cat"     => Some(PetKind::Cat),
            "dog"     => Some(PetKind::Dog),
            "monkey"  => Some(PetKind::Monkey),
            "parrot"  => Some(PetKind::Parrot),
            "rabbit"  => Some(PetKind::Rabbit),
            "squirrel"=> Some(PetKind::Squirrel),
            _         => None,
        }
    }

    pub fn to_string(&self) -> &str
    {
        match self
        {
            PetKind::Axalotl => "Axalotl",
            PetKind::Cat => "Cat",
            PetKind::Dog => "Dog",
            PetKind::Monkey => "Monkey",
            PetKind::Parrot => "Parrot",
            PetKind::Rabbit => "Rabbit",
            PetKind::Squirrel => "Squirrel",
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]

pub struct Pet
{
    name: String,
    kind: PetKind,
    lives: u8,

    health: u8,
    happiness: u8,
    energy: u8,
    satiation: u8,
}


//Implementing default Pet:
impl Pet
{
    pub fn new(name: String, kind: PetKind) -> Self
    {
        Self
        {
            name,
            kind,
            lives: 1,
            health: 10,
            happiness: 10,
            energy: 10,
            satiation: 10,
        }
    }
}


//Implementing getters and setters:
impl Pet
{
    //Getters:
    pub fn name(&self) -> &str 
    {
        &self.name
    }

    pub fn kind(&self) -> &PetKind 
    {
        &self.kind
    }

    pub fn lives(&self) -> u8 
    {
        self.lives
    }

    pub fn health(&self) -> u8 
    {
        self.health
    }

    pub fn satiation(&self) -> u8 
    {
        self.satiation
    }

    pub fn energy(&self) -> u8 
    {
        self.energy
    }

    pub fn happiness(&self) -> u8 
    {
        self.happiness
    }

    //Setters:

    //Unfortunately nothing has infinite health so the health caps at 10 :(
    pub fn set_health(&mut self, value: i8) 
    {
        let new_health = self.health as i8 + value;
    
        if new_health < 0 
        {
            self.health = 0;
        } 
        else 
        {
            self.health = if new_health > 10 {10} else {new_health} as u8;
        }
    }
    
    //If the pet's happines is between 11 and 15 it is overjoyed
    pub fn set_happiness(&mut self, value: i8) 
    {
        let new_happiness = self.happiness as i8 + value;

        if new_happiness < 0
        {
            self.happiness = 0;
        }
        else
        {
            self.happiness = if new_happiness <= 15 {new_happiness} else {15} as u8;
        }   
    }

    //Nobody can have infinite energy so we can't increase it infinitely but it caps at 15 if the energy is between 10 and 15
    //the pet is hyperenergetic and needs some activities.
    pub fn set_energy(&mut self, value: i8) 
    {
        let new_energy = self.energy as i8 + value;

        if new_energy < 0
        {
            self.energy = 0;
        }
        else
        {
            self.energy = if new_energy <= 15 {new_energy} else {15} as u8;
        }
    }

    //If pet is fed too much the satiation increases and leads to obesity
    pub fn set_satiation(&mut self, value: i8) 
    {
        let new_satiation = self.satiation as i8 + value;
        
        if new_satiation < 0
        {
            self.satiation = 0;
        }
        else
        {
            self.satiation = new_satiation as u8;
        }
    }
}

//Implementation for loading and saving object of type pet from and in file:
impl Pet
{
    pub fn save_pet_to_file(&self) -> io::Result<()>
    {
        let directory = "pets"; 

        if fs::metadata(directory).is_err() 
        {
            fs::create_dir_all(directory)?;
        }

        let filename = format!("{}/{}.json", directory, self.name); 
        let serialized = serde_json::to_string_pretty(&self)?;

        fs::write(&filename, serialized)?;
        println!("Pet saved to {}", filename);
        Ok(())
    }

    pub fn load_from_name(name: &str) -> io::Result<Self> 
    {
        let directory = "pets";
        let filename = format!("{}/{}.json", directory, name);

        let data = fs::read_to_string(&filename)?;
        let pet: Pet = serde_json::from_str(&data)?;

        if pet.lives == 0 
        {
            eprintln!(
                "Pet {} is dead (0 lives). Removing file {} and aborting load.",
                pet.name, filename
            );
            let _ = fs::remove_file(&filename); 
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Cannot load dead pet, file removed."
            ));
        }
        
        println!("Pet loaded from {}", filename);
        Ok(pet)
    }
}

//Check if the pet is dead:
pub fn is_dead(pet: &Pet) -> bool 
    {
        if pet.lives == 0 
        {
            println!("{} has 0 lives left and is now dead.", pet.name);
            
            let path = format!("pets/{}.json", pet.name);

            if let Err(e) = fs::remove_file(&path) 
            {
                println!("Warning: could not remove file '{}': {}", path, e);
            } 
            else 
            {
                println!("Pet file '{}' removed.", path);
            }
    
            return true; 
        }
        false
    }

//List of previous sessions
pub fn pets_list() -> io::Result<Vec<String>> 
{
    let directory = "pets";
    let mut pets = Vec::new();

    if fs::metadata(directory).is_err() 
    {
        println!("The pets directory does not exist.");
        return Ok(vec![]);
    }

    if let Ok(entries) = fs::read_dir(directory) 
    {
        for entry in entries 
        {
            let entry = entry?;
            if let Some(file_name) = entry.path().file_stem() 
            {
                if let Some(file_name) = file_name.to_str() 
                {
                    pets.push(file_name.to_string());
                }
            }
        }
    }

    if pets.is_empty() 
    {
        println!("No pets found in the directory.");
    }

    Ok(pets)
}

//Implementation for pet's pictures
impl PetKind 
{
    pub fn ascii_art(&self) -> &str 
    {
        match self 
        {
            PetKind::Axalotl => "
                  -,__
                     \\\\
   .      ,          : :
 -,_:____:_.'       :  |
-,_(• _ •)__,  __,-'   /
    '===='----'    ___/
      '--_____,.--',;
     -,' -,'     -'
",
            PetKind::Cat => "
               /\\\\_
              (` ..\\
         __,.-\" =__Y=
       .\"        )
 _    /   ,    \\/\\_
((____|    )_-\\ \\_-`
`-----'`-----` `--`
",
            PetKind::Dog => "
     _  
 __/_  `.  .-\"\"\"-.
 \\_,` | \\-'  /   )`-')
  \"\") `\"`    \\  ((`\"`
 ___Y  ,    .'7 /|
(_,___/...-` (_/_/
",
            PetKind::Monkey => "
     _____
   _/.-.-.\\_       
  /|( o o )|\\      
 | //  \"  \\\\ |     
/ / \\'---'/ \\ \\     
\\ \\_/`\"\"\"`\\_/ /    
 \\___________/
",
            PetKind::Parrot => "
  (((  
  `-`-.
  '( @ >
   _) (
  /    )
 /_,'  / 
   \\  / 
===m\"\"m===
",
            PetKind::Rabbit => "
         ,\\
         \\\\\\,_
          \\` ,\\
     __,.-\" =__)
   .\"        )
,_/   ,    \\/\\_
\\_|    )_-\\ \\_-`
   `-----` `--`
",
            PetKind::Squirrel => r#"
(\__/)  .~    ~. ))
 /O O  ./      .'
{O__,   \    {
  / .  . )    \
  |-| '-' \    }
 .(   _(   )_.'
'---.~_ _ _&
"#,
        }
    }
}

//Printing pet's stats
impl Pet 
{
    pub fn display_stats(&self) 
    {
        println!("{} the {}:", self.name, self.kind.to_string());
        println!("{}", self.kind.ascii_art());

        print!("Lives: ");
        for _ in 0..self.lives 
        {
            print!("♥ ");
        }
        println!();

        print!("Health: ");
        for _ in 0..self.health 
        {
            print!("█ ");
        }
        println!();

        print!("Happiness: ");
        for _ in 0..self.happiness 
        {
            print!("😊");
        }
        println!();

        print!("Energy: ");
        for _ in 0..self.energy 
        {
            print!("⚡");
        }
        println!();

        print!("Satiation: ");
        for _ in 0..self.satiation 
        {
            print!("🍎");
        }
        println!();

        println!();
    }
}

//Implementing sleeping
impl Pet
{
    pub fn sleep(&mut self)
    {   
        self.energy = 10;

        if self.satiation >= 3
        {
            self.satiation = self.satiation - 3;
        }
        else
        {
            self.satiation = 0;
        }

        if self.happiness < 5 
        {
            self.happiness = 5;
        } 
        else 
        {
            self.happiness = (self.happiness + 1).min(10);
        }

        println!("Energy restored to 10, satiation decreased by 3, and happiness adjusted.",);
    }
}

//Implementing pet interactions - feeding and playing
impl Pet
{
    pub fn feed(&mut self, food_name: &str, food_map: &HashMap<String, (u8, u8)>) -> bool
    {
        if let Some(&(satiation_gain, energy_gain)) = food_map.get(food_name) 
        {
            self.set_satiation(satiation_gain as i8);
            self.set_energy(energy_gain as i8);

            println!(
                "{} was fed {}. Satiation +{}, Energy +{}.",
                self.name,
                food_name,
                satiation_gain,
                energy_gain
            );
            return true;
        } 
        else 
        {
            println!("{} cannot eat {}. Food not recognized.", self.name, food_name);
            return false;
        }
    }
    
    pub fn play(&mut self, game_name: &str, games_map: &HashMap<String, GameEffect>) -> bool
    {
        if let Some(game) = games_map.get(game_name) 
        {
            if self.energy >= game.energy_cost && self.satiation >= game.satiation_cost 
            {
                println!(
                    "Happiness +{}, Energy -{}, Satiation -{}.",
                    game.happiness_increase,
                    game.energy_cost,
                    game.satiation_cost,
                );

                self.set_happiness(game.happiness_increase as i8);
                self.set_energy(-(game.energy_cost as i8));
                self.set_satiation(-(game.satiation_cost as i8));

                return true;
            } 
            else 
            {
                println!(
                    "{} does not have enough energy or satiation to play {}.",
                    self.name,
                    game_name
                );

                return false;
            }
        } 
        else 
        {
            println!("{} cannot play {}. Game not recognized.", self.name, game_name);
            return false;
        }
    }
}

//Implementing health checks
impl Pet
{
    pub fn check_satiation(&mut self) 
    {
        if self.satiation >= 11 && self.satiation <= 14 
        {
            println!(
                "Warning: {}'s satiation is high ({})! Consider playing games to reduce it.",
                self.name,
                self.satiation
            );
        } 
        else if self.satiation > 15 
        {
            println!(
                "Warning: {}'s satiation ({}) is dangerously high! Health reduced by 1.",
                self.name,
                self.satiation
            );
            self.set_health(-1);
        } 
        else if self.satiation == 0 
        {
            println!(
                "Warning: {}'s satiation is too low (0)! Health reduced by 1.",
                self.name
            );
            self.set_health(-1);
        }
    }

    pub fn check_energy(&mut self) -> bool
    {
        if self.energy <= 5 
        {
            if self.energy == 0 
            {
                println!(
                    "Warning: {}'s energy is too low (0)! Health reduced by 1.",
                    self.name
                );
                self.set_health(-1);
            }
            else
            {
                println!(
                    "Warning: {}'s energy is low! Consider doing something.",
                    self.name,
                );
            }

            return true;
        } 
        
        false
    }

    pub fn is_health_zero(&mut self) 
    {
        if self.health == 0 && self.lives > 0 
        {
            self.lives -= 1;
            self.health = 10;
            self.energy = 10;
            self.satiation = 10;
            self.happiness = 10;
            println!("{} lost a life! Remaining lives: {}", self.name, self.lives);
        }
    }
}

//We can not be with the pet for the whole time:
impl Pet
{
    pub fn leave_pet_alone(&mut self, time: u8) 
    {
        println!("You have left {} alone for {} hours.", self.name, time);

        self.set_satiation(-(time as i8));
        self.set_energy(-(time as i8));
        self.set_happiness(-(time as i8));

        println!("You left your pet alone. Satiation -{}, Energy -{}, Happiness -{}.", time, time, time);
    }

    pub fn leave_food_while_gone(&mut self, food_map: &HashMap<String, (u8, u8)>) 
    {
    
        println!("Foods available for {}:", self.kind().to_string());
        let valid_foods = get_food_for_pet(self.kind());
        for food in &valid_foods 
        {
            if let Some(&(sat, en)) = food_map.get(food) 
            {
                println!(" - {} (Satiation +{}, Energy +{})", food, sat, en);
            } 
            else 
            {
                println!(" - {}", food);
            }
        }
    
        println!("Enter the names of foods to leave (comma-separated):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
    
        let foods_left: Vec<String> = input.trim().split(',').map(|s| s.trim().to_string()).collect();
    
        for food_name in foods_left 
        {
            if let Some(&(sat, en)) = food_map.get(&food_name) 
            {
                self.set_satiation(sat as i8);
                self.set_energy(en as i8);
                println!(" - {} (Satiation +{}, Energy +{})", food_name, sat, en);
            } 
            else 
            {
                println!(" - {} (Unknown food, not applied)", food_name);
            }
        }
    }
}

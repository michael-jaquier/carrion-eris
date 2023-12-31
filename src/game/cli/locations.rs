use crate::{
    enemy::{Enemy, Mob, MobGrade},
    ui::cli::TerminalMessages,
};
use rand::{
    distributions::{Distribution, WeightedIndex},
    Rng,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::BufReader};

type Coordinates = (i32, i32, u8);

use super::{game_loop::GameState, states::State};
#[derive(Deserialize, Serialize, Debug)]
struct Description {
    x: i32,
    y: i32,
    z: u8,
    descriptor: String,
    tile: String,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Locations {
    locations: HashMap<Coordinates, Location>,
    current: Coordinates,
}

impl Locations {
    pub fn new() -> Locations {
        let locations = generate_map();
        Self {
            locations,
            current: (1, 1, 1),
        }
    }

    pub(crate) fn enemy(&self) -> Option<&Enemy> {
        self.current().enemy.as_ref()
    }

    pub(crate) fn tag_enemy(&mut self) {
        let coordinates = self.current;
        self.get_mut(&coordinates).unwrap().tagged = true;
    }

    pub(crate) fn enemy_killed(&mut self) {
        let coordinates = self.current;
        self.get_mut(&coordinates).unwrap().enemy = None;
    }

    fn get_mut(&mut self, coords: &Coordinates) -> Option<&mut Location> {
        self.locations.get_mut(coords)
    }

    fn current(&self) -> &Location {
        self.locations.get(&self.current).unwrap()
    }

    pub(crate) fn flee(state: &mut GameState) -> TerminalMessages {
        let current = state.location.current();
        let direction = *current.directions.first().unwrap();
        let direction: String = direction.into();
        state.location.tag_enemy();
        Locations::travel(state, &[&direction])
    }

    fn shift_direction(&mut self, direction: ValidDirections) -> (i32, i32, u8) {
        let current = self.current;
        let new_location = match direction {
            ValidDirections::East => (current.0 + 1, current.1, current.2),
            ValidDirections::West => (current.0 - 1, current.1, current.2),
            ValidDirections::North => (current.0, current.1 + 1, current.2),
            ValidDirections::South => (current.0, current.1 - 1, current.2),
            ValidDirections::Up => (current.0, current.1, current.2 + 1),
            ValidDirections::Down => (current.0, current.1, current.2 - 1),
        };
        self.current = new_location;
        new_location
    }

    pub(crate) fn travel(state: &mut GameState, args: &[&str]) -> TerminalMessages {
        let current = state.location.current();
        if args.is_empty() {
            return vec![
                (
                    "You can travel in the following directions: ".to_string(),
                    crossterm::style::Color::Red,
                ),
                (
                    format!("{directions:?}", directions = current.directions),
                    crossterm::style::Color::Red,
                ),
            ];
        }
        let direction = match ValidDirections::try_from(args[0]) {
            Ok(dir) => dir,
            Err(_) => {
                return vec![
                    (
                        "You can travel in the following directions: ".to_string(),
                        crossterm::style::Color::Red,
                    ),
                    (
                        format!("{directions:?}", directions = current.directions),
                        crossterm::style::Color::Red,
                    ),
                ];
            }
        };

        if !current.directions.contains(&direction) {
            return vec![
                (
                    "You can travel in the following directions: ".to_string(),
                    crossterm::style::Color::Red,
                ),
                (
                    format!("{directions:?}", directions = current.directions),
                    crossterm::style::Color::Red,
                ),
            ];
        }

        state.location.shift_direction(direction);

        let mut msg = Vec::new();
        msg.push((
            state.location.current().descriptor.clone(),
            crossterm::style::Color::Green,
        ));
        msg.push((
            format!(
                "You can travel in the following directions: {directions:?}",
                directions = state.location.current().directions
            ),
            crossterm::style::Color::Magenta,
        ));

        if state.location.current().tagged && state.location.current().enemy.is_some() {
            msg.push((
                "You have been here before...and the locals remember you".to_string(),
                crossterm::style::Color::Red,
            ));
            msg.push((format!("Enemy {enemy} prepares their attack", 
             enemy = state.location.current().enemy.as_ref().unwrap().kind), crossterm::style::Color::Red));

            state.state = State::Fighting;
            return msg;
        }
        
        if let Some(mob) = mob_generation_algo(state.location.current()) {
            if state.location.current().enemy.is_some() && state.character.is_some() {
                msg.push((
                    format!(
                        "You see a {enemy} in the distance",
                        enemy = state.location.current().enemy.as_ref().unwrap().kind
                    ),
                    crossterm::style::Color::Red,
                ));
                return msg
            }
            let enemy = mob.generate(state.character.as_ref().unwrap().level);
            let coordinates = state.location.current;
            state.location.get_mut(&coordinates).unwrap().enemy = Some(enemy);
            msg.push((
                format!("You see a {enemy} in the distance", enemy = mob),
                // Light Red
                crossterm::style::Color::Rgb {
                    r: 255,
                    g: 100,
                    b: 100,
                },
            ));
        }

        msg
    }

    pub(crate) fn go_to_origin(&mut self) {
        self.current = (1, 1, 1);
    }

    pub(crate) fn get_mut_enemy(&mut self) -> &mut Enemy {
        let coordinates = self.current;
        self.get_mut(&coordinates)
            .unwrap()
            .enemy
            .as_mut()
            .expect("No enemy found")
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Location {
    x: i32,
    y: i32,
    z: u8,
    directions: Vec<ValidDirections>,
    enemy: Option<Enemy>,
    tagged: bool,
    descriptor: String,
}

impl Default for Locations {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum ValidDirections {
    East,
    West,
    North,
    South,
    Up,
    Down,
}

impl TryFrom<&str> for ValidDirections {
    type Error = String;

    // east should match on east or "e"
    // west should match on west or "w"
    // north should match on north or "n"
    // south should match on south or "s"
    // up should match on up or "u"
    // down should match on down or "d"
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case("east") || value.eq_ignore_ascii_case("e") {
            return Ok(ValidDirections::East);
        }
        if value.eq_ignore_ascii_case("west") || value.eq_ignore_ascii_case("w") {
            return Ok(ValidDirections::West);
        }
        if value.eq_ignore_ascii_case("north") || value.eq_ignore_ascii_case("n") {
            return Ok(ValidDirections::North);
        }
        if value.eq_ignore_ascii_case("south") || value.eq_ignore_ascii_case("s") {
            return Ok(ValidDirections::South);
        }
        if value.eq_ignore_ascii_case("up") || value.eq_ignore_ascii_case("u") {
            return Ok(ValidDirections::Up);
        }
        if value.eq_ignore_ascii_case("down") || value.eq_ignore_ascii_case("d") {
            return Ok(ValidDirections::Down);
        }

        Err(format!("Invalid direction: {direction}", direction = value))
    }
}

impl From<ValidDirections> for String {
    fn from(value: ValidDirections) -> Self {
        format!("{:?}", value).to_lowercase()
    }
}

fn load_descriptions() -> Vec<Description> {
    let file = File::open("map.json").expect("Failed to open map.json");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to parse JSON")
}

/// The further we are from 0,0,0 the harder enemies should get
/// MobGrade increases as we go further from the center
fn mob_generation_algo(location: &Location) -> Option<Mob> {
    if location.enemy.is_some() {
        return Some(location.enemy.as_ref().unwrap().kind);
    }
    if !rand::thread_rng().gen_bool(0.85) {
        return None;
    }
    let mut rng = rand::thread_rng();

    let distance =
        ((location.x.pow(2) + location.y.pow(2) + location.z.pow(2) as i32) as f64).sqrt();
    let grade = match distance {
        d if d <= 5.0 => {
            let dist = WeightedIndex::new(vec![50, 40, 10]).unwrap();
            match dist.sample(&mut rng) {
                0 => MobGrade::Weak,
                1 => MobGrade::Normal,
                _ => MobGrade::Strong,
            }
        }
        d if d <= 10.0 => {
            let dist = WeightedIndex::new(vec![20, 30, 30, 20]).unwrap();
            match dist.sample(&mut rng) {
                0 => MobGrade::Weak,
                1 => MobGrade::Normal,
                2 => MobGrade::Strong,
                _ => MobGrade::Champion,
            }
        }
        d if d <= 15.0 => {
            let dist = WeightedIndex::new(vec![10, 20, 30, 30, 10]).unwrap();
            match dist.sample(&mut rng) {
                0 => MobGrade::Weak,
                1 => MobGrade::Normal,
                2 => MobGrade::Strong,
                3 => MobGrade::Champion,
                _ => MobGrade::Elite,
            }
        }
        _ => {
            let dist = WeightedIndex::new(vec![10, 20, 20, 30, 20]).unwrap();
            match dist.sample(&mut rng) {
                0 => MobGrade::Weak,
                1 => MobGrade::Normal,
                2 => MobGrade::Strong,
                3 => MobGrade::Champion,
                4 => MobGrade::Elite,
                _ => MobGrade::Legendary,
            }
        }
    };

    Some(grade.into())
}

fn valid_direction_map(cordinates: Coordinates, locations: &HashMap<(i32, i32, u8), Location>) -> Vec<ValidDirections> {
    let mut neighbors = Vec::new();
    let north = (cordinates.0, cordinates.1 + 1, cordinates.2);
    let south = (cordinates.0, cordinates.1 - 1, cordinates.2);
    let east = (cordinates.0 + 1, cordinates.1, cordinates.2);
    let west = (cordinates.0 - 1, cordinates.1, cordinates.2);
    let up = (cordinates.0, cordinates.1, cordinates.2 + 1);
    let down = (cordinates.0, cordinates.1, cordinates.2 - 1);

    if locations.get(&north).is_some() {
        neighbors.push(ValidDirections::North);
    }
    if locations.get(&south).is_some() {
        neighbors.push(ValidDirections::South);
    }
    if locations.get(&east).is_some() {
        neighbors.push(ValidDirections::East);
    }
    if locations.get(&west).is_some() {
        neighbors.push(ValidDirections::West);
    }
    if locations.get(&up).is_some() {
        neighbors.push(ValidDirections::Up);
    }
    if locations.get(&down).is_some() {
        neighbors.push(ValidDirections::Down);
    }

    neighbors
}


fn generate_map() -> HashMap<(i32, i32, u8), Location> {
    let descriptions = load_descriptions();
    let mut map = HashMap::new();

    for desc in &descriptions {
        let directions = Vec::new();

        let location = Location {
            x: desc.x,
            y: desc.y,
            z: desc.z,
            directions,
            enemy: None,
            tagged: false,
            descriptor: desc.descriptor.clone(),
        };

        let coordinates = (desc.x, desc.y, desc.z);
        map.insert(coordinates, location);
    }
    for desc in &descriptions {
        let coordinates = (desc.x, desc.y, desc.z);
        let new_directions = valid_direction_map(coordinates, &map);
        map.get_mut(&coordinates).unwrap().directions = new_directions;
    }

    map
}

#[cfg(test)]
mod test {
    #[test]
    fn test_serialization_of_description() {
        use super::Description;
        let desc = Description {
            x: 0,
            y: 0,
            z: 0,
            descriptor: "You are in a dark room".to_string(),
            tile: "Water".to_string(),
        };

        let json = serde_json::to_string(&desc).unwrap();
        assert_eq!(
            json,
            r#"{"x":0,"y":0,"z":0,"descriptor":"You are in a dark room","tile":"Water"}"#
        );
 

    }
    
    #[test]
    fn test_deserialize_descriptions() {
        use super::Description;
        let file = std::fs::File::open("map.json").unwrap();
        let reader = std::io::BufReader::new(file);
        let descriptions: Vec<Description> = serde_json::from_reader(reader).unwrap();
        assert!(!descriptions.is_empty());
    }

}
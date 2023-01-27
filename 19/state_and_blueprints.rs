use std::collections::HashMap;
use std::ops::Add;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl ResourceType {
    pub fn from_string(string: &String) -> Result<Self, String> {
        return match &string.to_string().to_lowercase()[..] {
            "ore" => Ok(ResourceType::Ore),
            "clay" => Ok(ResourceType::Clay),
            "obsidian" => Ok(ResourceType::Obsidian),
            "geode" => Ok(ResourceType::Geode),
            other => Err(other.to_string()),
        };
    }

    pub fn to_string(&self) -> String {
        return match self {
            ResourceType::Ore => "ore".to_string(),
            ResourceType::Clay => "clay".to_string(),
            ResourceType::Obsidian => "obsidian".to_string(),
            ResourceType::Geode => "geode".to_string(),
        };
    }

    pub fn iter() -> Iter<Self> {
        return vec![Self::Ore, Self::Clay, Self::Obsidian, Self::Geode].iter();
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct ResourceTally {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl ResourceTally {
    pub fn new() -> ResourceTally {
        return ResourceTally {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    pub fn from_tallies(ore: u32, clay: u32, obsidian: u32, geode: u32) -> ResourceTally {
        return ResourceTally {
            ore: ore,
            clay: clay,
            obsidian: obsidian,
            geode: geode,
        }
    }

    pub fn from_hashmap(hashmap: HashMap<ResourceType, u32>) -> ResourceTally {
        return ResourceTally {
            ore: *hashmap.get(&ResourceType::Ore).unwrap(),
            clay: *hashmap.get(&ResourceType::Clay).unwrap(),
            obsidian: *hashmap.get(&ResourceType::Obsidian).unwrap(),
            geode: *hashmap.get(&ResourceType::Geode).unwrap(),
        }
    }

    pub fn as_hashmap(&self) -> HashMap<ResourceType, u32> {
        return HashMap::from([
            (ResourceType::Ore, self.ore), 
            (ResourceType::Clay, self.clay), 
            (ResourceType::Obsidian, self.obsidian), 
            (ResourceType::Geode, self.geode),
        ])
    }

    pub fn copy_tally(&self) -> Self {
        return Self::from_hashmap(self.as_hashmap);
    }

    pub fn update_resource_from_string(&mut self, resource_type_str: &String, amount: u32) {
        if let Ok(resource_type) = ResourceType::from_string(resource_type_str) {
            self.update_resource(resource_type, amount);
        }
        else {
            panic!("{} is not a valid resource_type!", resource_type_str);
        }
    }

    pub fn update_resource(&mut self, resource_type: ResourceType, amount: u32) {
        match resource_type {
            ResourceType::Ore => self.ore = amount,
            ResourceType::Clay => self.clay = amount,
            ResourceType::Obsidian => self.obsidian = amount,
            ResourceType::Geode => self.geode = amount,
        };
    }

    pub fn new_tally_with_added_resource(&self, resource_type: Self, amount: u32) -> Self {
        let mut current_resource_as_hashmap = self.as_hashmap();
        current_resource_as_hashmap[resource_type] += amount;
        return Self::from_hashmap(current_resource_as_hashmap);
    }
}

impl Add for ResourceTally { 
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return Self::from_tallies(
            self.ore + other.ore, 
            self.clay + other.clay, 
            self.obsidian + other.obsidian,
            self.geode + other.geode,
        )
    }
}

impl Sub for ResourceTally { 
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        return Self::from_tallies(
            self.ore - other.ore, 
            self.clay - other.clay, 
            self.obsidian - other.obsidian,
            self.geode - other.geode,
        )
    }
}

#[derive(Debug)]
pub struct State {
    time_left: u32,
    resources: ResourceTally,
    robots: ResourceTally,
}

impl State {
    pub fn new() -> State {
        let mut robot_tally = ResourceTally::new();
        robot_tally.ore = 1;
        return State {
            time_left: 24, 
            resources: ResourceTally::new(), 
            robots: robot_tally,
        }
    }

    pub fn new_state(&self, robots_to_produce: ResourceTally, cost: ResourceTally) -> State {
        self.time_left -= 1;
        self.resources = self.resources + self.robots - cost;
        self.robots = self.robots + robots_to_produce 
        self.create_robots();
    }
}

#[derive(Debug)]
pub struct Blueprint {
    index: u32,
    costs: HashMap<ResourceType, ResourceTally>,
    best_value: u32,
}

impl Blueprint {
    pub fn new(index: u32, costs: HashMap<ResourceType, ResourceTally>) -> Blueprint {
        return Blueprint {index: index, costs: costs, best_value: 0};
    }
}

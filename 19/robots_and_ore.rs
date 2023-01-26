use std::collections::HashMap;
use std::ops::Add;

enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

struct ResourceTally {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl ResourceTally {
    fn new() -> ResourceTally {
        return ResourceTally {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn new(ore, clay, obsidian, geode) -> ResourceTally {
        return ResourceTally {
            ore: ore,
            clay: clay,
            obsidian: obsidian,
            geode: geode,
        }
    }

    fn as_hashmap(&self) -> HashMap<ResourceType, u32> {
        return HashMap::from([
            (ResourceType::Ore, self.ore), 
            (ResourceType::Clay, self.clay), 
            (ResourceType::Obsidian, self.obsidian), 
            (ResourceType::Geode, self.geode),
        ])
    } 
}

impl Add for ResourceTally { 
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return Self::new(
            self.ore + other.ore, 
            self.clay + other.clay, 
            self.obsidian + other.obsidian,
            self.geode + other.geode,
        )
    }
}

struct State {
    time_left: u32,
    resources: ResourceTally,
    robots: ResourceTally,
    robots_in_production: ResourceTally,
}

impl State {
    fn new() -> {
        let robot_tally = ResourceTally::new();
        robot_tally.ore = 1;
        return State {
            time_left: 24, 
            resources: ResourceTally::new(), 
            robots: robot_tally,
        }
    }

    fn create_robots(&mut self) {
        self.robots = self.robots + self.robots_in_production;
        self.robots_in_production = ResourceTally::new();
    }

    fn tock(&mut self) -> {
        self.time_left -= 1;
        self.resources = self.resources + self.robots;
    }
}

struct Blueprint {
    index: u32,
    costs: HashMap<ResourceType, ResourceTally>,
    state: State,
}

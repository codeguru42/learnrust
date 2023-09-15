use crate::prelude::*;
use super::MapArchitect;

pub struct DrunkardWalkArchitect {}

impl MapArchitect for DrunkardWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        mb
    }
}

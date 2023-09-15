use crate::prelude::*;
use super::MapArchitect;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
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

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0) && map.tiles[map_idx(x+ix, y+iy)] ==  TileType::Wall {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }
}

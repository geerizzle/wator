use config::Config;
use entity::Entity;
use rand::seq::IndexedRandom;

use crate::ui::AppState;

pub mod config;
pub mod entity;

const NEIGHBORS: [(i16, i16); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

pub struct Wator {
    state: AppState,
    width: u16,
    height: u16,
    config: Config,
    num_fish: u32,
    is_looping: bool,
    num_sharks: u32,
    world: Vec<Entity>,
}

impl Default for Wator {
    fn default() -> Self {
        Self {
            config: Config::default(),
            width: 0,
            is_looping: false,
            height: 0,
            state: AppState::InWaTor,
            world: Vec::new(),
            num_fish: 0,
            num_sharks: 0,
        }
    }
}

impl Wator {
    pub fn new(width: u16, height: u16) -> Self {
        let world: Vec<Entity> = Vec::with_capacity((width * height) as usize);
        Self {
            width,
            height,
            world,
            ..Default::default()
        }
    }

    pub fn toggle_loop(&mut self) {
        self.is_looping = !self.is_looping;
    }

    pub fn is_looping(&self) -> bool {
        self.is_looping
    }

    pub fn initialize(&mut self) {
        for _ in 0..(self.width * self.height) {
            let entity = if rand::random::<f32>() < 0.1 {
                Entity::Shark {
                    age: 0,
                    energy: self.config.shark_energy,
                }
            } else if rand::random::<f32>() < 0.3 {
                Entity::Fish { age: 0 }
            } else {
                Entity::Empty
            };
            self.world.push(entity);
        }

        self.num_fish = self.world.iter().filter(|e| e.is_fish()).count() as u32;
        self.num_sharks = self.world.iter().filter(|e| e.is_shark()).count() as u32;
    }

    pub fn simulate(&mut self) {
        let max_size = (self.width * self.height) as usize;
        for i in 0..max_size {
            if self.world[i].is_empty() {
                continue;
            }
            let x = (i as u16 % self.width) as i16;
            let y = (i as u16 / self.width) as i16;
            self.world[i].deprive();
            if self.world[i].is_dead(self.config.clone()) {
                self.world[i] = Entity::Empty;
                continue;
            }
            let empty_cells: Vec<i16> = NEIGHBORS
                .iter()
                .map(|(dx, dy)| x + dx + self.width as i16 * (y + dy))
                .filter(|nid| *nid >= 0 && *nid < max_size as i16)
                .filter(|nid| self.world[*nid as usize].is_empty())
                .collect();

            let fish_cells: Vec<i16> = NEIGHBORS
                .iter()
                .map(|(dx, dy)| x + dx + self.width as i16 * (y + dy))
                .filter(|nid| *nid >= 0 && *nid < max_size as i16)
                .filter(|nid| self.world[*nid as usize].is_fish())
                .collect();

            if self.world[i].is_fish() {
                if let Some(move_to) = empty_cells.choose(&mut rand::rng()) {
                    let move_to = *move_to as usize;
                    if self.world[i].can_reproduce() {
                        self.world[move_to] = self.world[i].clone();
                        self.world[i] = self.world[i].spawn_new(self.config.clone());
                    } else {
                        self.world[move_to] = self.world[i].clone();
                        self.world[i] = Entity::Empty;
                    }
                }
            }

            if self.world[i].is_shark() {
                if let Some(move_to) = fish_cells.choose(&mut rand::rng()) {
                    let move_to = *move_to as usize;
                    self.world[i].gain_energy();
                    self.world[move_to] = self.world[i].clone();
                    if self.world[i].can_reproduce() {
                        self.world[i] = self.world[i].spawn_new(self.config.clone());
                    } else {
                        self.world[i] = Entity::Empty;
                    }
                } else if let Some(move_to) = empty_cells.choose(&mut rand::rng()) {
                    let move_to = *move_to as usize;
                    self.world[move_to] = self.world[i].clone();
                    if self.world[i].can_reproduce() {
                        self.world[i] = self.world[i].spawn_new(self.config.clone());
                    } else {
                        self.world[i] = Entity::Empty;
                    }
                }
            }
        }
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn chronon(&self) -> u64 {
        self.config.chronon
    }

    pub fn num_fish_in_area(&self, max_w: u16, max_h: u16) -> u32 {
        self.world
            .iter()
            .take((max_w * max_h) as usize)
            .filter(|e| e.is_fish())
            .count() as u32
    }

    pub fn num_sharks_in_area(&self, max_w: u16, max_h: u16) -> u32 {
        self.world
            .iter()
            .take((max_w * max_h) as usize)
            .filter(|e| e.is_shark())
            .count() as u32
    }

    pub fn state_mut(&mut self) -> &mut AppState {
        &mut self.state
    }

    pub fn world(&self) -> &[Entity] {
        &self.world
    }
}

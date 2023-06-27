#![allow(unused)]
use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::collections::hash_map::IterMut;
use std::f32::consts::PI;

use macroquad::{prelude::*, color}; 
use parry2d::shape::{Cuboid, Ball};
use ::rand::{Rng, thread_rng};
use rapier2d::prelude::*;
use crate::kinetic::{Detection, contact_circles};
use crate::util::*;
use crate::consts::*;
use crate::timer::*;
use crate::neuro::*;
use crate::world::*;

pub struct StaticElement {
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
    pub color: color::Color,
    pub shape: Cuboid,
    pub physics_handle: Option<RigidBodyHandle>,
}

impl StaticElement {
    pub fn new(position: Vec2, w: f32, h: f32, color: color::Color) -> Self {
        Self { 
            pos: position, 
            width: w, 
            height: h, 
            color: color, 
            shape: Cuboid { half_extents: vector![w, h] }, 
            physics_handle: None 
        }
    }

    pub fn draw(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let w = self.width;
        let h = self.height;
        draw_rectangle(x0, y0, w, h, self.color);
    }

    pub fn update2(&mut self, physics: &mut World) {
        match self.physics_handle {
            Some(handle) => {
                let physics_data = physics.get_physics_data(handle);
                self.pos = physics_data.position;
            },
            None => {},
        }

    }
}

pub struct Molecule {
    pub pos: Vec2,
    pub rot: f32,
    pub vel: f32,
    pub ang_vel: f32,
    pub size: f32,
    pub color: color::Color,
    pub shape: Ball,
    pub field_range: i32,
    pub physics_handle: Option<RigidBodyHandle>,
}

impl Molecule {
    pub fn new() -> Self {
        let s = (rand::gen_range(MOLECULE_SIZE_MIN, MOLECULE_SIZE_MAX) as f32).round();
        let motor = thread_rng().gen_bool(1.0);
        Self {
            pos: random_position(WORLD_W, WORLD_H),
            //pos: Vec2::new(0.0, 0.0),
            rot: random_rotation(),
            vel: rand::gen_range(0.0, 1.0)*MOLECULE_SPEED,
            ang_vel: 0.0,
            size: s,
            color: random_color(),
            shape: Ball { radius: s },
            field_range: rand::gen_range(16, 96),
            physics_handle: None,
        }
    }
    pub fn draw(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        draw_circle(x0, y0, self.size, self.color);
        draw_circle_lines(x0, y0, self.field_range as f32, 0.5, GRAY);
    }

    pub fn update2(&mut self, physics: &mut World) {
        match self.physics_handle {
            Some(handle) => {
                let physics_data = physics.get_physics_data(handle);
                self.pos = physics_data.position;
                self.rot = physics_data.rotation;
                self.field_range = physics_data.field_radius as i32;
            },
            None => {},
        }

    }
}



pub struct MoleculesBox {
    pub molecules: HashMap<u64, Molecule>
}

impl MoleculesBox {
    pub fn new() -> Self {
        Self {
            molecules: HashMap::new(),
        }
    }

    pub fn add_many_molecules(&mut self, molecules_num: usize, physics_world: &mut World) {
        for _ in 0..molecules_num {
            let molecule = Molecule::new();
            _ = self.add_molecule(molecule, physics_world);
        }
    }

    pub fn add_molecule(&mut self, mut molecule: Molecule, physics_world: &mut World) -> u64 {
        let key: u64 = thread_rng().gen::<u64>();
        let handle = physics_world.add_circle_body(&molecule.pos, molecule.size, molecule.field_range as f32);
        molecule.physics_handle = Some(handle);
        self.molecules.insert(key, molecule);
        return key;
    }

    pub fn get(&self, id: u64) -> Option<&Molecule> {
        return self.molecules.get(&id);
    }

    pub fn remove(&mut self, id: u64) {
        self.molecules.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, Molecule> {
        return self.molecules.iter();
    }
    
    pub fn get_iter_mut(&mut self) -> IterMut<u64, Molecule> {
        return self.molecules.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.molecules.len();
    }
}

pub struct StaticElementBox {
    pub elements: HashMap<u64, StaticElement>
}

impl StaticElementBox {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn add_element(&mut self, mut static_element: StaticElement, physics_world: &mut World) -> u64 {
        let key: u64 = thread_rng().gen::<u64>();
        let handle = physics_world.add_static_body(&static_element.pos, static_element.width, static_element.height);
        static_element.physics_handle = Some(handle);
        self.elements.insert(key, static_element);
        return key;
    }

    pub fn get(&self, id: u64) -> Option<&StaticElement> {
        return self.elements.get(&id);
    }

    pub fn remove(&mut self, id: u64) {
        self.elements.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, StaticElement> {
        return self.elements.iter();
    }
    
    pub fn get_iter_mut(&mut self) -> IterMut<u64, StaticElement> {
        return self.elements.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.elements.len();
    }
}

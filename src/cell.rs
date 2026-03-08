use crate::{Game, utils::UpdateList};
use js_sys::Math;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Element {
    Air,
    Sand,
    Water,
    Steam,
    Stone,
    Fire,
}

impl Element {
    pub fn colors(&self) -> Vec<[u8; 3]> {
        match self {
            Element::Air => vec![[255, 255, 255]],
            Element::Sand => vec![[255, 255, 0]],
            Element::Water => vec![[0, 0, 255]],
            Element::Steam => vec![[0, 0, 255]],
            Element::Stone => vec![[100, 100, 100]],
            Element::Fire => vec![[255, 0, 0]],
        }
    }

    pub fn transparency(&self) -> u8 {
        match self {
            Element::Air => 0,
            Element::Sand => 255,
            Element::Water => 255,
            Element::Steam => 120,
            Element::Stone => 255,
            Element::Fire => 255,
        }
    }

    pub fn density(&self) -> f32 {
        // Density of the element in kg/m^3
        match self {
            Element::Air => 1.225,
            Element::Sand => 1400.0,
            Element::Water => 1000.0,
            Element::Steam => 0.6,
            Element::Stone => 2500.0,
            Element::Fire => 0.0,
        }
    }

    pub fn tags(&self) -> Vec<String> {
        match self {
            Element::Air => vec!["gas".to_string(), "empty".to_string()],
            Element::Sand => vec!["solid".to_string(), "powder".to_string()],
            Element::Water => vec!["liquid".to_string()],
            Element::Steam => vec!["gas".to_string()],
            Element::Stone => vec!["solid".to_string()],
            Element::Fire => vec![],
        }
    }

    pub fn thermal_conductivity(&self) -> f32 {
        match self {
            Element::Air => 0.026,
            Element::Sand => 0.2,
            Element::Water => 0.6,
            Element::Steam => 0.026,
            Element::Stone => 2.0,
            Element::Fire => 0.026,
        }
    }

    pub fn heat_capacity(&self) -> f32 {
        match self {
            Element::Air => 1012.0,
            Element::Sand => 800.0,
            Element::Water => 4180.0,
            Element::Steam => 1860.0,
            Element::Stone => 800.0,
            Element::Fire => 1012.0,
        }
    }

    pub fn behavior(&mut self) -> fn([usize; 2], &mut Game) {
        match self {
            Element::Air => todo!(),
            Element::Sand => todo!(),
            Element::Water => todo!(),
            Element::Steam => todo!(),
            Element::Stone => todo!(),
            Element::Fire => todo!(),
        }
    }

    pub fn standard_temperature(&self) -> f32 {
        match self {
            Element::Air => 293.0,
            Element::Sand => 293.0,
            Element::Water => 293.0,
            Element::Steam => 383.0,
            Element::Stone => 293.0,
            Element::Fire => 1000.0,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, PartialEq, Debug)]
pub struct Cell {
    element: Element,
    temperature: f32,
    random: u8,
    velocity: [f32; 2],
}

impl Default for Cell {
    fn default() -> Self {
        let random = (Math::random() * 255.0) as u8;

        Self {
            element: Element::Air,
            temperature: Element::Air.standard_temperature(),
            random,
            velocity: [0.0, 0.0],
        }
    }
}

impl Cell {
    pub fn new(element: Element) -> Self {
        Self {
            element,
            temperature: element.standard_temperature(),
            random: (Math::random() * 255.0) as u8,
            velocity: [0.0, 0.0],
        }
    }
}

#[wasm_bindgen]
impl Cell {
    pub fn element(&self) -> Element {
        self.element
    }

    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    pub fn random(&self) -> u8 {
        self.random
    }

    pub fn velocity_x(&self) -> f32 {
        self.velocity[0]
    }

    pub fn velocity_y(&self) -> f32 {
        self.velocity[1]
    }

    pub fn set_temperature(&mut self, value: f32) {
        self.temperature = value;
    }
}

impl Cell {
    pub fn velocity(&self) -> [f32; 2] {
        self.velocity
    }
}

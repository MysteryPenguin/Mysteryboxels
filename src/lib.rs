mod behavior;
mod cell;
mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, PointerEvent};

use crate::{
    cell::{Cell, Element},
    utils::{CellList, UpdateList},
};

#[wasm_bindgen]
pub struct Game {
    cells: CellList,
    active_cells: UpdateList,
    active_temp_cells: UpdateList,
    width: usize,
    settings: GameSettings,
    height: usize,
    canvas: HtmlCanvasElement,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, canvas: HtmlCanvasElement) -> Self {
        Self {
            cells: CellList::new(width, height),
            active_cells: UpdateList::new(),
            active_temp_cells: UpdateList::new(),
            width,
            height,
            settings: GameSettings::default(),
            canvas,
        }
    }

    pub fn tick_elements(&mut self) {
        let active = std::mem::take(&mut self.active_cells);

        for [x, y] in active.into_iter() {
            let Some(cell) = self.cells.get(x, y) else {
                continue;
            };

            cell.element().behavior()([x, y], self);
        }
    }

    pub fn tick_temperature(&mut self) {
        let active = std::mem::take(&mut self.active_temp_cells);

        for [x, y] in active.into_iter() {
            let [Some(cell), neighbors @ ..] = self.cells.get_mut_neighbors_and_self(x, y) else {
                continue;
            };

            for (i, neighbor) in neighbors.into_iter().enumerate() {
                let [nx, ny] = [i % self.width, i / self.width];
                let Some(neighbor) = neighbor else {
                    continue;
                };

                let k = (neighbor.element().thermal_conductivity()
                    + cell.element().thermal_conductivity())
                    / 2.0;

                let delta_temp = cell.temperature() - neighbor.temperature();

                cell.set_temperature(
                    cell.temperature() - (k * delta_temp) / cell.element().heat_capacity(),
                );
                neighbor.set_temperature(
                    neighbor.temperature() - (k * delta_temp) / neighbor.element().heat_capacity(),
                );

                self.active_temp_cells.add(nx, ny);
            }
            self.active_temp_cells.add(x, y);
        }
    }

    pub fn create_pixels(&mut self, event: PointerEvent) {
        let [x, y] = self.get_pointer_pos(event);

        self.settings.selected_cell = self.cells.get(x, y).cloned();
        if self.settings.draw_mode == DrawMode::Select {
            return;
        }

        let brush_size = self.settings.brush_size;
        let half = brush_size / 2;

        for oy in -(half as i8)..=half as i8 {
            for ox in -(half as i8)..=half as i8 {
                let [px, py] = [
                    (x as isize + ox as isize) as usize,
                    (y as isize + oy as isize) as usize,
                ];
                let Some(cell) = self.cells.get_mut(px, py) else {
                    continue;
                };

                if let Some(element) = self.settings.selected_element
                    && cell.element().tags().contains(&String::from("empty"))
                {
                    self.cells.set(px, py, Cell::new(element));
                } else if self.settings.is_heating {
                    cell.set_temperature(cell.temperature() + self.settings.heat as f32);
                    self.active_temp_cells.add(px, py);
                }
            }
        }
    }
}

impl Game {
    pub fn get_pointer_pos(&self, event: PointerEvent) -> [usize; 2] {
        let rect = self.canvas.get_bounding_client_rect();

        let client_x = event.client_x() as f64;
        let client_y = event.client_y() as f64;

        let x = ((client_x - rect.left()) * (self.width as f64 / rect.width())).floor() as usize;
        let y = ((client_y - rect.top()) * (self.height as f64 / rect.height())).floor() as usize;

        [x, y]
    }
}

#[wasm_bindgen]
pub struct GameSettings {
    tick_rate: u16,
    gravity: f32,
    brush_size: u8,
    selected_element: Option<Element>,
    selected_cell: Option<Cell>,
    heat: i8,
    is_heating: bool,
    draw_mode: DrawMode,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            tick_rate: 20,
            gravity: 9.81,
            brush_size: 1,
            selected_element: Some(Element::Sand),
            selected_cell: None,
            heat: 0,
            is_heating: false,
            draw_mode: DrawMode::Draw,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Run,
    Paused,
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub enum DrawMode {
    Select,
    Draw,
}

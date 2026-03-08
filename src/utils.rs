use std::vec::IntoIter;

use wasm_bindgen::prelude::*;

use crate::cell::Cell;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
#[derive(Clone, PartialEq, Debug)]
pub struct CellList {
    width: usize,
    list: Vec<Cell>,
}

impl CellList {
    pub fn new(width: usize, height: usize) -> Self {
        let mut list = Vec::with_capacity(width * height);
        list.fill(Cell::default());

        Self { width, list }
    }

    pub fn set(&mut self, x: usize, y: usize, value: Cell) {
        self.list[y * self.width + x] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.list.get(y * self.width + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.list.get_mut(y * self.width + x)
    }

    pub unsafe fn get_mut_unchecked(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if self.list.len() <= y * self.width + x {
            return unsafe { Some(self.list.get_unchecked_mut(y * self.width + x)) };
        }

        None
    }

    pub fn swap(&mut self, target1: [usize; 2], target2: [usize; 2]) {
        self.list.swap(
            target2[1] * self.width + target2[0],
            target1[1] * self.width + target1[0],
        );
    }

    pub fn get_neighbors_and_self(&mut self, x: usize, y: usize) -> [Option<&Cell>; 5] {
        [
            self.get(x, y),
            self.get(x - 1, y),
            self.get(x + 1, y),
            self.get(x, y - 1),
            self.get(x, y + 1),
        ]
    }

    fn idx(&self, x: usize, y: usize) -> Option<usize> {
        if self.list.len() <= y * self.width + x {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    pub fn get_mut_neighbors_and_self(&mut self, x: usize, y: usize) -> [Option<&mut Cell>; 5] {
        let coords = [
            self.idx(x, y),
            x.checked_sub(1).and_then(|nx| self.idx(nx, y)),
            self.idx(x + 1, y),
            y.checked_sub(1).and_then(|ny| self.idx(x, ny)),
            self.idx(x, y + 1),
        ];

        // collect valid indices
        let mut indexed: Vec<(usize, usize)> = coords
            .iter()
            .enumerate()
            .filter_map(|(i, &idx)| idx.map(|v| (i, v)))
            .collect();

        indexed.sort_by_key(|&(_, idx)| idx);

        let mut result: [Option<&mut Cell>; 5] = [None, None, None, None, None];

        let mut slice: &mut [Cell] = &mut self.list;

        for (original_pos, idx) in indexed {
            let (_, right) = slice.split_at_mut(idx);
            let (cell, rest) = right.split_first_mut().unwrap();

            result[original_pos] = Some(cell);
            slice = rest;
        }

        result
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct UpdateList(Vec<[usize; 2]>);

impl UpdateList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, x: usize, y: usize) {
        if !self.0.contains(&[x, y]) {
            self.0.push([x, y]);
        }
    }

    pub fn add_neighbors(&mut self, x: usize, y: usize, add_self: bool) {
        if add_self {
            self.add(x, y);
        }

        self.add(x - 1, y);
        self.add(x + 1, y);
        self.add(x, y - 1);
        self.add(x, y + 1);
    }
}

impl UpdateList {
    pub fn into_iter(self) -> IntoIter<[usize; 2]> {
        self.0.into_iter()
    }
}

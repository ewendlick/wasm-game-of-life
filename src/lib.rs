mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// TODO: confirm that this is the proper location for this code
extern crate web_sys;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

extern crate js_sys;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        // TODO: handle content outside of range
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                // log!(
                //     "cell[{}, {}] is initially {:?} and has {} live neighbors",
                //      row,
                //      col,
                //      cell,
                //      live_neighbors
                // );

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                // log!("    it becomes {:?}", next_cell);

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    // Set width of universe
    // Resets all cells to the dead state
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    // Set height of universe
    // Resets all cells to the dead state
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn insert_glider(&mut self, row: u32, column: u32) {
        let mut idx = self.get_index(row, column);
        // TODO: Consider adding a direction, or randomizing direction
        //
        // TODO: do not draw outside of range
        self.cells[idx] = Cell::Alive;
        idx = self.get_index(row + 1, column + 1);
        self.cells[idx] = Cell::Alive;
        idx = self.get_index(row + 1, column + 2);
        self.cells[idx] = Cell::Alive;
        idx = self.get_index(row + 2, column);
        self.cells[idx] = Cell::Alive;
        idx = self.get_index(row + 2, column + 1);
        self.cells[idx] = Cell::Alive;
    }

    pub fn insert_pulsar(&mut self, row: u32, column: u32) {
        // TODO: See if there is a more clever way to do this
        //
        // TODO: Check and see if overflow is an issue when drawing this
        let mut idx;
        for offset_row in 0..=12 {
            if offset_row == 0 || offset_row == 5 || offset_row == 7 || offset_row == 12 {
                // TODO: tighten up to single lines?
                idx = self.get_index(row + offset_row, column + 2);
                self.cells[idx] = Cell::Alive;
                idx = self.get_index(row + offset_row, column + 3);
                self.cells[idx] = Cell::Alive;
                idx = self.get_index(row + offset_row, column + 4);
                self.cells[idx] = Cell::Alive;
                idx = self.get_index(row + offset_row, column + 8);
                self.cells[idx] = Cell::Alive;
                idx = self.get_index(row + offset_row, column + 9);
                self.cells[idx] = Cell::Alive;
                idx = self.get_index(row + offset_row, column + 10);
                self.cells[idx] = Cell::Alive;
            } else if offset_row == 2 || offset_row == 3 || offset_row == 4 ||
                offset_row == 8 || offset_row == 9 || offset_row == 10 {
                idx = self.get_index(row + offset_row, column + 0);
                self.cells[idx] = Cell::Alive;
                idx = self.get_index(row + offset_row, column + 5);
                self.cells[idx] = Cell::Alive;
                idx = self.get_index(row + offset_row, column + 7);
                self.cells[idx] = Cell::Alive;
                idx = self.get_index(row + offset_row, column + 12);
                self.cells[idx] = Cell::Alive;
            }
        }
    }

    pub fn clear_all(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                self.cells[idx] = Cell::Dead;
            }
        }
    }

    pub fn generate_random(&mut self) {
        self.cells = (0..self.width * self.height)
            .map(|_i| {
                if js_sys::Math::random() < 0.5  {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        // for row in 0..self.height {
        //     for col in 0..self.width {
        //         let idx = self.get_index(row, col);
        //         self.cells[idx] = // random value
        //     }
        // }
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

// Created without #[wasm_bindgen] to keep this out of Javascript
impl Universe {
    // Get the dead and alive values of the entire universe
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    // Set cells to be alive in a universe by passing the row and column of each cell as an array
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

}


impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }
//
// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }

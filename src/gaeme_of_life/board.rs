use rand::Rng;
use wasm_bindgen::prelude::*;

use crate::gaeme_of_life::cell::CellSet;

#[wasm_bindgen]
pub struct AABB {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[wasm_bindgen]
impl AABB {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[wasm_bindgen]
pub struct Board {
    cells: [CellSet; 2],
    pub width: i32,
    pub height: i32,
    pub color_width: i32,
    pub tick_count: u32,
}

#[wasm_bindgen]
impl Board {
    pub fn new(width: u32, height: u32, default_color: &[u8]) -> Self {
        let mut rng = rand::thread_rng();
        let mut cells = CellSet::new((width * height * (default_color.len()) as u32) as usize);

        for i in 0..cells.cells.len() / default_color.len() {
            cells.cells[(i + 1) * default_color.len() - 1] = 255;
        }
        let cells_odd = cells.clone();

        for i in 0..cells_odd.cells.len() / default_color.len() {
            let r: f32 = rng.gen();
            cells.cells[i * default_color.len()] = if r > 0.9 { 255 } else { 0 };
        }

        Self {
            cells: [cells, cells_odd],
            width: width as i32,
            height: height as i32,
            color_width: default_color.len() as i32,
            tick_count: 0,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> bool {
        let cells = &self.cells[((self.tick_count) % 2) as usize];

        let ret = cells.bit_get(self.bit_index(x, y));
        web_sys::console::log_1(
            &format!(
                "x:{}, y:{}, is_alive: {}, index: {}",
                x,
                y,
                ret,
                self.bit_index(x, y)
            )
            .into(),
        );
        ret
    }

    pub fn set(&mut self, x: i32, y: i32, alive: bool) -> bool {
        let ret = self.cells[((self.tick_count) % 2) as usize].bit_get(self.bit_index(x, y));
        self.cells[((self.tick_count) % 2) as usize].bit_set(self.bit_index(x, y), alive);
        web_sys::console::log_1(
            &format!("x:{}, y:{}, set_alive:{}, is_alive:{}", x, y, alive, ret).into(),
        );
        ret
    }

    pub fn start_ptr_even(&self) -> *const u8 {
        self.cells[0].cells.as_ptr()
    }

    pub fn start_ptr_odd(&self) -> *const u8 {
        self.cells[1].cells.as_ptr()
    }

    pub fn bit_index(&self, x: i32, y: i32) -> usize {
        let wrapped_y = (y % self.height + self.height) % self.height;
        let wrapped_x = (x % self.width + self.width) % self.width;
        (wrapped_y * self.color_width * self.width + wrapped_x * self.color_width) as usize
    }

    pub fn count_neighbors(&self, x: i32, y: i32) -> f32 {
        let cells_read = &self.cells[(self.tick_count % 2) as usize];

        let mut sum = 0.0;
        for i in x - 1..x + 2 {
            for j in y - 1..y + 2 {
                if (i, j) != (x, y) {
                    if cells_read.bit_get(self.bit_index(i, j)) {
                        sum += 1.0;
                    }
                }
            }
        }
        sum
    }

    pub fn tick(&mut self, aabb: &AABB) {
        // web_sys::console::log_1(
        //     &format!(
        //         "Ticking ({}, {}) to ({}, {})",
        //         aabb.x,
        //         aabb.y,
        //         aabb.width + aabb.x,
        //         aabb.height + aabb.y
        //     )
        //     .into(),
        // );

        for y in aabb.y..aabb.height + aabb.y {
            for x in aabb.x..aabb.width + aabb.x {
                let alive = {
                    let count = self.count_neighbors(x, y) as u32;

                    self.cells[(self.tick_count % 2) as usize].bit_get(self.bit_index(x, y))
                        && count > 1
                        && count < 4
                        || count == 3
                };

                self.cells[((self.tick_count + 1) % 2) as usize]
                    .bit_set(self.bit_index(x, y), alive);
            }
        }

        self.tick_count += 1;
    }
}

use std::collections::HashSet;

use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
#[derive(PartialEq)]
pub struct Square {
    name: String,
    is_fixed: bool,
    x: f32,
    y: f32,
    size: f32,
    mass: f32,
    x_vel: f32,
    y_vel: f32,
    x_acc: f32,
    y_acc: f32
}

#[wasm_bindgen]
impl Square {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, is_fixed: bool, x: f32, y: f32, size: f32, mass: f32) -> Square {
        Square {name, is_fixed, x, y, size, mass, x_vel:0.0, y_vel:0.0, x_acc:0.0, y_acc:0.0 }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Force {
    name: String,
    x: f32,
    y: f32
}

#[wasm_bindgen]
impl Force {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, x: f32, y: f32) -> Force {
        Force { name, x, y }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f32 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f32 {
        self.y
    }
}

#[wasm_bindgen]
pub struct World {
    squares: Vec<Square>,
    forces: Vec<Force>,
    last_update: f64,
    x_max: f32,
    y_max: f32
}

#[wasm_bindgen] 
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(time: f64, canvas_width: f32, canvas_height: f32) -> World {
        
        World { squares: Vec::new(), forces: Vec::new(), last_update: time, x_max: canvas_width, y_max: canvas_height }
    }

    #[wasm_bindgen]
    pub fn add_square(&mut self, name: &str, is_fixed: bool, x: f32, y: f32, size: f32, mass: f32) {
        // Check for overlap with any existing square
        for s in &self.squares {
            let overlap_x = x < s.x + s.size && x + size > s.x;
            let overlap_y = y < s.y + s.size && y + size > s.y;
            if overlap_x && overlap_y {
                return;
            }
        }
        self.squares.push(Square::new(name.to_string(), is_fixed, x, y, size, mass));
    }


    #[wasm_bindgen]
    pub fn add_force(&mut self, force: Force) {
        self.forces.push(force);
    }

    /*returns a vector of f32 values, where those values have meaning grouped in 3
    1st: x; 2nd: y; 3rd: size*/
    #[wasm_bindgen]
    pub fn get_square_props(&self) -> Vec<f32> {
        self
        .squares
        .iter()
        .flat_map(|s| [s.x, s.y, s.size])
        .collect()
    }

    #[wasm_bindgen]
    pub fn get_global_forces(&self) -> Vec<Force> {
        self.forces.clone()
    }

    #[wasm_bindgen]
    pub fn change_force_x(&mut self, index: usize, x: f32) {
        self.forces[index].x = x;
    }

    #[wasm_bindgen]
    pub fn change_force_y(&mut self, index: usize, y: f32) {
        self.forces[index].y = y;
    }

    #[wasm_bindgen]
    pub fn update(&mut self, time: f64) {
        let dt = (time - self.last_update) as f32;
        self.last_update = time;

        // Update acceleration for non-fixed squares
        for s in self.squares.iter_mut().filter(|s| !s.is_fixed) {
            s.x_acc = self.forces.iter().map(|f| f.x).sum::<f32>() / s.mass;
            s.y_acc = self.forces.iter().map(|f| f.y).sum::<f32>() / s.mass;
        }

        let sqr_len = self.squares.len();

        for i in 0..sqr_len {
            // Split the slice to avoid Rust borrowing issues
            let (left, right) = self.squares.split_at_mut(i);
            let (a, right) = right.split_first_mut().unwrap(); // a = self.squares[i]

            if a.is_fixed { continue; }

            // Predict next position
            let mut future_x = a.x + a.x_vel * dt + 0.5 * a.x_acc * dt.powi(2);
            let mut future_y = a.y + a.y_vel * dt + 0.5 * a.y_acc * dt.powi(2);

            let mut iterations = 0;
            loop {
                if iterations > 5 { break; } // prevent infinite loops in dense clusters
                iterations += 1;

                let mut collision_found = false;

                // Check against left part of the slice
                for b in left.iter() {
                    let b_left = b.x;
                    let b_right = b.x + b.size;
                    let b_top = b.y;
                    let b_bottom = b.y + b.size;

                    let a_left = future_x;
                    let a_right = future_x + a.size;
                    let a_top = future_y;
                    let a_bottom = future_y + a.size;

                    let x_overlap = (a_right.min(b_right) - a_left.max(b_left)).max(0.0);
                    let y_overlap = (a_bottom.min(b_bottom) - a_top.max(b_top)).max(0.0);

                    if x_overlap > 0.0 && y_overlap > 0.0 {
                        collision_found = true;

                        if x_overlap < y_overlap {
                            if a_left < b_left {
                                future_x = b_left - a.size;
                            } else {
                                future_x = b_right;
                            }
                            a.x_vel = 0.0;
                        } else {
                            if a_top < b_top {
                                future_y = b_top - a.size;
                            } else {
                                future_y = b_bottom;
                            }
                            a.y_vel = 0.0;
                        }
                    }
                }

                // Check against right part of the slice
                for b in right.iter() {
                    let b_left = b.x;
                    let b_right = b.x + b.size;
                    let b_top = b.y;
                    let b_bottom = b.y + b.size;

                    let a_left = future_x;
                    let a_right = future_x + a.size;
                    let a_top = future_y;
                    let a_bottom = future_y + a.size;

                    let x_overlap = (a_right.min(b_right) - a_left.max(b_left)).max(0.0);
                    let y_overlap = (a_bottom.min(b_bottom) - a_top.max(b_top)).max(0.0);

                    if x_overlap > 0.0 && y_overlap > 0.0 {
                        collision_found = true;

                        if x_overlap < y_overlap {
                            if a_left < b_left {
                                future_x = b_left - a.size;
                            } else {
                                future_x = b_right;
                            }
                            a.x_vel = 0.0;
                        } else {
                            if a_top < b_top {
                                future_y = b_top - a.size;
                            } else {
                                future_y = b_bottom;
                            }
                            a.y_vel = 0.0;
                        }
                    }
                }

                if !collision_found { break; }
            }

            // Apply final position and update velocity
            a.x = future_x;
            a.y = future_y;
            a.x_vel += a.x_acc * dt;
            a.y_vel += a.y_acc * dt;
        }

        // Remove invisible squares
        self.squares.retain(|s| 
            s.x + s.size >= 0.0 &&
            s.y + s.size >= 0.0 &&
            s.x <= self.x_max &&
            s.y <= self.y_max
        );
    }

    
}
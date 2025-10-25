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
}

#[wasm_bindgen]
pub struct World {
    squares: Vec<Square>,
    forces: Vec<Force>,
    last_update: f64
}

#[wasm_bindgen] 
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(time: f64) -> World {
        
        World { squares: Vec::new(), forces: Vec::new(), last_update: time }
    }

    #[wasm_bindgen]
    pub fn add_square(&mut self, square: Square) {
        self.squares.push(square);
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
    pub fn update(&mut self, time: f64) {
        let dt = (time - self.last_update) as f32;
        self.last_update = time;

        // Updateacceleration
        for s in self.squares.iter_mut().filter(|s| !s.is_fixed) {
            s.x_acc = self.forces.iter().map(|f| f.x).sum::<f32>() / s.mass;
            s.y_acc = self.forces.iter().map(|f| f.y).sum::<f32>() / s.mass;
        }

        //Collision detection & positioning
        let sqr_len = self.squares.len();
        for i in 0..sqr_len {
            let a = &self.squares[i];
            if a.is_fixed { continue; }
            let mut after_collision = None;
            let no_collision_x = a.x + a.x_vel * dt + 0.5 * a.x_acc * dt.powi(2);
            let no_collision_y = a.y + a.y_vel * dt + 0.5 * a.y_acc * dt.powi(2);


            for j in 0..sqr_len {
                if j >= i {continue;}
                let b = &self.squares[j];

                let a_left_bound = a.x;
                let a_right_bound = a.x + a.size;
                let a_top_bound = a.y;
                let a_bottom_bound = a.y + a.size;
                
                let b_left_bound = b.x;
                let b_right_bound = b.x + b.size;
                let b_top_bound = b.y;
                let b_bottom_bound = b.y + b.size;
                
                //when only moves on the y axis
                if a_bottom_bound <= b_top_bound
                    && no_collision_y + a.size >= b_top_bound
                    && a_right_bound >= b_left_bound
                    && b_right_bound >= a_left_bound 
                {
                    after_collision = Some((a.x, b.y - b.size));
                    break;
                }
            }
            let a = &mut self.squares[i];
            if let Some(coordinates) = after_collision {
                (a.x, a.y) = coordinates;
                (a.x_vel, a.y_vel) = (0.0, 0.0);
                a.is_fixed = true;
                
            } else {
                (a.x, a.y) = (no_collision_x, no_collision_y);                
                a.x_vel += a.x_acc * dt;
                a.y_vel += a.y_acc * dt;
            }

        }
    }
}
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Square {
    name: String,
    x: f32,
    y: f32,
    size: f32,
    mass: f32,
}

#[wasm_bindgen]
impl Square {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, x: f32, y: f32, size: f32, mass: f32) -> Square {
        Square {name, x, y, size, mass }
    }
}

#[wasm_bindgen]
pub struct World {
    squares: Vec<Square>
}

#[wasm_bindgen] 
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new() -> World {
        World { squares: Vec::new() }
    }

    pub fn add_square(&mut self, square: Square) {
        self.squares.push(square);
    }

    /*returns a vector of f32 values, where those values have meaning grouped in 3
    1st: x; 2nd: y; 3rd: size*/
    pub fn get_square_props(&self) -> Vec<f32> {
        self
        .squares
        .iter()
        .flat_map(|s| [s.x, s.y, s.size])
        .collect()
    }


}
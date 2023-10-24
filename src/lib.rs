mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("HI KURKA!");
}

#[wasm_bindgen]
pub fn return_raw_42() -> i32 {
    return 42;
}

#[wasm_bindgen]
pub struct Universe {
    pub answer: i32,
}

#[wasm_bindgen]
impl Universe {
    #[wasm_bindgen(constructor)]
    pub fn new(n: i32) -> Self {
        Universe { answer: n }
    }

    pub fn cool_universe() -> Self {
        Universe::new(42)
    }

    pub fn get_answer(&self) -> i32 {
        self.answer
    }
}

#[wasm_bindgen(inspectable)]
pub struct InspectableUniverse {
    pub answer: i32,
}

#[wasm_bindgen]
impl InspectableUniverse {
    #[wasm_bindgen(constructor)]
    pub fn new(n: i32) -> Self {
        InspectableUniverse { answer: n }
    }

    pub fn cool_universe() -> Self {
        InspectableUniverse::new(42)
    }
}

#[wasm_bindgen]
pub fn rust_please_sort_my_numbers(mut numbers: Vec<i32>) -> Vec<i32> {
    numbers.sort();
    numbers
}

#[wasm_bindgen]
pub fn increment_universe_answer(universe: &mut InspectableUniverse) {
    universe.answer += 1;
}

#[wasm_bindgen]
pub fn consume_universe(_universe: InspectableUniverse) {}

////////////////////////////////////////////////////////

#[wasm_bindgen]
pub fn get_max_i64() -> i64 {
    // translates correctly to `bigint`` in js
    i64::MAX
}

#[wasm_bindgen]
pub fn get_max_i8() -> i8 {
    // translates to `number` in js
    i8::MAX
}

#[wasm_bindgen]
pub fn get_max_u8() -> u8 {
    // translates to `number` in js
    u8::MAX
}

#[wasm_bindgen]
pub fn get_max_f64() -> f64 {
    // translates to `number` in js
    f64::MAX
}

#[wasm_bindgen]
pub fn get_nan_f64() -> f64 {
    // translates to `number` in js
    f64::NAN
}

#[wasm_bindgen]
pub fn get_inf_f64() -> f64 {
    // translates to `number` in js
    f64::INFINITY
}

#[wasm_bindgen]
pub fn get_neg_inf_f64() -> f64 {
    // translates to `number` in js
    f64::NEG_INFINITY
}

// Not possible !!!
// #[wasm_bindgen]
// pub fn get_max_i128() -> i128 {
//     i128::MAX
// }

#[wasm_bindgen]
pub fn accept_u8_and_return_it(number: u8) -> u8 {
    // in js this function accepts `number` and can overflow
    number
}

// Not possible !!! quite obviously
// #[wasm_bindgen]
// pub fn return_mut_ref() -> &mut i32 {
//     todo!()
// }

// Not possible !!! quite obviously
// #[wasm_bindgen]
// pub fn generic<T>(d: T) {}

/// We all love docs, we really love good docs!.
///
/// Hopefully this works :D
#[wasm_bindgen]
pub fn documented_function() {}

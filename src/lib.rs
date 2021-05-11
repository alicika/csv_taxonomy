mod utils;

use wasm_bindgen::prelude::*;
use ndarray::{Array2};
use std::str::FromStr;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn fit (csv_content: &[u8], num_clusters: usize) -> Vec<f64> {
    let data: Vec<f64> = read_data(csv_content);
    let arr = Array2::from_shape_vec((data.len() / 2, 2), data).unwrap();
    let (means, _clusters) = rkm::kmeans_lloyd(&arr.view(), num_clusters);

    let mut serialized_vec : Vec<f64> = Vec::new();
    for row in means.genrows() {
        serialized_vec.push(row[0]);
        serialized_vec.push(row[1]);
    }
    return serialized_vec;
}

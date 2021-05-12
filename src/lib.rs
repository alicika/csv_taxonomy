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
pub fn fit (csv_content: &[u8], num_clusters: usize) -> Vec<f64> {
    let data: Vec<f32> = read_data(csv_content);
    let arr = Array2::from_shape_vec((data.len() / 2, 2), data).unwrap();
    let (means, _clusters) = rkm::kmeans_lloyd(&arr.view(), num_clusters as usize);

    let mut serialized_vec : Vec<f64> = Vec::new();
    for row in means.genrows() {
        serialized_vec.push(row[0]);
        serialized_vec.push(row[1]);
    }
    return serialized_vec;
}

fn read_data(csv_content: &[u8]) -> Vec<f32> {
    let v : Vec<u8> = csv_content.to_vec();
    println!("長さ: {}", v.len());

    let mut data_reader = csv::Reader::from_reader(csv_content);
    let mut data: Vec<f32> = Vec::new();
    for record in data_reader.records() {
        for field in record.unwrap().iter() {
            let value = f32::from_str(field);
            data.push(value.unwrap());
        }
    }
    data
}
// importing packages/macros
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log; // log_1 function logs one value. log_2 logs 2, etc etc
use base64::{ decode, encode }; // to convert base64 to vector and back
use image::load_from_memory; // stores copy of image for manipulation
use image::ImageOutputFormat::Png;

/* 
    Will transfer a file from JS to Rust.
    We are borrowing the string because we aren't updating the string directly
    Instead we are transforming the string to a binary image file
*/

/* 
macro attributes: feature to run macros on a function
bindgen crate exports a macro for handling action of exporting a fn to JS 
*/
#[wasm_bindgen]
// pub modifier makes function public, but only to Rust
pub fn grayscale(encoded_file: &str) -> String {
    // Need type conversion. Rust strings != JS strings
    // into() can be used for type conversion 
    log(&encoded_file.into());
    log(&"Grayscale called".into());

    // U8 type - unsigned number, ie numbers are always positive, since binary data always positive
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    // Unwrapped object is of type DynamicImage, an object with methods for interacting with an image
    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    img = img.grayscale();
    log(&"Grayscale effect applied".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap(); // initiates process of converting an image into binary data
    log(&"New image written".into());

    let encoded_img = encode(&buffer); // returns a Base64 string
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_img
    );

    return data_url;

}
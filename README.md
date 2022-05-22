# Image processing with Web Assembly
## Goal
Took a quick course on Zero To Mastery to learn the basics of Web Assembly. As a bonus, we get to learn the basics of the Rust language.
## App
The app does the following:
- Upload a PNG file using HTML
Read file in JS using FileReader class
Convert image file into Base64 string
Transferring files in binary format between programs can be tricky
Sendings strings between programs is easy, hence FileReader.result → string with Base64 encoding
Pass Base64 string to Rust function
Rust functions must be exposed to JS. Needs two things: 1) Rust function must be public (pub modifier) and, 2) have wasm_bindgen macro attribute to handle export from Rust to JS
Transform string to binary file in Rust
Manipulate image in Rust (image is of type DynamicImage)
Use buffer to store image
Encode image to Base64 string
Pass string to JS to render the image

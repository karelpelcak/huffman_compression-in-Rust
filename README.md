# Huffman Image Compression in Rust

This Rust project demonstrates how to compress an image using Huffman encoding and then save it as a compressed file. Additionally, it provides functionality to save the original image in JPEG format with adjustable quality.

## Table of Contents

1. [Features](#features)
2. [Dependencies](#dependencies)
3. [How It Works](#how-it-works)
4. [Usage](#usage)
5. [Installation and Running](#installation-and-running)
6. [Example Output](#example-output)
7. [License](#license)

## Features

- **Image Compression using Huffman Encoding**: The program reads an image, computes the frequency of each pixel, builds a Huffman tree, and generates binary codes for each pixel color.
- **Save Compressed Data to a File**: The compressed image data is saved as a text file.
- **JPEG Image Saving with Adjustable Quality**: Allows saving the original image in JPEG format with a specified quality level (0-100).

## Dependencies

The project uses the following Rust crates:

- [`image`](https://crates.io/crates/image): For image processing and handling different image formats.
- [`jpeg-encoder`](https://crates.io/crates/jpeg-encoder): For encoding and saving images in JPEG format.
- [`std`](https://doc.rust-lang.org/std/): Standard library for I/O, file handling, and collections.

Make sure to add these dependencies to your `Cargo.toml` file:

```toml
[dependencies]
image = "0.24.6"
jpeg-encoder = "0.2.0"
```

## How It Works

1. **Loading the Image**: The program reads an image file (e.g., PNG or JPEG) using the `image` crate.
2. **Building Huffman Tree**:
   - It calculates the frequency of each pixel's color (RGBA).
   - A Huffman tree is constructed based on the pixel frequencies.
   - Binary codes are generated for each pixel using the tree.
3. **Compressing the Image Data**: The original image pixels are replaced with their corresponding binary Huffman codes.
4. **Saving the Compressed Data**: The compressed data is written to a text file.
5. **Saving the Image in JPEG Format**: The original image is saved in a JPEG file with adjustable quality settings.

## Usage

1. **Compression and Saving**:
   - The main function compresses the image and saves the compressed data to `compressed_image.txt`.
   - It also saves the original image as `output_image.jpg` with the specified quality.

2. **Adjusting Compression Quality**:
   - You can change the quality level in the `main()` function by modifying the `quality` variable (range 0-100).

## Installation and Running

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/your-username/huffman-image-compression.git
   cd huffman-image-compression
   ```

2. **Add Dependencies**: Ensure you have the necessary dependencies in your `Cargo.toml`.

3. **Build the Project**:
   ```bash
   cargo build --release
   ```

4. **Run the Program**:
   ```bash
   cargo run --release
   ```

5. **Modify Image Path and Quality**:
   - Edit the `main()` function to specify the path to your image and adjust the quality as needed.

## Example Output

- Compressed data will be saved in `compressed_image.txt`.
- The JPEG version of the original image will be saved as `output_image.jpg` with the specified quality.

### Sample Output:
```
Compressed data length: 154863
Codes: {Rgba(255, 0, 0, 255): "010", Rgba(0, 255, 0, 255): "001", ...}
```

## License

This project is open source and available under the MIT License. You are free to use, modify, and distribute it as per the terms of the license.

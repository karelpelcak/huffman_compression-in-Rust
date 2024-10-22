use image::{DynamicImage, GenericImageView, Rgba};
use jpeg_encoder::{Encoder, ColorType};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

// Huffmanův uzel
#[derive(Debug)]
struct Node {
    value: Option<Rgba<u8>>,
    frequency: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: Option<Rgba<u8>>, frequency: usize) -> Self {
        Node {
            value,
            frequency,
            left: None,
            right: None,
        }
    }
}

// BinaryHeap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl Eq for Node {}

// stavba Huffmanova stromu
fn build_huffman_tree(frequencies: &HashMap<Rgba<u8>, usize>) -> Option<Box<Node>> {
    let mut heap = BinaryHeap::new();

    for (value, &freq) in frequencies {
        heap.push(Node::new(Some(*value), freq));
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        let mut merged = Node::new(None, left.frequency + right.frequency);
        merged.left = Some(Box::new(left));
        merged.right = Some(Box::new(right));

        heap.push(merged);
    }

    heap.pop().map(Box::new)
}

// Generování kódů
fn generate_codes(node: &Node, prefix: String, codes: &mut HashMap<Rgba<u8>, String>) {
    if let Some(value) = node.value {
        codes.insert(value, prefix);
    } else {
        if let Some(ref left) = node.left {
            generate_codes(left, format!("{}0", prefix), codes);
        }
        if let Some(ref right) = node.right {
            generate_codes(right, format!("{}1", prefix), codes);
        }
    }
}

// Komprese obrázku
fn compress_image(image: &DynamicImage) -> (String, HashMap<Rgba<u8>, String>) {
    let mut frequencies = HashMap::new();

    // Získání pixelových hodnot
    for pixel in image.pixels() {
        let rgba = pixel.2; // Rgba<u8>
        *frequencies.entry(rgba).or_insert(0) += 1;
    }

    let root = build_huffman_tree(&frequencies).unwrap();
    let mut codes = HashMap::new();
    generate_codes(&root, String::new(), &mut codes);

    // Komprese dat
    let compressed: String = image.pixels()
        .map(|pixel| codes[&pixel.2].clone())
        .collect();

    (compressed, codes)
}

// Uložení komprimovaných dat do souboru
fn save_to_file<P: AsRef<Path>>(path: P, compressed: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(compressed.as_bytes())?;
    Ok(())
}

fn save_image_with_quality<P: AsRef<Path>>(image: &DynamicImage, path: P, quality: u8) -> Result<(), Box<dyn std::error::Error>> {
    // Převod na formát RGB8, protože jpeg-encoder nepodporuje RGBA
    let rgb_image = image.to_rgb8();
    let (width, height) = rgb_image.dimensions();

    // Vytvoření výstupního souboru
    let mut output = File::create(path)?;

    // Kódování s nastavením kvality
    let encoder = Encoder::new(&mut output, quality);
    encoder.encode(&rgb_image, width as u16, height as u16, ColorType::Rgb)?;

    Ok(())
}

// Hlavní funkce
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Načtení obrázku
    let img_path = "images.png";
    let img = image::open(img_path)?;

    // Komprese obrázku
    let (compressed_data, codes) = compress_image(&img);
    
    println!("Compressed data length: {}", compressed_data.len());
    println!("Codes: {:?}", codes);

    // Uložení komprimovaných dat do souboru
    save_to_file("compressed_image.txt", &compressed_data)?;

    // Uložení zmenšeného obrázku s kvalitou
    let quality = 70; // nastavte kvalitu v procentech (0-100)
    save_image_with_quality(&img, "output_image.jpg", quality)?;

    Ok(())
}

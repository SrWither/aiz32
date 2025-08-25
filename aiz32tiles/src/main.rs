use std::fs::File;
use std::io::Write;
use std::env;
use image::GenericImageView;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Uso: {} <imagen> <tile_size> <salida_rom>", args[0]);
        eprintln!("Ejemplo: {} tiles.png 16 tiles.rom", args[0]);
        return;
    }

    let img_path = &args[1];
    let tile_size: u32 = args[2].parse().expect("tile_size debe ser un número");
    let rom_path = &args[3];

    let img = image::open(img_path).expect("No se pudo abrir la imagen");
    let (img_width, img_height) = img.dimensions();

    assert!(img_width % tile_size == 0 && img_height % tile_size == 0,
        "La imagen debe ser múltiplo del tamaño del tile");

    let tiles_x = img_width / tile_size;
    let tiles_y = img_height / tile_size;

    let mut rom: Vec<u32> = Vec::new();

    for ty in 0..tiles_y {
        for tx in 0..tiles_x {
            for py in 0..tile_size {
                for px in 0..tile_size {
                    let px_x = tx * tile_size + px;
                    let px_y = ty * tile_size + py;
                    let pixel = img.get_pixel(px_x, px_y);
                    let color = ((pixel[0] as u32) << 16)
                              | ((pixel[1] as u32) << 8)
                              | (pixel[2] as u32);
                    rom.push(color);
                }
            }
        }
    }

    // Guardar ROM
    let mut file = File::create(rom_path).expect("No se pudo crear el archivo");
    for color in rom {
        file.write_all(&color.to_be_bytes()).expect("Error escribiendo ROM");
    }

    println!("ROM generada con éxito: {}", rom_path);
}

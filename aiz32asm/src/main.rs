use aiz32asm::{assemble_file, assemble_to_formats, opcode::opcode_table};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Uso: aiz32asm <archivo.asm> <salida.bin> [--raw]");
        std::process::exit(1);
    }

    let input = &args[1];
    let output = &args[2];
    let table = opcode_table();
    let generate_raw = args.get(3).map_or(false, |s| s == "--raw");

    let content = fs::read_to_string(input).expect("No se pudo leer el archivo .asm");
    let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    if generate_raw {
        let (rawhex, rawbin) = assemble_to_formats(lines, &table);

        let input_path = Path::new(input);
        let base_name = input_path
            .file_stem()
            .expect("Archivo sin nombre")
            .to_string_lossy();

        let hex_file = format!("{}.rawhex", base_name);
        fs::write(&hex_file, rawhex.join("\n")).expect("No se pudo escribir el archivo rawhex");
        println!("Archivo rawhex generado: {}", hex_file);

        let bin_file = format!("{}.rawbin", base_name);
        fs::write(&bin_file, rawbin.join("\n")).expect("No se pudo escribir el archivo rawbin");
        println!("Archivo rawbin generado: {}", bin_file);
    }

    assemble_file(input, output, &table);
    println!("Archivo compilado correctamente: {}", output);
}

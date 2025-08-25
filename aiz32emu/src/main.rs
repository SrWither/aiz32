pub mod console;
pub mod gpu;

use aiz32core::{alu::Flags, cpu::CPU};
use std::cell::RefCell;
use std::env;
use std::fs;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::console::Console;
use crate::gpu::GPU;
use sdl2::pixels::PixelFormatEnum;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Uso: {} <binario> <ram_size> <sp_base> <debug>", args[0]);
        return;
    }

    let program_path = &args[1];
    let ram_size: usize = args[2].parse().expect("RAM size inválido");
    let sp_base: u32 = args[3].parse().expect("SP base inválida");
    let debug: bool = args[4].parse::<u8>().unwrap_or(0) != 0;

    let program = fs::read(program_path).expect("No se pudo leer el archivo binario");
    let pc_dir = ram_size as u32;
    let mut cpu = CPU::new(ram_size, program.clone(), sp_base, pc_dir);

    let gpu = Rc::new(RefCell::new(GPU::new(320, 240, vec![])));
    let console = Rc::new(RefCell::new(Console::new()));
    cpu.io.register_peripheral(console.clone());
    cpu.io.register_peripheral(gpu.clone());

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gpu_size = {
        let g = gpu.borrow();
        (g.width as u32, g.height as u32)
    };

    let scale_factor = 2;
    let scaled_width = gpu_size.0 * scale_factor;
    let scaled_height = gpu_size.1 * scale_factor;

    let window = video_subsystem
        .window("AIZ32", scaled_width, scaled_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync() // sincronización vertical para 60 FPS
        .build()
        .unwrap();

    canvas.set_logical_size(gpu_size.0, gpu_size.1).unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::ARGB8888, gpu_size.0, gpu_size.1)
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    let target_frame_duration = Duration::from_secs_f64(1.0 / 60.0);
    let mut last_frame_time = Instant::now();

    // ciclo principal
    while !cpu.halted {
        // Ejecuta un número razonable de pasos de CPU por frame
        for _ in 0..50_000 {
            if cpu.halted { break; }
            cpu.step();
        }

        // Renderizado GPU
        {
            let mut gpu_borrow = gpu.borrow_mut();

            if gpu_borrow.frame_dirty {
                let width = gpu_borrow.width;
                let fb = gpu_borrow.framebuffer();
                texture.update(None, bytemuck::cast_slice(fb), width * 4).unwrap();

                canvas.clear();
                canvas.copy(&texture, None, None).unwrap();
                canvas.present();

                gpu_borrow.present(); // swap de buffers
            }
        }

        // Control de FPS
        let now = Instant::now();
        let elapsed = now.duration_since(last_frame_time);
        if elapsed < target_frame_duration {
            std::thread::sleep(target_frame_duration - elapsed);
        }
        last_frame_time = Instant::now();

        // Manejo de eventos SDL
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return,
                _ => {}
            }
        }

        // Debug opcional
        if debug {
            let flags = Flags::from_u32(cpu.regs.flags());
            println!(
                "PC: {:08X}, SP: {:08X}, Flags: Z={} C={} O={}",
                cpu.regs.pc(),
                cpu.regs.sp(),
                flags.zero as u8,
                flags.carry as u8,
                flags.overflow as u8
            );
        }
    }
}

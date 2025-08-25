pub mod console;
pub mod gpu;
pub mod keyboard;

use aiz32core::{alu::Flags, cpu::CPU};
use sdl2::keyboard::Mod;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::console::Console;
use crate::gpu::GPU;
use crate::keyboard::Keyboard;

use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

fn load_gpu_rom(path: &str) -> Vec<u32> {
    let mut file = File::open(path).expect("No se pudo abrir el archivo ROM");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Error leyendo ROM");

    assert!(
        buf.len() % 4 == 0,
        "ROM inválida: debe ser múltiplo de 4 bytes"
    );

    buf.chunks(4)
        .map(|chunk| {
            ((chunk[0] as u32) << 24)
                | ((chunk[1] as u32) << 16)
                | ((chunk[2] as u32) << 8)
                | (chunk[3] as u32)
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 7 {
        eprintln!(
            "Uso: {} <binario> <ram_size> <sp_base> <debug> <gpu_width> <gpu_height> <gpu_rom>",
            args[0]
        );
        eprintln!(
            "Ejemplo: {} program.bin 65536 65535 0 640 480 tiles.rom",
            args[0]
        );
        return;
    }

    let program_path = &args[1];
    let ram_size: usize = args[2].parse().expect("RAM size inválido");
    let sp_base: u32 = args[3].parse().expect("SP base inválida");
    let debug: bool = args[4].parse::<u8>().unwrap_or(0) != 0;

    let gpu_width: usize = args[5].parse().expect("GPU width inválido");
    let gpu_height: usize = args[6].parse().expect("GPU height inválido");
    let gpu_rom_path = &args[7];

    let program = fs::read(program_path).expect("No se pudo leer el archivo binario");
    let pc_dir = ram_size as u32;
    let mut cpu = CPU::new(ram_size, program.clone(), sp_base, pc_dir);

    let gpu_rom = load_gpu_rom(gpu_rom_path);
    let gpu = Rc::new(RefCell::new(GPU::new(gpu_width, gpu_height, gpu_rom)));

    let console = Rc::new(RefCell::new(Console::new()));

    let keyboard = Rc::new(RefCell::new(Keyboard::new()));
    cpu.io.register_peripheral(console.clone());
    cpu.io.register_peripheral(gpu.clone());
    cpu.io.register_peripheral(keyboard.clone());

    // Inicialización SDL
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let scale_factor = 1;
    let scaled_width = gpu_width as u32 * scale_factor;
    let scaled_height = gpu_height as u32 * scale_factor;

    let window = video_subsystem
        .window("AIZ32", scaled_width, scaled_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    canvas
        .set_logical_size(gpu_width as u32, gpu_height as u32)
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::ARGB8888,
            gpu_width as u32,
            gpu_height as u32,
        )
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    let target_frame_duration = Duration::from_secs_f64(1.0 / 60.0);
    let mut last_frame_time = Instant::now();

    // ciclo principal
    while !cpu.halted {
        for _ in 0..50_000 {
            if cpu.halted {
                break;
            }
            cpu.step();
        }

        {
            let mut gpu_borrow = gpu.borrow_mut();

            let fb = gpu_borrow.framebuffer();
            texture
                .update(None, bytemuck::cast_slice(fb), gpu_width * 4)
                .unwrap();

            canvas.clear();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();

            gpu_borrow.present();
        }

        let now = Instant::now();
        let elapsed = now.duration_since(last_frame_time);
        if elapsed < target_frame_duration {
            std::thread::sleep(target_frame_duration - elapsed);
        }
        last_frame_time = Instant::now();

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return,
                Event::KeyDown {
                    keycode: Some(k),
                    keymod,
                    ..
                } => {
                    let shift = keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
                    let key = map_keycode(k, shift);
                    keyboard.borrow_mut().key_down(key);
                }
                Event::KeyUp {
                    keycode: Some(k),
                    keymod,
                    ..
                } => {
                    let shift = keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
                    let key = map_keycode(k, shift);
                    keyboard.borrow_mut().key_up(key);
                }

                _ => {}
            }
        }

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

fn map_keycode(key: Keycode, shift: bool) -> u8 {
    if key.name() == "{" {
        if shift {
            return 123;
        } else {
            return 91;
        }
    }

    if key.name() == "}" {
        if shift {
            return 125;
        } else {
            return 93;
        }
    }

    match key {
        // Letras (A–Z y a–z)
        Keycode::A => {
            if shift {
                65
            } else {
                97
            }
        }
        Keycode::B => {
            if shift {
                66
            } else {
                98
            }
        }
        Keycode::C => {
            if shift {
                67
            } else {
                99
            }
        }
        Keycode::D => {
            if shift {
                68
            } else {
                100
            }
        }
        Keycode::E => {
            if shift {
                69
            } else {
                101
            }
        }
        Keycode::F => {
            if shift {
                70
            } else {
                102
            }
        }
        Keycode::G => {
            if shift {
                71
            } else {
                103
            }
        }
        Keycode::H => {
            if shift {
                72
            } else {
                104
            }
        }
        Keycode::I => {
            if shift {
                73
            } else {
                105
            }
        }
        Keycode::J => {
            if shift {
                74
            } else {
                106
            }
        }
        Keycode::K => {
            if shift {
                75
            } else {
                107
            }
        }
        Keycode::L => {
            if shift {
                76
            } else {
                108
            }
        }
        Keycode::M => {
            if shift {
                77
            } else {
                109
            }
        }
        Keycode::N => {
            if shift {
                78
            } else {
                110
            }
        }
        Keycode::O => {
            if shift {
                79
            } else {
                111
            }
        }
        Keycode::P => {
            if shift {
                80
            } else {
                112
            }
        }
        Keycode::Q => {
            if shift {
                81
            } else {
                113
            }
        }
        Keycode::R => {
            if shift {
                82
            } else {
                114
            }
        }
        Keycode::S => {
            if shift {
                83
            } else {
                115
            }
        }
        Keycode::T => {
            if shift {
                84
            } else {
                116
            }
        }
        Keycode::U => {
            if shift {
                85
            } else {
                117
            }
        }
        Keycode::V => {
            if shift {
                86
            } else {
                118
            }
        }
        Keycode::W => {
            if shift {
                87
            } else {
                119
            }
        }
        Keycode::X => {
            if shift {
                88
            } else {
                120
            }
        }
        Keycode::Y => {
            if shift {
                89
            } else {
                121
            }
        }
        Keycode::Z => {
            if shift {
                90
            } else {
                122
            }
        }

        // Números 0–9 (con shift para símbolos)
        Keycode::Num1 => {
            if shift {
                33
            } else {
                49
            }
        } // ! o 1
        Keycode::Num2 => {
            if shift {
                34
            } else {
                50
            }
        } // " o 2
        Keycode::Num3 => {
            if shift {
                35
            } else {
                51
            }
        } // # o 3
        Keycode::Num4 => {
            if shift {
                36
            } else {
                52
            }
        } // $ o 4
        Keycode::Num5 => {
            if shift {
                37
            } else {
                53
            }
        } // % o 5
        Keycode::Num6 => {
            if shift {
                38
            } else {
                54
            }
        } // & o 6
        Keycode::Num7 => {
            if shift {
                47
            } else {
                55
            }
        } // / o 7
        Keycode::Num8 => {
            if shift {
                40
            } else {
                56
            }
        } // ( o 8
        Keycode::Num9 => {
            if shift {
                41
            } else {
                57
            }
        } // ) o 9
        Keycode::Num0 => {
            if shift {
                61
            } else {
                48
            }
        } // = o 0

        Keycode::Quote => 39, // '

        Keycode::Plus => {
            if shift {
                42
            } else {
                43
            }
        }
        Keycode::Comma => {
            if shift {
                59
            } else {
                44
            }
        } // < o ,
        Keycode::Minus => {
            if shift {
                95
            } else {
                45
            }
        } // _ o -
        Keycode::Period => {
            if shift {
                58
            } else {
                46
            }
        } // : o .
        Keycode::Slash => {
            if shift {
                63
            } else {
                47
            }
        } // ? o /
        Keycode::Semicolon => {
            if shift {
                58
            } else {
                59
            }
        } // : o ;
        Keycode::Equals => {
            if shift {
                43
            } else {
                61
            }
        } // + o =
        Keycode::At => 64, // @
        Keycode::LeftBracket => {
            if shift {
                123
            } else {
                91
            }
        } // { o [
        Keycode::Backslash => {
            if shift {
                124
            } else {
                92
            }
        } // | o \
        Keycode::RightBracket => {
            if shift {
                125
            } else {
                93
            }
        } // } o ]
        Keycode::Caret => 94, // ^
        Keycode::Backquote => {
            if shift {
                126
            } else {
                96
            }
        } // ~ o `

        Keycode::Less => {
            if shift {
                62
            } else {
                60
            }
        } // > o <

        // Keycode::LeftBracket => 123,  // {
        // // Keycode::Bar => 124,          // |
        // Keycode::RightBracket => 125, // }
        // Keycode::Tilde => 126,        // ~

        // Control
        Keycode::Space => 158,
        Keycode::Backspace => 200,
        Keycode::Return => 201,

        _ => 0,
    }
}

use std::path::Path;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;

struct World {
    pixels: Vec<u8>,
}

impl World {
    fn new() -> Self {
        Self {
            pixels: vec![211; (WIDTH * HEIGHT * 4) as usize],
        }
    }

    fn draw_pixel(&mut self, x: u32, y: u32, color: [u8; 4]) {
        if x < WIDTH && y < HEIGHT {
            let index = ((y * WIDTH + x) * 4) as usize;
            self.pixels[index..index + 4].copy_from_slice(&color);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut content = String::new();
    let args: Vec<String> = env::args().collect();
    if (args.len() != 2) 
    {
        println!("Usage: ./Baguette <File Path>");
        return Ok(());
    }
    let filepath : &String = &args[1];
    if (Path::new(filepath).extension().unwrap() != "baguette" && Path::new(filepath).extension().unwrap() != "croissant") {
        println!("invalid file extension");
        println!("{:?}", Path::new(filepath).extension().unwrap());
    }

    let mut file = File::open(filepath)?;
    file.read_to_string(&mut content)?;
    let content_array: Vec<&str> = convert_string_into_array(content.as_str());
    init_window(content_array)?;

    #[allow(unreachable_code)]
    Ok(())
}

fn convert_string_into_array(file_content: &str) -> Vec<&str> {
    let arr : Vec<&str> = file_content.split(' ').collect();
    let mut split_arr = Vec::new();
    for i in 0..arr.len() {
        if (arr[i].contains('\n')) {
            let mut split_vec = arr[i].split('\n').collect::<Vec<&str>>();
            split_arr.push("\n");
            split_arr.push(&mut split_vec[1]);
        } else {
            split_arr.push(arr[i]);
        }
    }
    return split_arr;
}

fn init_window(content_array: Vec<&str>) -> Result<(), pixels::Error>
{
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pixel Drawing")
        .with_inner_size(LogicalSize::new(WIDTH as u32, HEIGHT as u32))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
            pixels.clear_color(pixels::wgpu::Color::BLACK);
            pixels
        };

    let mut canvas = World::new();

    let mut width = 40;
    let mut height = 40;

    for &word in &content_array {
        match word {
            "\n" => {
                height += 1;
                width = 40;
            }
            "Croissant" | "croissant" => {
                if (width < WIDTH -40 && height < HEIGHT -40) {
                    canvas.draw_pixel(width, height, [0, 0, 0, 255]);
                }
                width += 1;
            }
            "Baguette" | "baguette" => {
                if (width < WIDTH - 40 && height < HEIGHT -40) {
                    canvas.draw_pixel(width, height, [179, 145, 103, 255]);
                }
                width += 1;
            }
            _ => width += 1,
        }

        if (width >= WIDTH - 40) {
            height += 1;
            width = 40;
        }
        if (height >= HEIGHT - 40) {
            break;
        }
    }

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                pixels.frame_mut().copy_from_slice(&canvas.pixels);
                if let Err(err) = pixels.render() {
                    eprintln!("Render error: {}", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    
        window.request_redraw();
    });
}


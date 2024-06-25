use std::path::Path;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

use pixels::{Pixels, SurfaceTexture};

use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use image::{ImageBuffer, Rgba};

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;
const VIEWPORT_WIDTH: u32 = 880;
const VIEWPORT_HEIGHT: u32 = 460;

struct World {
    pixels: Vec<u8>,
    background_color: [u8; 4],
}

impl World {
    fn new() -> Self {
        Self {
            pixels: vec![211; (WIDTH * HEIGHT * 4) as usize],
            background_color: [211, 211, 211, 255], // Default light gray
        }
    }

    fn draw_pixel(&mut self, x: u32, y: u32, color: [u8; 4]) {
        if x < WIDTH && y < HEIGHT {
            let index = ((y * WIDTH + x) * 4) as usize;
            if color != self.background_color {
                self.pixels[index..index + 4].copy_from_slice(&color);
            }
        }
    }
    fn save_as_png(&self, filename: &str) -> Result<(), image::ImageError> {
        let mut img = ImageBuffer::new(WIDTH, HEIGHT);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let index = ((y * WIDTH + x) * 4) as usize;
            *pixel = Rgba([
                self.pixels[index],
                self.pixels[index + 1],
                self.pixels[index + 2],
                self.pixels[index + 3],
            ]);
        }
        img.save(filename)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut content = String::new();
    let args: Vec<String> = env::args().collect();
    if (args.len() != 2) 
    {
        eprintln!("Usage: ./Baguette <File Path>");
        process::exit(1);
        return Ok(());
    }
    let filepath : &String = &args[1];
    if (Path::new(filepath).extension().unwrap() != "baguette" && Path::new(filepath).extension().unwrap() != "croissant") {
        eprintln!("Compiller error: Invalid file extension use .baguette or .croissant");
        process::exit(1);
    }

    let mut file = File::open(filepath)?;
    file.read_to_string(&mut content)?;
    let content_array: Vec<&str> = convert_string_into_array(content.as_str());
    
    let mut canvas = World::new();
    process_content(&mut canvas, &content_array);
    let _ = canvas.save_as_png("output.png");
    
    init_window(canvas)?;

    #[allow(unreachable_code)]
    Ok(())
}

fn convert_string_into_array(file_content: &str) -> Vec<&str> {
    file_content
    .lines()
    .flat_map(|line| {
        let mut words = line.split_whitespace().collect::<Vec<&str>>();
        words.push("\n");
        words
    })
    .collect()
}

fn process_content(canvas: &mut World, content_array: &[&str]) {
    let mut width = 40;
    let mut height = 40;

    for &word in content_array {
        let mut contains = false;
        match word {
            "\n" => {
                height += 1;
                width = 40;
                contains = true;
            }
            "Croissant" | "croissant" => {
                if width < VIEWPORT_WIDTH + 40 && height < VIEWPORT_HEIGHT + 40 {
                    canvas.draw_pixel(width, height, [0, 0, 0, 255]);
                }
                width += 1;
                contains = true;
            }
            "Baguette" | "baguette" => {
                if width < VIEWPORT_WIDTH + 40 && height < VIEWPORT_HEIGHT + 40 {
                    canvas.draw_pixel(width, height, [179, 145, 103, 255]);
                }
                width += 1;
                contains = true;
            }
            _ => width += 1,
        }
        if word.contains("Patisserie(") {
            contains = true;
            if !word.contains(")")
            {
                let error = height - 39;
                eprintln!("Syntax Error in line: {error} no Space supported inside of the color specification of the Patisserie keyword");
                process::exit(1);  
            }
            let color_str = word.trim_start_matches("Patisserie(").trim_end_matches(")");
            let color_values: Vec<&str> = color_str.split(',').collect();
            if color_values.len() >= 3 {
                let r = color_values[0].trim().parse().unwrap_or(255);
                let g = color_values[1].trim().parse().unwrap_or(255);
                let b = color_values[2].trim().parse().unwrap_or(255);
                let a = if color_values.len() > 3 {
                    color_values[3].trim().parse().unwrap_or(255)
                } else {
                    255
                };
                canvas.background_color = [r, g, b, a];
                for pixel in canvas.pixels.chunks_mut(4) {
                    pixel.copy_from_slice(&canvas.background_color);
                }
            }
        }
        if word.contains("Boulangerie(") 
        {
            contains = true;
            if !word.contains(")")
            {
                let error = height - 39;
                eprintln!("Syntax Error in line: {error} no Space supported inside of the loop");
                process::exit(1);
            }
            let mut repeate : String = String::new();
            let mut givencolor : String = String::new();
            let mut iscolorgiven : i32 = 0;
            let mut i = 0;
            for c in word.chars() {
                if c.is_numeric() && iscolorgiven < 2{
                    repeate.push(c);
                }
                if iscolorgiven >= 1 
                {
                    if c.is_numeric() || c == ',' {
                        givencolor.push(c);
                    }
                }
                if c == '('{
                    iscolorgiven += 1;
                }
                i += 1;
                if i == 13 && !c.is_numeric(){
                    let error = height - 39;
                    eprintln!("Syntax Error in line: {error} Boulangerie(val,type) with color Boulangerie(val,type(rgbcolor))");
                    process::exit(1);   
                }
                
            }
            if word.contains("Croissant") || word.contains("croissant") || word.contains("Baguette") || word.contains("baguette")
            {
                for _i in 0..repeate.parse::<i32>().unwrap() 
                {
                    if width < VIEWPORT_WIDTH + 40 && height < VIEWPORT_HEIGHT + 40 {
                        if iscolorgiven > 1 
                        {
                            let mut split_string = givencolor.split(',').collect::<Vec<&str>>();
                            canvas.draw_pixel(width, height, [
                                split_string[0].parse().unwrap_or(255),
                                split_string[1].parse().unwrap_or(255),
                                split_string[2].parse().unwrap_or(255),
                                if split_string.len() > 3 {
                                    split_string[3].parse().unwrap_or(255)
                                } else {
                                    255
                                }
                            ]);
                        }
                        else if word.contains("roissant")
                        {
                            canvas.draw_pixel(width, height, [0, 0, 0, 255]);
                        }else{
                            canvas.draw_pixel(width, height, [179, 145, 103, 255]);
                        }
                    }
                    width += 1;
                    if width >= VIEWPORT_WIDTH + 40 {
                        let error = height - 39;
                        eprintln!("Linelimit out of bounce width");
                        process::exit(1);
                    }
                }
            } 
            else 
            {
                let error = height - 39;
                eprintln!("Syntax Error in line: {error}");
                process::exit(1);
            }
            width -= 1;
        }
        else if word.contains("(") && word.contains(")") && (word.contains("Croissant") || word.contains("croissant") || word.contains("Baguette") || word.contains("baguette"))
        {
            contains = true;
            let mut repeate : String = String::new();
            
            for c in word.chars() {
                if c.is_numeric() || c == ',' {
                    repeate.push(c);
                }
            }
            let mut split_string = repeate.split(',').collect::<Vec<&str>>();
            if width < VIEWPORT_WIDTH + 40 && height < VIEWPORT_HEIGHT + 40 {
                canvas.draw_pixel(width, height, [
                    split_string[0].parse().unwrap_or(255),
                    split_string[1].parse().unwrap_or(255),
                    split_string[2].parse().unwrap_or(255),
                    if split_string.len() > 3 {
                        split_string[3].parse().unwrap_or(255)
                    } else {
                        255
                    }
                ]);
            }
        }
        if contains == false {
            let error = height - 39;
            eprintln!("Syntax Error in line error 2: {error}");
            process::exit(1);
        }
        if width >= VIEWPORT_WIDTH + 40 {
            let error = height - 39;
            eprintln!("Linelimit out of bounce width");
            process::exit(1);
        }
        if height >= (VIEWPORT_HEIGHT + 40) {
            let error = height - 39;
            eprintln!("Linelimit out of bounce height");
            process::exit(1);
        }
    }
}

fn init_window(canvas: World) -> Result<(), pixels::Error> 
{
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Baguette")
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


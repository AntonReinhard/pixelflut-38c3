use std::net::TcpStream;
use std::io::*;
use std::fs::File;
use std::fs;
use gif::DecodeOptions;

extern crate tinyppm;

//const X_SIZE:i32 = 3840;
//const Y_SIZE:i32 = 1080;

struct RGB {
    r:u8,
    g:u8,
    b:u8,
}

fn get_image(filename: &String) {
    let ppm_image_result = tinyppm::ppm_loader::read_image_data(filename);
    let ppm_image = match ppm_image_result {
        Ok(image) => image,
        _ => panic!("unable to read specified image file!"),
    };

    let mut frame: Vec<Vec<RGB>> = vec![vec![RGB(0, 0, 0); image.width()]; image.height()];
    
    for px in image.pixels(){

}
    // `ppm_image` is now a struct containing image with, height and pixels 
}

fn pixel(
    stream:&mut TcpStream,
    x:u16,
    y:u16,
    px::RGB,
) -> std::io::Result<()> {
    let send_str = format!("PX {} {} {:02x}{:02x}{:02x}\n", x, y, px.r, px.g, px.b);
    //println!("{}", send_str);
    stream.write(send_str.as_bytes());
    Ok(())
}

fn show_picture(
    stream:&mut TcpStream,
    x_pos:u16,
    y_pos:u16,
    width:u16,
    height:u16,
    buf:Vec<Vec<RGB>>
) -> std::io::Result<()> {
    println!("width: {} height: {}", width, height);

    let mut y = 0;
    c += 1;
    for row in buf {
        let mut x = 0;
        for px in row {
            pixel(stream, x + x_pos, y + y_pos, px);
            x += 1;
        }
        y += 1;
    }
    Ok(())
}

fn stream_gif(
    stream:&mut TcpStream,
    x_pos:u16,
    y_pos:u16
) -> std::io::Result<()> {
    let mut decoder = DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = decoder.read_info(GIF).unwrap();

    let mut frames: Vec<Vec<u8>> = Vec::new();
    let (mut width, mut height) = (0, 0);

    loop {
        let frame_info = match decoder.next_frame_info().unwrap() {
            Some(info) => info,
            None => break,
        };

        width = frame_info.width;
        height = frame_info.height;

        let mut frame: Vec<u8> =
            vec![0; frame_info.width as usize * frame_info.height as usize * 4];

        decoder.read_into_buffer(&mut frame).unwrap();
        frames.push(frame);
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let paths = fs::read_dir("./bee").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    let mut stream = TcpStream::connect("wall.c3pixelflut.de:1337")?;

    stream_gif(
        &mut stream,
        3500,
        0
    );
    Ok(())
} // the stream is closed here

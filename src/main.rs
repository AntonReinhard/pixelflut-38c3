use std::net::TcpStream;
use std::io::*;
use std::fs::File;
use std::fs;

extern crate tinyppm;

//const X_SIZE:i32 = 3840;
//const Y_SIZE:i32 = 1080;

#[derive(Clone)] struct RGB {
    r:u8,
    g:u8,
    b:u8,
}

fn get_image(filename: &String) -> Vec<Vec<RGB>> {
    let ppm_image_result = tinyppm::ppm_loader::read_image_data(filename);
    let ppm_image = match ppm_image_result {
        Ok(image) => image,
        _ => panic!("unable to read specified image file!"),
    };

    let mut frame: Vec<Vec<RGB>> = vec![vec![RGB{r: 0, g: 0, b: 0}; ppm_image.width()]; ppm_image.height()];

    let mut x = 0;
    let mut y = 0;

    for px in ppm_image.pixels() {
        frame[y][x] = RGB{
            r: (*px >> 24u8) as u8,
            g: (*px >> 16u8 & 0xFF) as u8,
            b: (*px >> 8u8 & 0xFF) as u8,
        };
        x += 1;
        if x >= ppm_image.width() {
            y += 1;
            x = 0;
        }
    }
    
    frame
}

fn pixel(
    stream:&mut TcpStream,
    x:u16,
    y:u16,
    px:RGB
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

fn main() -> std::io::Result<()> {
    let paths = fs::read_dir("./bee").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    let mut stream = TcpStream::connect("wall.c3pixelflut.de:1337")?;

    let image = get_image(&String::from("./bee/bee_00001.ppm"));

    Ok(())
} // the stream is closed here

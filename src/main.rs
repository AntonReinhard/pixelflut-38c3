use std::net::TcpStream;
use std::io::*;

extern crate tinyppm;

const X_SIZE:i16 = 3840;
const Y_SIZE:i16 = 1080;

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
            r: (*px >> 16u8 & 0xFF) as u8,
            g: (*px >> 8u8 & 0xFF) as u8,
            b: (*px >> 0u8 & 0xFF) as u8,
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
    px:&RGB
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
    buf:&Vec<Vec<RGB>>,
    scale:u16,
) -> std::io::Result<()> {
    let mut y = 0;
    for row in buf {
        let mut x = 0;
        for px in row {
            for x_i in 0..scale {
                for y_i in 0..scale {
                    pixel(stream, x_pos + x*scale + x_i, y_pos + y*scale + y_i, px);
                }
            }
            x += 1;
        }
        y += 1;
    }
    Ok(())
}

fn flood_white(stream:&mut TcpStream) -> std::io::Result<()> {
    for x in 1..X_SIZE {
        for y in 1..Y_SIZE {
            pixel(stream, x as u16, y as u16, &RGB{r:255, g:255, b:255})?;
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("wall.c3pixelflut.de:1337")?;

    let mut frames : Vec<Vec<Vec<RGB>>> = Vec::new();
    for i in 1..13721 {
        let filename = format!("./bee/bee_{:05}.ppm", i);
        let image = get_image(&String::from(filename));
        frames.push(image);
    }

    //loop {
    //    flood_white(&mut stream);
    //}

    println!("printing");
    loop {
        for frame in &frames {
            for _ in 1..2 {
                show_picture(&mut stream, 700, 0, &frame, 2);
            }
        }
    }



    Ok(())
} // the stream is closed here

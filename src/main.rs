use image::{self, io::Reader, Rgb};

fn main() {
    let mut cin = "".to_string();
    println!("Image path: ");
    std::io::stdin().read_line(&mut cin).expect("failed to read line");
    let mut path = std::path::PathBuf::new();
    path.push(cin.trim());

    cin = "".to_string();
    println!("Blur radius: ");
    std::io::stdin().read_line(&mut cin).expect("failed to read line");
    let radius: i16 = cin.trim().parse().expect("invalid radius, enter a positive whole number");

    println!("the full path is {}", path.clone().canonicalize().unwrap().to_str().unwrap());

    let opened_image = Reader::open(path.clone())
        .expect("unable to open image")
        .decode()
        .expect("unable to parse image")
        .into_rgb8();

    let mut new_image = opened_image.clone();

    for y in 0..opened_image.height(){
        for x in 0..opened_image.width(){
            new_image.put_pixel(x, y,
                blurred_pixel(
                    vectorize(
                        x, y, radius, &opened_image 
                    )
                )
            );
        }
    }

    match new_image.save(path){
        Ok(_) => {},
        Err(e) => println!("failed to save image ({e})"),
    }
}

fn vectorize(x: u32, y: u32, radius: i16, matrix: &image::ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<Rgb<u8>>{
    let mut result: Vec<Rgb<u8>> = vec![];
    for x1 in (x as i16 - radius)..=(x as i16 + radius){
        for y1 in (y as i16 - radius)..=(y as i16 + radius){
            if x1 < 0 || y1 < 0 {
                continue
            }

            append_some(matrix.get_pixel_checked(x1 as u32, y1 as u32).copied(), &mut result);
        }
    }

    return result;
}

fn append_some(pixel: Option<Rgb<u8>>, pixels: &mut Vec<Rgb<u8>>){
    if pixel.is_some(){
        pixels.push(pixel.unwrap());
    }
}

fn blurred_pixel(pixels: Vec<Rgb<u8>>) -> Rgb<u8>{
    let pixel_count = pixels.len() as u64;
    let mut rgb: (u64, u64, u64) = (0, 0, 0);

    for pixel in pixels{
        let components = pixel.0;
        rgb.0 += *components.get(0).unwrap() as u64; 
        rgb.1 += *components.get(1).unwrap() as u64;
        rgb.2 += *components.get(2).unwrap() as u64;
    }

    rgb.0 = rgb.0 / pixel_count;
    rgb.1 = rgb.1 / pixel_count;
    rgb.2 = rgb.2 / pixel_count;

    let blured = Rgb([rgb.0 as u8, rgb.1 as u8, rgb.2 as u8]);
    return blured;
}

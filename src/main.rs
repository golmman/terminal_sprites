use image::GenericImageView;

pub const RESET: &'static str = "\x1b[0m";

fn main() {
    let img = image::open("tests/pics/pineapple.png").unwrap();

    println!("{:?}", img.dimensions());
    println!("{:?}", img.color());
    //println!("{:?}", img.get_pixel(10, 10).0);

    let (font_w, font_h) = get_font_size();

    let mut pic_width = 0;

    for y in 0..img.dimensions().0 {
        let mut width_actual = 0;

        for x in 0..img.dimensions().1 {
            let width_optimal = (x + 1) * font_h;

            let image::Rgba([r, g, b, ..]) = img.get_pixel(y, x);
            print!("{}", ansi_background_rgb(r, g, b));
            while width_actual < width_optimal {
                print!(" ");
                width_actual += font_w;
            }
        }
        println!("{}", RESET);
    }
    println!("{}", RESET);
}

fn ansi_background_rgb(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{};{};{}m", r, g, b)
}

fn get_font_size() -> (u32, u32) {
    (7, 17)
}

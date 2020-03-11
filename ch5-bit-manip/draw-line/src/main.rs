pub mod bitref;
pub mod screen;

pub use bitref::*;
pub use screen::Screen;

fn main() {
    let mut bytes = [0_u8; 126];
    let mut s = Screen::new(&mut bytes[..], 48);

    for y in 1..20 {
        s.draw_horizontal(19 - y, 28 + y / 2, y);
    }

    println!("{}", &s);
}

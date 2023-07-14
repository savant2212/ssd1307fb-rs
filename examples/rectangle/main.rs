extern crate ssd1307fd;

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    prelude::Point,
    prelude::Size,
    primitives::{
         PrimitiveStyle, Rectangle,
    },
};

fn main() {
    let mut display = ssd1307fd::Display::new("/dev/fb1").unwrap();
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);

    // Draw a filled square
    Rectangle::new(Point::new(0, 0), Size::new(16, 16))
        .into_styled(fill)
        .draw(&mut display)
        .unwrap();

    Rectangle::new(Point::new(112, 0), Size::new(16, 16))
        .into_styled(fill)
        .draw(&mut display)
        .unwrap();

    Rectangle::new(Point::new(0, 48), Size::new(16, 16))
        .into_styled(fill)
        .draw(&mut display)
        .unwrap();

    Rectangle::new(Point::new(112, 48), Size::new(16, 16))
        .into_styled(fill)
        .draw(&mut display)
        .unwrap();

    Rectangle::new(Point::new(56, 24), Size::new(16, 16))
        .into_styled(fill)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap()
}

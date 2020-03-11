use std::fmt;

use crate::bitref::{BitMut, BitRef};

#[derive(Debug)]
pub struct Screen<T>
where
    T: AsRef<[u8]>,
{
    pixels: T, // each bit is 1 for white, 0 for black.
    width_pixels: usize,
}

impl<T> Screen<T>
where
    T: AsRef<[u8]>,
{
    fn check_invariants(&self) {
        assert_eq!(self.width() % 8, 0);
        assert_eq!((self.pixels.as_ref().len() * 8) % self.width(), 0);
    }

    pub fn new(source: T, width_pixels: usize) -> Self {
        assert_eq!(width_pixels % 8, 0);
        let width_bytes = width_pixels / 8;
        assert_eq!(source.as_ref().len() % width_bytes, 0);

        Self {
            pixels: source,
            width_pixels,
        }
    }

    pub fn width(&self) -> usize {
        self.width_pixels
    }

    pub fn height(&self) -> usize {
        self.pixels.as_ref().len() * 8 / self.width_pixels
    }

    pub fn get(&self, x: usize, y: usize) -> Option<BitRef> {
        if x >= self.width() {
            return None;
        }
        let source_byte = self.pixels.as_ref().get(y * self.width_bytes() + x / 8)?;
        let bit_idx = x % 8;
        Some(unsafe { BitRef::new_unchecked(source_byte, bit_idx as u8) })
    }

    pub fn get_mut<'a>(&'a mut self, x: usize, y: usize) -> Option<BitMut<'a>>
    where
        T: AsMut<[u8]>,
    {
        if x >= self.width() {
            return None;
        }
        // Need to precompute for to satisfy borrow checker
        let width_bytes = self.width_bytes();

        let source_byte = self.pixels.as_mut().get_mut(y * width_bytes + x / 8)?;
        let bit_idx = x % 8;
        Some(unsafe { BitMut::new_unchecked(source_byte, bit_idx as u8) })
    }

    pub fn draw_horizontal(
        &mut self,
        x1: usize,
        x2: usize, // inclusive
        y: usize,
    ) where
        T: AsMut<[u8]>,
    {
        assert!(x1 < self.width());
        assert!(x2 < self.width());
        assert!(y < self.height());

        for x in x1..=x2 {
            *self.get_mut(x, y).unwrap() = true;
        }
    }

    fn width_bytes(&self) -> usize {
        self.width() / 8
    }
}

impl<T> fmt::Display for Screen<T>
where
    T: AsRef<[u8]>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Top of screen.
        write!(f, "+")?;
        for _ in 0..self.width() {
            write!(f, "-")?;
        }
        writeln!(f, "+")?;

        // Rows.
        for y in 0..self.height() {
            write!(f, "|")?;
            for x in 0..self.width() {
                write!(f, "{}", if *self.get(x, y).unwrap() { '#' } else { ' ' })?;
            }
            writeln!(f, "|")?;
        }

        // And the bottom.
        write!(f, "+")?;
        for _ in 0..self.width() {
            write!(f, "-")?;
        }
        write!(f, "+")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let s = Screen::new(vec![0_u8; 100], 32);
        s.check_invariants();
        assert_eq!(s.width(), 32);
        assert_eq!(s.height(), 25);
    }

    #[test]
    fn width_height() {
        let s = Screen::new(vec![0_u8; 6 * 24 / 8], 24);
        assert_eq!(s.width(), 24);
        assert_eq!(s.height(), 6);
    }

    #[test]
    fn blank_begin() {
        let s = Screen::new(vec![0_u8; 32 * 24 / 8], 32);
        for x in 0..32 {
            for y in 0..24 {
                assert_eq!(*s.get(x, y).unwrap(), false);
            }
        }
    }

    #[test]
    fn read_write() {
        let mut s = Screen::new(vec![0_u8; 3 * 16 / 8], 16);
        *s.get_mut(5, 2).unwrap() = true;
        for x in 0..16 {
            for y in 0..3 {
                assert_eq!(s.get(x, y).unwrap(), x == 5 && y == 2);
            }
        }
    }

    #[test]
    fn display() {
        let mut s = Screen::new(vec![0_u8; 8 * 8 / 8], 8);
        *s.get_mut(0, 0).unwrap() = true;
        *s.get_mut(1, 0).unwrap() = true;
        *s.get_mut(5, 0).unwrap() = true;
        *s.get_mut(7, 0).unwrap() = true;
        *s.get_mut(4, 6).unwrap() = true;
        let expected = "+--------+
|##   # #|
|        |
|        |
|        |
|        |
|        |
|    #   |
|        |
+--------+";
        assert_eq!(format!("{}", &s), expected);
        println!("{}", &s);
    }

    #[test]
    fn draw_horizontal_full() {
        let mut s = Screen::new(vec![0_u8; 10 * 24 / 8], 24);
        s.draw_horizontal(0, 23, 6);
        let expected = "+------------------------+
|                        |
|                        |
|                        |
|                        |
|                        |
|                        |
|########################|
|                        |
|                        |
|                        |
+------------------------+";
        assert_eq!(format!("{}", &s), expected);
    }

    #[test]
    fn draw_horizontal_left() {
        let mut s = Screen::new(vec![0_u8; 10 * 24 / 8], 24);
        s.draw_horizontal(0, 14, 6);
        let expected = "+------------------------+
|                        |
|                        |
|                        |
|                        |
|                        |
|                        |
|###############         |
|                        |
|                        |
|                        |
+------------------------+";
        assert_eq!(format!("{}", &s), expected);
    }

    #[test]
    fn draw_horizontal_right() {
        let mut s = Screen::new(vec![0_u8; 10 * 24 / 8], 24);
        s.draw_horizontal(20, 23, 6);
        let expected = "+------------------------+
|                        |
|                        |
|                        |
|                        |
|                        |
|                        |
|                    ####|
|                        |
|                        |
|                        |
+------------------------+";
        assert_eq!(format!("{}", &s), expected);
    }

    #[test]
    fn draw_horizontal_middle() {
        let mut s = Screen::new(vec![0_u8; 10 * 24 / 8], 24);
        s.draw_horizontal(10, 18, 6);
        let expected = "+------------------------+
|                        |
|                        |
|                        |
|                        |
|                        |
|                        |
|          #########     |
|                        |
|                        |
|                        |
+------------------------+";
        assert_eq!(format!("{}", &s), expected);
    }

    #[test]
    #[should_panic]
    fn bad_width() {
        Screen::new(vec![0_u8; 1 * 8 / 8], 1);
    }

    #[test]
    fn access_too_right() {
        let s = Screen::new(vec![0_u8; 16 * 16 / 8], 16);
        assert_eq!(s.get(16, 4), None);
    }

    #[test]
    fn access_too_down() {
        let s = Screen::new(vec![0_u8; 9 * 16 / 8], 16);
        assert_eq!(s.get(5, 9), None);
    }
}

use std::ops::{Add, Sub, Mul};

const EPSILON: f32 = 0.00001;

#[derive(Clone, Debug)]
pub struct Color {
  pub red: f32,
  pub green: f32,
  pub blue: f32
}

impl Color {
  pub fn new(r: f32, g: f32, b: f32) -> Color {
    Color { red: r, green: g, blue: b }
  }

  pub fn new_t(t: (f32, f32, f32)) -> Color {
    Color::new(t.0, t.1, t.2)
  }

  pub fn ppm(&self) -> String {
    format!("{} {} {}", ppm_value(&self.red), ppm_value(&self.green), ppm_value(&self.blue))
  }
}

fn ppm_value(val: &f32) -> isize {
  let v : isize = (val * 255.0).round() as isize;
  if v > 255 {255} else if v < 0 {0} else {v}
}
/*
fn ppm_len(str: String) -> [String] {
  if str.chars().count() > 70 {
    let str_it = str.chars();
    [str_it.take(69)]
  } else {

  }

}
*/


impl Add for Color {
  type Output = Color;

  fn add(self, rhs: Color) -> Color {
    Color::new(self.red + rhs.red, self.green + rhs.green, self.blue + rhs.blue)
  }
}

impl Sub for Color {
  type Output = Color;

  fn sub(self, rhs: Color) -> Color {
    Color::new(self.red - rhs.red, self.green - rhs.green, self.blue - rhs.blue)
  }
}

impl Mul<f32> for Color {
  type Output = Color;

  fn mul(self, rhs: f32) -> Color {
    Color::new(self.red * rhs, self.green * rhs, self.blue * rhs)
  }
}

impl Mul for Color {
  type Output = Color;
  fn mul(self, rhs: Color) -> Color {
    Color::new(self.red * rhs.red, self.green * rhs.green, self.blue * rhs.blue)
  }
}

impl PartialEq for Color {
  fn eq(&self, other: &Color) -> bool {
      let diff = self.clone() - other.clone();
      diff.red.abs() < EPSILON && diff.green.abs() < EPSILON && diff.blue.abs() < EPSILON
  }
}
impl Eq for Color {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn access_color_component() {
    let c = Color::new(-0.5, 0.4, 1.7);
    assert_eq!(c.red, -0.5);
    assert_eq!(c.green, 0.4);
    assert_eq!(c.blue, 1.7);
  }

  #[test]
  fn add_colors() {
    let res = Color::new(1.6, 0.7, 1.0);
    let c = test_colors().0 + test_colors().1;
    assert_eq!(c, res);
  }

  #[test]
  fn sub_colors() {
    assert_eq!(test_colors().0 - test_colors().1, Color::new(0.2, 0.5, 0.5));
  }

  #[test]
  fn mul_scalar() {
    assert_eq!(Color::new(0.2,0.3,0.4)*2.0, Color::new(0.4,0.6,0.8));
  }

  #[test]
  fn mul_colors() {
    assert_eq!(test_colors().0 * test_colors().1, Color::new(0.63, 0.06, 0.1875));
  }



  fn test_colors() -> (Color, Color) {
    (Color::new(0.9, 0.6, 0.75),
     Color::new(0.7, 0.1, 0.25))
  }
}
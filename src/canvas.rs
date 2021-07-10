//use crate::ray_tuple::RayTuple;
use crate::color::Color;
use std::mem;
use substring::Substring;

const MAX_COLOR_VALUE: isize = 255;
//const MIN_COLOR_VALUE: isize = 0;
const BLACK: (f32, f32, f32) = (0.0, 0.0, 0.0);

#[derive(Clone, Debug)]
pub struct Canvas {
  height: usize,
  width: usize,
  rows: Vec<CanvasRow>
}

#[derive(Clone, Debug)]
struct CanvasRow {
  pixels: Vec<Color>
}

impl Canvas {
  pub fn new(width: usize, height: usize) -> Canvas {
    Canvas { height: height, width: width, rows: vec![CanvasRow::new(width, Color::new_t(BLACK)); height]}
  }

  pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> Color {
    let row = &mut self.rows[y];
    mem::replace(&mut row.pixels[x], color)
  }

  pub fn canvas_to_ppm(&self) -> String {
    let header = format!("P3\n{} {}\n{}", &self.width, &self.height, MAX_COLOR_VALUE);
    let pixel_strs: &Vec<String> = &self.rows.iter().map(|r| r.ppm()).collect();

    let pixels = pixel_strs.join("\n");
    format!("{}\n{}\n", header, pixels) 
  }
  
}

impl CanvasRow {
  pub fn new(width: usize, color: Color) -> CanvasRow {
    CanvasRow { pixels: vec![color; width] }
  }

  pub fn ppm(&self) -> String {
    let strs : &Vec<String> = &self.pixels.iter().map(|p| p.ppm()).collect();
    ppm_join(strs, 69)
  }

}

fn ppm_join(strs: &Vec<String>, max: usize) -> String {
  let lines = ppm_split(&strs.join(" "), max);
  if lines[1].trim().is_empty(){
    format!("{}", lines[0])
  } else {
    format!("{}\n{}", lines[0], lines[1])
  }
}


fn ppm_split(str: &str, max: usize) -> [String; 2] {
  if str.chars().count() > max {
      let split = ppm_space_index(str, max);
      [str.substring(0,split).to_string(), str.substring(split+1, str.len()).to_string()]
  } else {
    [str.to_string(), String::new()]
  }
}

fn ppm_space_index(str: &str, max: usize) -> usize {
  let mut split = max;
  for i in (0..max+1).rev() {
    if str.substring(i,i+1) == " " {
      break;
    }
    split = i-1; //-1 to remove the space
  } 
  split
}

#[test]
fn test_ppm_space_index() {
  let str = "123 456 789\n";
  assert_eq!(ppm_space_index(str, 6), 3);
}

#[test]
fn test_ppm_split() {
  let str = "123 456 789";
  assert_eq!(str.substring(3,4), " ");
  assert_eq!(ppm_split(str, 6), ["123", "456 789"]);
}

#[test]
fn test_ppm_join_long() {
  let str = vec!(String::from("123"), String::from("456"), String::from("789"));
  assert_eq!(ppm_join(&str, 6), String::from("123\n456 789"));
}

#[test]
fn test_ppm_split_short() {
  let str = "123 456 789";
  assert_eq!(ppm_split(str, 20), ["123 456 789", ""]);
}

#[test]
fn create_a_canvas() {
  let c = Canvas::new(10,20);
  assert_eq!(c.width, 10);
  assert_eq!(c.height, 20);
  for row in c.rows {
    for pixel in row.pixels {
      assert_eq!(pixel, Color::new_t(BLACK))
    }
  }
}

#[test]
fn write_pixel_to_canvas() {
  let red = Color::new(1.0, 0.0, 0.0);
  let mut c = Canvas::new(10, 20);
  c.write_pixel(2, 3, red);
  assert_eq!(c.rows[3].pixels[2], Color::new(1.0, 0.0, 0.0))
}

#[test]
fn ppm_header() {
  let c = Canvas::new(5, 3);
  let ppm = c.canvas_to_ppm();
  assert_eq!(ppm.lines().nth(0).unwrap(), "P3");
  assert_eq!(ppm.lines().nth(1).unwrap(), "5 3");
  assert_eq!(ppm.lines().nth(2).unwrap(), "255");
}

#[test]
fn ppm_pixels() {
  let mut canvas = Canvas::new(5,3);
  let c1 = Color::new(1.5,0.0,0.0);
  let c2 = Color::new(0.0,0.5,0.0);
  let c3 = Color::new(-0.5,0.0,1.0);
  canvas.write_pixel(0, 0, c1);
  canvas.write_pixel(2, 1, c2);
  canvas.write_pixel(4, 2, c3);
  let ppm = canvas.canvas_to_ppm();
  assert_eq!(ppm.lines().nth(3).unwrap(), "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
  assert_eq!(ppm.lines().nth(4).unwrap(), "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
  assert_eq!(ppm.lines().nth(5).unwrap(), "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
}

#[test]
fn split_ppm_lines() {
  let mut canvas = Canvas::new(10,20);
  for i in 0..20 {
    for j in 0..10 {
      canvas.write_pixel(j as usize, i as usize, Color::new(1.0, 0.8, 0.6));
    }
  }
  let ppm = canvas.canvas_to_ppm();
  assert_eq!(ppm.lines().nth(3).unwrap(), "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
  assert_eq!(ppm.lines().nth(4).unwrap(), "153 255 204 153 255 204 153 255 204 153 255 204 153");
  assert_eq!(ppm.lines().nth(5).unwrap(), "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
  assert_eq!(ppm.lines().nth(6).unwrap(), "153 255 204 153 255 204 153 255 204 153 255 204 153");

}

#[test]
fn ppm_trailing_newline() {
  let canvas = Canvas::new(5,3);
  let ppm = canvas.canvas_to_ppm();
  assert_eq!(ppm.chars().last().unwrap(), '\n');
}
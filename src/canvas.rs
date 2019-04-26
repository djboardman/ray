use crate::ray_tuple::RayTuple;
const MAX_COLOR_VALUE: isize = 255;
const MIN_COLOR_VALUE: isize = 0;

#[derive(Clone, Debug)]
pub struct Canvas {
    height: usize,
    width: usize,
    pixels: Vec<Vec<RayTuple>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let black = RayTuple::color(0.0, 0.0, 0.0);
        let pixels = vec![vec![black; width]; height];
        Canvas {height: height, width: width, pixels: pixels}
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: RayTuple) {
        let row = &mut self.pixels[y];
        let slice = &[color];
        row.splice(x..x+1, slice.iter().cloned());
    }
    
    pub fn pixel_at(&self, x: usize, y: usize) -> RayTuple {
        self.pixels[y][x].clone()
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);
        let mut line: String = "".to_string();
        let mut px: String;
        for row in &self.pixels {
            for pixel in row {
                /*
                s = format!("{}{} ", s, convert_color_value(pixel.x));
                s = format!("{}{} ", s, convert_color_value(pixel.y));
                s = format!("{}{} ", s, convert_color_value(pixel.z));
                */
                px =  pixel.scaled(MIN_COLOR_VALUE, MAX_COLOR_VALUE);
                if px.len() + line.len() > 70 {
                    ppm = format!("{}{}\n", ppm,  line.trim());
                    line = format!("{} ", px);
                } else {
                    line = format!("{}{} ", line, px);
                }
            }
            ppm = format!("{}{}\n", ppm, line.trim());
            line = "".to_string();
        }
        return ppm.to_string();
    }
}

#[test]
fn create_a_canvas() {
    let c = Canvas::new(10, 20);
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
    for mut row in c.pixels {
        for pixel in row.iter_mut() {
            assert_eq!(*pixel, RayTuple::color(0.0, 0.0, 0.0));
        }
    }
}

#[test]
fn write_pixel_to_canvas() {
    let mut c = Canvas::new(10, 20);
    let red = RayTuple::color(1.0, 0.0, 0.0);
    c.write_pixel(2, 3, red.clone());
    assert_eq!(c.pixel_at(2, 3), red);
    assert_eq!(c.pixel_at(5, 5), RayTuple::color(0.0, 0.0, 0.0));
}

#[test]
fn ppm_header() {
    let c = Canvas::new(5, 3);
    let ppm = c.to_ppm();
    assert_eq!(ppm.lines().nth(0).unwrap(), "P3");
    assert_eq!(ppm.lines().nth(1).unwrap(), "5 3");
    assert_eq!(ppm.lines().nth(2).unwrap(), "255");
}

#[test]
fn ppm_pixel_data() {
    let mut c = Canvas::new(5, 3);
    let c1 = RayTuple::color(1.5, 0.0, 0.0);
    let c2 = RayTuple::color(0.0, 0.5, 0.0);
    let c3 = RayTuple::color(-0.5, 0.0, 1.0);
    c.write_pixel(0, 0, c1);
    c.write_pixel(2, 1, c2);
    c.write_pixel(4, 2, c3);
    let ppm = c.to_ppm();
    assert_eq!(ppm.lines().nth(3).unwrap(), "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
    assert_eq!(ppm.lines().nth(4).unwrap(), "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
    assert_eq!(ppm.lines().nth(5).unwrap(), "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
}

#[test]
fn long_ppm_lines() {
    let mut c = Canvas::new(10, 2);
    let c1 = RayTuple::color(1.0, 0.8, 0.6);
    for y in 0..c.height {
        for x in 0..c.width {
            c.write_pixel(x, y, c1.clone());
        }
    }
    let ppm = c.to_ppm();
    assert_eq!(ppm.lines().nth(3).unwrap(), "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
    assert_eq!(ppm.lines().nth(4).unwrap(), "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
    assert_eq!(ppm.lines().nth(5).unwrap(), "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
    assert_eq!(ppm.lines().nth(6).unwrap(), "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
}

#[test]
fn ppm_terminated() {
    let c = Canvas::new(5, 3);
    let ppm = c.to_ppm();
    assert_eq!(ppm.chars().last().unwrap().to_string(), "\n");
}

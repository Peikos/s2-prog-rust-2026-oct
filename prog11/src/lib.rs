use std::ops::Add;

pub fn module_test() {
    println!("Hello from the root module, lib!")
}

#[derive(Clone, Debug)]
pub struct Point {
    x: f64,
    y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {

        Point { y: y, x}
    }
    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()  // return is optioneel; zonder semicolon
    }                                               //   is de laatste expressie de return waarde.

    pub fn get_x(&self) -> f64 {
        self.x
    }
    pub fn get_y(&self) -> f64 {
        self.y
    }
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f64) {
        if y > 0. {
            self.y = y;
        }
    }
}

pub fn foo(p: &mut Point) -> &mut f64 {
    p.x = 0.0;
    return &mut p.y
}
pub fn bar(y: &mut f64) -> &mut f64 {
    *y *= 2.0;                            // Pas de geleende waarde aan, niet de verwijzing zelf, met *
    y
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
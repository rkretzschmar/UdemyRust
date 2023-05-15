pub trait Area {
    fn area(&self) -> f32;
}

pub trait Circumference {
    fn circumference(&self) -> f32;
}

pub trait GeoProperties: Area + Circumference {}

#[derive(Debug)]
pub struct Circle {
    radius: f32,
}

impl GeoProperties for Circle {}

impl Area for Circle {
    fn area(&self) -> f32 {
        return self.radius * self.radius * 3.14159;
    }
}

impl Circumference for Circle {
    fn circumference(&self) -> f32 {
        return 2.0 * self.radius * 3.14159;
    }
}

#[derive(Debug)]
pub struct Square {
    length: f32,
}

impl GeoProperties for Square {}

impl Area for Square {
    fn area(&self) -> f32 {
        return self.length * self.length;
    }
}

impl Circumference for Square {
    fn circumference(&self) -> f32 {
        return 4.0 * self.length;
    }
}

pub struct GeometricFormsList {
    forms: Vec<Box<dyn Area>>,
}

impl GeometricFormsList {
    fn run(&self) {
        for form in self.forms.iter() {
            println!("Area: {}", form.area());
        }
    }
}

fn main() {
    let circle = Box::new(Circle { radius: 1.0 });
    let square = Box::new(Square { length: 2.0 });

    let forms = GeometricFormsList {
        forms: vec![circle, square],
    };

    forms.run();
}

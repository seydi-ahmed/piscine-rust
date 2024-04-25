

#[derive(Debug)]
pub struct Point {
    pub x : f64,
    pub y : f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point{
        let p1 : Point = Point{
            x : x,
            y : y
        };

        p1
    }

    pub fn distance(&self, p1: &Point) -> f64{
        ((self.x - p1.x).powf(2.0) + (self.y - p1.y).powf(2.0)).sqrt()
    }
}

#[derive(Debug)]
pub struct Circle {
	pub center : Point,
	pub radius : f64,
}

impl Circle {

    pub fn intersect(&self, cercle2: &Circle) -> bool{
        let somme_des_rayons : f64 = self.radius + cercle2.radius;
        let distance_des_centres : f64 = self.center.distance(&cercle2.center);
        if distance_des_centres <= somme_des_rayons{
            return true;
        }
        false
    }

    pub fn new(x_bi: f64, y_bi: f64, radius_bi: f64) -> Self{
        let center_bi : Point = Point::new(x_bi, y_bi);
        Circle { center: center_bi, radius: (radius_bi) }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}


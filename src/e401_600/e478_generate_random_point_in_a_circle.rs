struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

impl Solution {

    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self { radius, x_center, y_center }
    }

    fn rand_point(&self) -> Vec<f64> {
        loop {
            let x = rand::random_range(-self.radius..=self.radius);
            let y = rand::random_range(-self.radius..=self.radius);
            if x*x+y*y <= self.radius*self.radius {
                return vec![x+self.x_center, y+self.y_center]
            }
        }
    }
}

pub fn main() {}

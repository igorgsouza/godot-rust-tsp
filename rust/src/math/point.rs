#[derive(Default)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn calculate_distance(&self, point: &Point) -> Result<f64, String> {
        Ok((((self.x - point.x).pow(2) + (self.y - point.y).pow(2)) as f64).sqrt())
    }
}

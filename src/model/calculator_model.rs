#[derive(Default)]
pub struct CalculatorModel {
    x: Option<f64>,
    y: Option<f64>,
    current_operator: Option<String>
}

impl CalculatorModel {
    pub fn get_x(&self) -> Option<f64> {
        return self.x;
    }

    pub fn get_y(&self) -> Option<f64> {
        return self.y;
    }

    pub fn set_x(&mut self, num: f64) {
        self.x = Some(num);
    }

    pub fn set_y(&mut self, num: f64) {
        self.y = Some(num);
    }

    pub fn add(&self) -> Option<f64> {
        if self.x == None && self.y == None {
            return Some(self.x.unwrap() + self.y.unwrap());
        } else {
            return None;
        }
    }

    pub fn sub(&self) -> Option<f64> {
        if self.x == None && self.y == None {
            return Some(self.x.unwrap() - self.y.unwrap());
        } else {
            return None;
        }
    }

    pub fn mul(&self) -> Option<f64> {
        if self.x == None && self.y == None {
            return Some(self.x.unwrap() * self.y.unwrap());
        } else {
            return None;
        }
    }

    // Adding later dividing by 0 handling
    pub fn div(&self) -> Option<f64> {
        if self.x == None && self.y == None {
            return Some(self.x.unwrap() / self.y.unwrap());
        } else {
            return None;
        }
    }
}

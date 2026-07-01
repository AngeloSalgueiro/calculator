#[derive(Debug)]
pub struct CalculatorModel{
    x: f64,
    y: f64,
}

impl CalculatorModel{
    fn set_x(mut self, num: f64){
        self.x = num;
    }

    fn set_y(mut self, num: f64){
        self.y = num;
    }

    fn add(self) -> f64{
        return self.x + self.y;
    }

    fn sub(self) -> f64{
        return self.x - self.y;
    }

    fn mul(self) -> f64{
        return self.x * self.y;
    }

    // Adding later dividing by 0 handling
    fn div(self) -> f64{
        return self.x / self.y;
    }
}
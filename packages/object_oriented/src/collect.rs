pub struct AveragedCollection {
    average: f64,
    list: Vec<u64>,
}
impl AveragedCollection {
    pub fn new() -> Self {
        Self {
            average: 0.0,
            list: vec![],
        }
    }
    pub fn add(&mut self, value: u64) {
        self.list.push(value);
        self.calculate();
    }
    pub fn remove(&mut self) -> Option<u64> {
        let res = self.list.pop();
        self.calculate();
        match res {
            None => None,
            Some(value) => Some(value),
        }
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    fn calculate(&mut self) -> f64 {
        let sum: u64 = self.list.iter().sum();
        self.average = sum as f64 / self.list.len() as f64;
        self.average()
    }
}

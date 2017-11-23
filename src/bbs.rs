use std::fmt::{
    Display,
    Formatter,
    Result as FMTResult,
};

use string_builder::StringBuilder;

#[derive(Debug, PartialEq, Clone)]
pub struct BBS {
    pub p: f64,
    pub q: f64,
    pub x: f64,
    pub m: f64,
}

impl BBS {
    pub fn new(p: f64, q: f64, x0: f64) -> Option<Self> {
        if ((x0 % p) == 0.0)
            || ((x0 % q) == 0.0)
            || ((p % 4.0) != 3.0)
            || ((q % 4.0) != 3.0)
            {
                return None;
            }
        let m = p * q;
        Some(Self { p, q, x: x0 % m, m })
    }
    pub fn next(&mut self) -> f64 {
        self.x = (self.x * self.x) % self.m;
        self.x / self.m
    }
    pub fn show(&self, count: usize) -> String {
        let mut builder = StringBuilder::with_capacity(count + 2);
        builder.push(format!("{} (", self));
        for i in self.clone().take(count) {
            builder.push(format!("  {}\n", i));
        }
        builder.push(")");
        return builder.to_string();
    }
}

impl Iterator for BBS {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        Some(BBS::next(self))
    }
}

impl Display for BBS {
    fn fmt(&self, formatter: &mut Formatter) -> FMTResult {
        write!(formatter, "BBS {{p: {}, q: {}, x: {}}}", self.p, self.q, self.x)
    }
}

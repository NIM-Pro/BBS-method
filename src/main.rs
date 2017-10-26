#![feature(try_from)]

use std::f64::NEG_INFINITY;
use std::fmt::{
    Display,
    Formatter,
    Result as FMTResult,
};

#[derive(Debug, PartialEq, Clone)]
struct BBS {
    p: u32,
    q: u32,
    x: u32,
    m: u32,
}

impl BBS {
    fn new(p: u32, q: u32, x0: u32) -> Option<Self> {
        if ((x0 % p) == 0)
            || ((x0 % q) == 0)
            || ((p % 4) != 3)
            || ((q % 4) != 3)
        {
            return None;
        }
        let m = p*q;
        Some(Self {
            p: p,
            q: q,
            x: x0 % m,
            m: m,
        })
    }
    fn next(&mut self) -> u32 {
        self.x = self.x.wrapping_mul(self.x) % self.m;
        self.x
    }
}

impl Iterator for BBS {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(BBS::next(self))
    }
}

impl Display for BBS {
    fn fmt(&self, formatter: &mut Formatter) -> FMTResult {
        let mut r = writeln!(formatter, "BBS {{p: {}, q: {}, x: {}}} (", self.p, self.q, self.x);
        if r.is_err() { return r; }
        for i in self.clone().take(16) {
            r = writeln!(formatter, "  {}", i);
            if r.is_err() { return r; }
        }
        writeln!(formatter, ")")
    }
}

#[derive(Debug, PartialEq)]
struct StatChar {
    pub m: f64,
    pub d: f64,
}

impl StatChar {
    fn calc<T>(gen: &mut T, count: usize) -> Self
    where T: Iterator<Item = u32> + Clone {
        let _gen = gen.clone();
        let mut sum_m = 0u32;
        for i in _gen.take(count) {
            sum_m += i;
        }
        use std::convert::TryFrom;
        let f_count = f64::from(
            u32::try_from(count)
                .expect("Can't convert count of numbers into double float type")
        );
        let m = f64::from(sum_m) / f_count;
        let mut sum_d = 0f64;
        let mut max = NEG_INFINITY;
        for i in gen.take(count) {
            let e = f64::from(i);
            if e > max { max = e; }
            let s = e - m;
            sum_d += s * s;
        }
        let d = sum_d / (f_count * max * max);
        Self {
            m: m,
            d: d,
        }
    }
}

impl Display for StatChar {
    fn fmt(&self, formatter: &mut Formatter) -> FMTResult {
        let mut r = writeln!(formatter, "Statistics:");
        if r.is_err() { return r; }
        r = writeln!(formatter, "  Math expecting: {}", self.m);
        if r.is_err() { return r; }
        writeln!(formatter, "  Disperce: {}", self.d)
    }
}

fn main() {
    let mut gen = BBS::new(107, 163, 199)
        .expect("Numbers is incorrect for this generator type!");
    print!("{}", gen);
    print!("{}", StatChar::calc(&mut gen, 100));
}

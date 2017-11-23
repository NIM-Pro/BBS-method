use std::ops::Index;
use std::ops::Deref;

pub type Element = u32;
pub const LAYERS: usize = 32;
pub const LAYERS_I: Element = 32;

#[derive(Debug, PartialEq, Clone)]
pub struct PrimeGenerator {
    numbers: Vec<Element>,
    k: Element,
}

impl PrimeGenerator {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            numbers: vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29],
            k: 0,
        }
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.numbers.len()
    }
    #[allow(dead_code)]
    pub fn get(&self, index: usize) -> Prime {
        Prime {
            generator: self,
            index,
        }
    }
    #[allow(dead_code)]
    #[inline]
    fn generate(&self) -> Vec<Element> {
        let mut data = Vec::with_capacity(LAYERS * 8);
        for i in 0..LAYERS_I {
            let k = self.k + i;
            data.push(30 * k + 1);
            data.push(30 * k + 7);
            data.push(30 * k + 11);
            data.push(30 * k + 13);
            data.push(30 * k + 17);
            data.push(30 * k + 19);
            data.push(30 * k + 23);
            data.push(30 * k + 29);
        }
        data
    }
    #[allow(dead_code)]
    #[inline]
    fn inc_k(&mut self) {
        self.k += LAYERS_I;
    }
    #[inline]
    fn find_pos(num: u32, data: &[Element]) -> Option<usize> {
        if data.len() == 0 { return None; }
        match data.first() {
            Some(elem) => {
                let elem = *elem;
                if elem == num { return Some(0); }
                if elem > num { return None; }
            }
            None => return None,
        };
        match data.last() {
            Some(elem) => {
                let elem = *elem;
                if elem == num { return Some(data.len() - 1); }
                if elem < num { return None; }
            }
            None => return None,
        };
        let m = data.len() - 1;
        for j in 1..m {
            let i = m - j;
            if num == data[i] {
                return Some(i);
            }
        }
        None
    }
    #[allow(dead_code)]
    fn none_numbers(prime: u32, data: &mut Vec<Element>) {
        let mut target = prime * prime;
        let mut position = 0;
        loop {
            match PrimeGenerator::find_pos(target, &data[position..]) {
                Some(pos) => {
                    data.remove(pos);
                    target += prime;
                    position += pos;
                }
                None => return,
            }
        }
    }
    #[allow(dead_code)]
    pub fn more(&mut self) {
        let mut data = self.generate();
        self.inc_k();
        for num in self.numbers.iter() {
            PrimeGenerator::none_numbers(*num, &mut data);
        }
        'filter: loop {
            let prime = match data.first() {
                Some(prime) => *prime,
                None => break 'filter,
            };
            data.remove(0);
            PrimeGenerator::none_numbers(prime, &mut data);
        }
        self.numbers.append(&mut data);
    }
}

#[allow(dead_code)]
pub struct Prime<'a> {
    generator: &'a PrimeGenerator,
    index: usize,
}

impl<'a> Prime<'a> {
    fn get_value(&self) -> &Element {
        self.generator.numbers.index(self.index)
    }
    #[allow(dead_code)]
    fn mutate(&mut self) {
        unimplemented!()
    }
}

impl<'a> Deref for Prime<'a> {
    type Target = Element;
    fn deref(&self) -> &Self::Target {
        self.get_value()
    }
}

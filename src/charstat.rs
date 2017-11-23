use bbs::BBS;

#[derive(Debug, PartialEq, Clone)]
pub struct StatChar {
    m: f64,
    d: f64,
    max: f64,
    freq: f64,
}

impl StatChar {
    pub fn calc(gen: &BBS, count: usize) -> Self {
        use std::convert::TryFrom;
        let f_count = f64::from(
            u32::try_from(count)
                .expect("Can't convert count of numbers into double float type")
        );
        let m = {
            let gen = (*gen).clone();
            let mut sum = 0f64;
            for i in gen.take(count) {
                sum += i;
            }
            sum / f_count
        };
        let max = gen.m;
        let d = {
            let gen = (*gen).clone();
            let mut sum = 0f64;
            for i in gen.take(count) {
                let s = i - m;
                sum += s * s;
            }
            sum / f_count
        };
        let freq = {
            let gen = (*gen).clone();
            let delta = d.sqrt();
            let min = m - delta;
            let max = m + delta;
            let mut freq_count = 0f64;
            for i in gen.take(count) {
                if (i >= min) && (i <= max) {
                    freq_count += 1.0;
                }
            }
            freq_count / f_count
        };
        Self { m, d, max, freq }
    }
    pub fn show(&self) -> String {
        return format!(
            concat!(
                "Statistics:\n",
                "  Math expecting: {}\n",
                "  Disperse: {}\n",
                "  Maximum: {}\n",
                "  Frequency test result: {}\n",
            ),
            self.m,
            self.d,
            self.max,
            self.freq,
        );
    }
}

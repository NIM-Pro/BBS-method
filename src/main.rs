#![feature(try_from)]

mod bbs;
mod charstat;
mod string_builder;
mod prime_generator;

use bbs::BBS;
use charstat::StatChar;

fn main() {
    let gen = BBS::new(
        4398042316799.0,
        1125899839733759.0,
        18014398241046527.0,
    )
        .expect("Numbers is incorrect for this generator type!");
    println!("{}", gen.show(10));
    println!("{}", StatChar::calc(&gen, 1_000_000).show());
}

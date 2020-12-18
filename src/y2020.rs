use crate::Year;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
// mod d13;
// mod d14;
// mod d15;
// mod d16;
// mod d17;
// mod d18;
// mod d19;
// mod d20;
// mod d21;
// mod d22;
// mod d23;
// mod d24;
// mod d25;

pub fn year() -> Year {
    Year{
        year: 2020,
        day_parsers: vec![d01::parse, 
                          d02::parse,
                          d03::parse,
                          d04::parse,
                          d05::parse,
                          d06::parse,
                          d07::parse,
                          d08::parse,
                          d09::parse,
                          d10::parse,
                          d11::parse,
                          d12::parse,
                        //   d13::parse,
                        //   d14::parse,
                        //   d15::parse,
                        //   d16::parse,
                        //   d17::parse,
                        //   d18::parse,
                        //   d19::parse,
                        //   d20::parse,
                        //   d21::parse,
                        //   d22::parse,
                        //   d23::parse,
                        //   d24::parse,
                        //   d25::parse,
                         ]
    }
}

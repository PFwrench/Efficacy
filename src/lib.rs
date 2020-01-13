#[macro_use]
extern crate clap;
extern crate colored;
extern crate config;
extern crate itertools;
extern crate regex;
extern crate serde;
extern crate shellexpand;

pub mod cli;
pub mod program;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

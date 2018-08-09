
extern crate rand;

use rand::{ Rng };


#[derive(Debug)]
pub struct Die<usize> {
    faces: usize,
}

pub struct DiceBag<T: Rolleable> {
    dice: Vec<T>,
}

pub struct RollResult(Vec<usize>);

pub trait Rolleable {
    fn roll(&self) -> usize;
}

impl Rolleable for Die<usize> {
    fn roll(&self) -> usize {
        rand::thread_rng().gen_range(1, self.faces)
    }
}

impl DiceBag<Die<usize>> {
    pub fn roll(&self) -> RollResult {
        RollResult(self.dice.iter().map(|x| x.roll()).collect())
    }
}
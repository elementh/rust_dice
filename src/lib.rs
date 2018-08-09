
extern crate rand;

use rand::{ OsRng, Rng };
use std::error::Error;


#[derive(Debug)]
pub struct Die<usize> {
    faces: usize,
}

pub struct DiceBag<T: Rolleable> {
    dice: Vec<T>,
}

struct RollResult(Vec<usize>);

pub trait Rolleable {
    fn roll(&self) -> usize;
}

impl Rolleable for Die<usize> {
    fn roll(&self) -> usize {
        rand::thread_rng().gen_range(1, self.faces)
    }
}


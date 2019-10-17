//! Mutators
//! fuzz (1)
//!  - thread (n)
//!    - seed (n)
//!    - connection (n)
//!      - mutation (1)
//!
//!
use crate::mutators::bitflipper::BitFlipper;

pub mod bitflipper;

/// Single round of mutations
///
/// Each seed has a starting state, and it will repeatedly mutate from that starting
/// point and return a mutation that is ONE mutation off from that base. Note that we
/// don't just constantly mutate the same buffer or we'd very quickly turn it into a
/// complete mess; we want to find small novel changes to the input that produce a
/// different output.
///
pub struct Seed {
    seed: Vec<u8>,
    buffer: Vec<u8>,
    mutators: Vec<Box<dyn Mutator>>,
}

// todo I'm gonna have to benchmark this shit.
/// Creating a new Vec for every single mutation is going to be a whole ton of allocations.
/// I wonder if we should be maintaining a couple of fixed size buffers and swapping between
/// them or something? The clean seed buffer, then a dirty buffer that we allow our mutators
/// to futz with and a reset method that cleans it back up?
///
/// just tried the above, is WAY faster than doing anything with a vector. I think it's mutate
/// reset mutate reset, gonna just add that to the trait.
///
/// If I have 1000 connections I want to be dealing with 1000 buffers, not one for every call.
/// But I will have one mutation for every call... so that gets weird. I think we gotta have a
/// shared mutation buffer, but the memory ownership for that gets weird. But the network just
/// needs a &[u8] to work from, so that should be okay to pass around.
///
impl Seed {
    pub fn new(seed: Vec<u8>) -> Seed {
        Seed {
            seed,
            buffer: vec![],
            mutators: vec![],
        }
    }

    ///
    pub fn mutate() -> Vec<u8> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum Range {
    All,
    First(usize),
    Last(usize),
    Range(usize, usize),
}

pub trait Mutator {
    fn mutate(&mut self, bytes: &mut [u8]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_bits() {
        let mut bytes = [0u8; 8];
        let mut bf = BitFlipper::new();

        bf.mutate(&mut bytes);

        dbg!(bytes, bf);
    }
}

mod mt;
mod rng_wrapper;
mod sfmt;
mod tiny_mt;

pub use mt::*;
pub use rng_wrapper::*;
pub use sfmt::*;
pub use tiny_mt::*;

pub trait Rng: Default {
    type Seed: Eq + Copy + Default;
    type CurrentState: Eq + Copy;

    fn new(seed: Self::Seed) -> Self;
    fn next_state(&mut self) -> Self::CurrentState;
    fn current_state(&mut self) -> Self::CurrentState;
}

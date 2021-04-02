// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// For serializing
#[macro_use]
extern crate serde_derive;

mod coasts;
mod erosion;
mod noise;
mod poisson;
mod rivers;
pub mod terrain_generator;
mod utils;
mod voronoi;

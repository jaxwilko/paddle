//! A 2D geometry module
//!
//! It contains basic shapes such as rectangles and circles, as well as vectors, lines, and a
//! universal Shape API. It also has matrix-backed Transforms for arbitrary constant-time 2D
//! transformations, such as rotating, scaling, or translating.
//!
//! The Tilemap allows 2D storage of data in a world-like grid, and also moving objects at given
//! speeds around the map, which is highly useful for games like platformers.

mod circle;
mod objects;
mod shape;
mod tilemap;
mod util;
pub use self::{
    circle::Circle,
    objects::{Line, Triangle},
    shape::Shape,
    tilemap::{Tile, Tilemap},
    util::{about_equal, lerp, lerp_angle},
};

/// Module for dealing with colors.

/// A color represented as a 24-bit RGB value.
/// This is the representation that contains the most detail
/// and takes up the most space.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rgb24 {
    /// The red component of the color.
    pub r: u8,
    /// The green component of the color.
    pub g: u8,
    /// The blue component of the color.
    pub b: u8,
}

impl Rgb24 {
    /// Create a new `Rgb24` color.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

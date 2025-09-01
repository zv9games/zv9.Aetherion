// generator/pattern_type.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
    Checkerboard,
    Radial,
    XorFractal,
    VerticalStripes,
    HorizontalStripes,
}

pub fn blend_noise_and_pattern(noise: bool, pattern: bool) -> bool {
    noise && pattern // or customize this later
}

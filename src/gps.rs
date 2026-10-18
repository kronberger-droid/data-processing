//! Perimeter, area and centroid of a closed GPS track.
//!
//! Input is `Gpsies_austria.csv`: `#` comment lines, then whitespace
//! separated `latitude longitude elevation index`, angles in degrees. The
//! track is closed implicitly, the last point connects back to the first.

/// Mean earth radius in metres.
pub const EARTH_RADIUS: f64 = 6_371_000.0;

/// Angles in degrees, elevation in metres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
    pub elevation: f64,
}

pub fn parse(text: &str) -> Result<Vec<Point>, String> {
    todo!()
}

/// Length of the closed track along great circles, in metres.
pub fn perimeter(points: &[Point]) -> f64 {
    todo!()
}

/// Enclosed area in square metres.
pub fn area(points: &[Point]) -> f64 {
    todo!()
}

/// Centroid of the enclosed area. `elevation` is left at zero.
pub fn centroid(points: &[Point]) -> Point {
    todo!()
}

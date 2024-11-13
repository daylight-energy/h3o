use crate::{
    error::{InvalidGeometry, InvalidLatLng},
    geom::{PolyfillConfig, ToCells},
    CellIndex, LatLng,
};
use alloc::boxed::Box;

/// A single point in 2D space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point(geo::Point);

impl Point {
    /// Initialize a new point from a point whose coordinates are in radians.
    ///
    /// # Errors
    ///
    /// [`InvalidGeometry`] if the point is invalid (e.g. contains non-finite
    /// coordinates).
    ///
    /// # Example
    ///
    /// ```
    /// use h3o::geom::Point;
    ///
    /// let p: geo::Point = (-0.7527922512723104, -0.4009198312650009).into();
    /// let point = Point::from_radians(p)?;
    /// # Ok::<(), h3o::error::InvalidGeometry>(())
    /// ```
    pub fn from_radians(point: geo::Point) -> Result<Self, InvalidGeometry> {
        Self::check_coords(&point).map(|()| Self(point))
    }

    /// Initialize a new point from a point whose coordinates are in degrees.
    ///
    /// # Errors
    ///
    /// [`InvalidGeometry`] if the point is invalid (e.g. contains non-finite
    /// coordinates).
    ///
    /// # Example
    ///
    /// ```
    /// use h3o::geom::Point;
    ///
    /// let p: geo::Point = (-43.13181884805516, -22.97101425458166).into();
    /// let point = Point::from_degrees(p)?;
    /// # Ok::<(), h3o::error::InvalidGeometry>(())
    /// ```
    pub fn from_degrees(
        mut point: geo::Point,
    ) -> Result<Self, InvalidGeometry> {
        point.set_x(point.x().to_radians());
        point.set_y(point.y().to_radians());
        Self::from_radians(point)
    }

    // Check that the point's coordinates are finite.
    fn check_coords(point: &geo::Point) -> Result<(), InvalidGeometry> {
        if !super::coord_is_valid(point.0) {
            return Err(InvalidGeometry::new("x and y must be valid"));
        }
        Ok(())
    }
}

impl From<Point> for geo::Point {
    fn from(value: Point) -> Self {
        value.0
    }
}

impl TryFrom<Point> for LatLng {
    type Error = InvalidLatLng;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        Self::from_radians(value.0.y(), value.0.x())
    }
}

impl ToCells for Point {
    fn max_cells_count(&self, _config: PolyfillConfig) -> usize {
        1
    }

    fn to_cells(
        &self,
        config: PolyfillConfig,
    ) -> Box<dyn Iterator<Item = CellIndex> + '_> {
        let ll = LatLng::try_from(*self).expect("valid coordinate");
        Box::new(core::iter::once(ll.to_cell(config.resolution)))
    }
}

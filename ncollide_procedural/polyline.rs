use na;
use na::{Translate, Rotate, Transform};
use math::{Scalar, Point, Vect};

/// Shapeetric description of a polyline.
#[derive(Clone)]
pub struct Polyline<P: Point> {
    /// Coordinates of the polyline vertices.
    pub coords:  Vec<P>,
    /// Coordinates of the polyline normals.
    pub normals: Option<Vec<P::Vect>>,
}

impl<P: Point> Polyline<P> {
    /// Creates a new polyline.
    pub fn new(coords: Vec<P>, normals: Option<Vec<P::Vect>>) -> Polyline<P> {
        Polyline {
            coords:  coords,
            normals: normals,
        }
    }
}

impl<P: Point> Polyline<P> {
    /// Translates each vertex of this polyline.
    pub fn translate_by<T: Translate<P>>(&mut self, t: &T) {
        for c in self.coords.iter_mut() {
            *c = t.translate(c);
        }
    }

    /// Rotates each vertex and normal of this polyline.
    // XXX: we should use Rotate<P> instead of the .set_coord.
    // Wa cannot make it a Rotate because the `Rotate` bound cannot appear twice… we have, again,
    // to wait for the trait reform.
    pub fn rotate_by<R: Rotate<P::Vect>>(&mut self, r: &R) {
        for c in self.coords.iter_mut() {
            let rc = r.rotate(c.as_vec());
            c.set_coords(rc);
        }

        for n in self.normals.iter_mut() {
            for n in n.iter_mut() {
                *n = r.rotate(n);
            }
        }
    }

    /// Transforms each vertex and rotates each normal of this polyline.
    pub fn transform_by<T: Transform<P> + Rotate<P::Vect>>(&mut self, t: &T) {
        for c in self.coords.iter_mut() {
            *c = t.transform(c);
        }

        for n in self.normals.iter_mut() {
            for n in n.iter_mut() {
                *n = t.rotate(n);
            }
        }
    }

    /// Scales each vertex of this polyline.
    pub fn scale_by_scalar(&mut self, s: &<P::Vect as Vect>::Scalar) {
        for c in self.coords.iter_mut() {
            *c = *c * *s
        }
        // FIXME: do something for the normals?
    }
}

impl<P> Polyline<P>
    where P: Point {
    /// Scales each vertex of this mesh.
    #[inline]
    pub fn scale_by(&mut self, s: &P::Vect) {
        for c in self.coords.iter_mut() {
            for i in 0 .. na::dim::<P::Vect>() {
                c[i] = (*c)[i] * s[i];
            }
        }
        // FIXME: do something for the normals?
    }
}

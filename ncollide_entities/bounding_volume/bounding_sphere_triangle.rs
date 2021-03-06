use na::Transform;
use bounding_volume::{BoundingSphere, HasBoundingVolume};
use bounding_volume;
use shape::Triangle;
use math::Point;


impl<P, M> HasBoundingVolume<M, BoundingSphere<P>> for Triangle<P>
    where P: Point,
          M: Transform<P> {
    #[inline]
    fn bounding_volume(&self, m: &M) -> BoundingSphere<P> {
        let pts = [ self.a().clone(), self.b().clone(), self.c().clone() ];
        let (center, radius) = bounding_volume::point_cloud_bounding_sphere(&pts[..]);

        BoundingSphere::new(m.transform(&center), radius)
    }
}

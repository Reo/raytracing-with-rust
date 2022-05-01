use crate::hittable::Hittable;
use crate::hittable::HitNode;
use crate::ray::Ray;
use crate::vec::Vec3d;
use crate::vec::Point3d;

// implementing external traits on external types
// https://doc.rust-lang.org/stable/book/ch19-03-advanced-traits.html
// #using-the-newtype-pattern-to-implement-external-traits-on-external-types
// struct HittableList(Vec<Hittable>);

// HitList is just a list of objects which the ray may intersect with
// in this case, a Vec of pointers to hittables
impl Hittable for Vec<&dyn Hittable> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hitnode: &mut HitNode) -> bool {
        // initialise
        let mut hit_something : bool = false;
        let mut curr_closest_t : f64 = t_max;
        let mut curr_node : HitNode = HitNode {
            p: Point3d::zero(),
            n: Vec3d::zero(),
            t: -1.0,
            is_front: false
        };

        // for each hit candidate in the list, check for intersection
        // and update vals appropriately
        for hit_cand in self.iter() {
            if hit_cand.hit(ray, t_min, t_max, &mut curr_node) && curr_node.t < curr_closest_t {
                hit_something = true;
                curr_closest_t = curr_node.t;
                *hitnode = curr_node;
            }
        }
        return hit_something;
    }
}


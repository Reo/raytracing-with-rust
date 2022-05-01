use crate::hittable::Hittable;
use crate::hittable::HitNode;
use crate::ray::Ray;

pub use Vec<dyn Hittable> as HittableList;

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hitnode: &mut HitNode) -> bool {
        let hit_something : bool = false;
        let curr_closest_t : f64 = t_max;
        let curr_node : HitNode;

        for hit_cand in self.iter() {
            // DEBUG
            println!("hit object {:?}", hit_cand);
            if hit_cand.hit(ray, t_min, t_max, &curr_node) && curr_node.t < curr_closest_t {
                hit_something = true;
                curr_closest_t = curr_node.t;
                hitnode = curr_node;
            }
        }
        return hit_something;
    }
}


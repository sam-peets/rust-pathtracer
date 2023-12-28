use crate::tracer::Ray;
use crate::triangle::Triangle;
use crate::vec4::Vec4;
use std::cmp;

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Vec4,
    pub max: Vec4,
}

impl AABB {
    pub fn contains(&self, p: &Vec4) -> bool {
        let gtMin = self.min.x <= p.x && self.min.y <= p.y && self.min.z <= p.z;
        let ltMax = self.max.x >= p.x && self.max.y >= p.y && self.max.z >= p.z;

        return gtMin && ltMax;
    }

    pub fn intersect_ray(&self, r: &Ray) -> bool {
        // https://tavianator.com/2022/ray_box_boundary.html

        let mut tmin: f64 = 0.;
        let mut tmax: f64 = f64::INFINITY;

        for d in 0..3 {
            let t1: f64 = (self.min.elem(d) - r.origin.elem(d)) / r.dir.elem(d);
            let t2: f64 = (self.max.elem(d) - r.origin.elem(d)) / r.dir.elem(d);

            tmin = f64::max(tmin, f64::min(t1, t2));
            tmax = f64::min(tmax, f64::max(t1, t2));
        }

        return tmin < tmax;
    }

    pub fn intersect_triangle(&self, t: &Triangle) -> bool {
        // https://omnigoat.github.io/2015/03/09/box-triangle-intersection/
        if (!self.intersect_aabb(&t.aabb())) {
            return false;
        }

        let n = t.normal();

        let p = self.min;
        let dp = self.max - p;

        let c = Vec4 {
            x: if n.x > 0. { dp.x } else { 0. },
            y: if n.y > 0. { dp.y } else { 0. },
            z: if n.z > 0. { dp.z } else { 0. },
            w: 1.,
        };

        let d1 = n.dot(c - t.p0.v);
        let d2 = n.dot(dp - c - t.p0.v);

        if ((n.dot(p) + d1) * (n.dot(p) + d2)) > 0. {
            return false;
        }

        let edge0 = t.p1.v - t.p0.v;
        let edge1 = t.p2.v - t.p1.v;
        let edge2 = t.p0.v - t.p2.v;

        let xym = if n.z < 0. { -1. } else { 1. };
        let ne0xy = Vec4::new(-edge0.y, edge0.x, 0., 0.) * xym;
        let ne1xy = Vec4::new(-edge1.y, edge1.x, 0., 0.) * xym;
        let ne2xy = Vec4::new(-edge2.y, edge2.x, 0., 0.) * xym;

        let v0xy = Vec4::new(t.p0.v.x, t.p0.v.y, 0., 0.);
        let v1xy = Vec4::new(t.p1.v.x, t.p1.v.y, 0., 0.);
        let v2xy = Vec4::new(t.p2.v.x, t.p2.v.y, 0., 0.);

        let de0xy = -ne0xy.dot(v0xy) + f64::max(0., dp.x * ne0xy.x) + f64::max(0., dp.y * ne0xy.y);
        let de1xy = -ne1xy.dot(v1xy) + f64::max(0., dp.x * ne1xy.x) + f64::max(0., dp.y * ne1xy.y);
        let de2xy = -ne2xy.dot(v2xy) + f64::max(0., dp.x * ne2xy.x) + f64::max(0., dp.y * ne2xy.y);

        let pxy = Vec4::new(p.x, p.y, 0., 0.);

        if (ne0xy.dot(pxy) + de0xy) < 0.
            || (ne1xy.dot(pxy) + de1xy) < 0.
            || (ne2xy.dot(pxy) + de2xy) < 0.
        {
            return false;
        }

        let yzm = if n.x < 0. { -1. } else { 1. };
        let ne0yz = Vec4::new(-edge0.z, edge0.y, 0., 0.) * yzm;
        let ne1yz = Vec4::new(-edge1.z, edge1.y, 0., 0.) * yzm;
        let ne2yz = Vec4::new(-edge2.z, edge2.y, 0., 0.) * yzm;

        let v0yz = Vec4::new(t.p0.v.y, t.p0.v.z, 0., 0.);
        let v1yz = Vec4::new(t.p1.v.y, t.p1.v.z, 0., 0.);
        let v2yz = Vec4::new(t.p2.v.y, t.p2.v.z, 0., 0.);

        let de0yz = -ne0yz.dot(v0yz) + f64::max(0., dp.y * ne0yz.x) + f64::max(0., dp.z * ne0yz.y);
        let de1yz = -ne1yz.dot(v1yz) + f64::max(0., dp.y * ne1yz.x) + f64::max(0., dp.z * ne1yz.y);
        let de2yz = -ne2yz.dot(v2yz) + f64::max(0., dp.y * ne2yz.x) + f64::max(0., dp.z * ne2yz.y);

        let pyz = Vec4::new(p.y, p.z, 0., 0.);

        if (ne0yz.dot(pyz) + de0yz) < 0.
            || (ne1yz.dot(pyz) + de1yz) < 0.
            || (ne2yz.dot(pyz) + de2yz) < 0.
        {
            return false;
        }

        let zxm = if n.y < 0. { -1. } else { 1. };
        let ne0zx = Vec4::new(-edge0.x, edge0.z, 0., 0.) * zxm;
        let ne1zx = Vec4::new(-edge1.x, edge1.z, 0., 0.) * zxm;
        let ne2zx = Vec4::new(-edge2.x, edge2.z, 0., 0.) * zxm;

        let v0zx = Vec4::new(t.p0.v.z, t.p0.v.x, 0., 0.);
        let v1zx = Vec4::new(t.p1.v.z, t.p1.v.x, 0., 0.);
        let v2zx = Vec4::new(t.p2.v.z, t.p2.v.x, 0., 0.);

        //double check this line
        let de0zx = -ne0zx.dot(v0zx) + f64::max(0., dp.z * ne0zx.x) + f64::max(0., dp.x * ne0zx.y);
        let de1zx = -ne1zx.dot(v1zx) + f64::max(0., dp.z * ne1zx.x) + f64::max(0., dp.x * ne1zx.y);
        let de2zx = -ne2zx.dot(v2zx) + f64::max(0., dp.z * ne2zx.x) + f64::max(0., dp.x * ne2zx.y);

        let pzx = Vec4::new(p.z, p.x, 0., 0.);

        if (ne0zx.dot(pzx) + de0zx) < 0.
            || (ne1zx.dot(pzx) + de1zx) < 0.
            || (ne2zx.dot(pzx) + de2zx) < 0.
        {
            return false;
        }

        return true;
    }

    pub fn intersect_aabb(&self, bb: &AABB) -> bool {
        let tx = self.min.x <= bb.max.x && self.max.x >= bb.min.x;
        let ty = self.min.y <= bb.max.y && self.max.y >= bb.min.y;
        let tz = self.min.z <= bb.max.z && self.max.z >= bb.min.z;
        return tx && ty && tz;
    }
}

use crate::math::structs::{Field, Inf};
use std::cmp::Ordering::*;

use super::vec2::{ccw, Vec2};

#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct Polygon2<T> {
	pub a: Vec<Vec2<T>>,
}
impl<T> Polygon2<T>
where
	T: Field,
{
	pub fn area_doubled(&self) -> T {
		let mut ret = T::zero();
		for i in 1..self.a.len() - 1 {
			ret = ret + ccw(self.a[i - 1], self.a[i], self.a[i + 1])
		}
		ret
	}
}

pub enum PointOnPolygon2 {
	INSIDE,
	EDGE,
	OUTSIDE,
}
#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct PolygonConvex2<T> {
	a: Vec<Vec2<T>>,
}
impl<T> PolygonConvex2<T>
where
	T: Field + Inf,
{
	pub fn from_vertices(mut a: Vec<Vec2<T>>) -> Self {
		let base = a
			.iter()
			.fold(Vec2::inf(), |acc, x| match x.partial_cmp(&acc) {
				None => acc,
				Some(z) => match z {
					Less => *x,
					Equal | Greater => acc,
				},
			});
		if base == Vec2::<T>::inf() {
			PolygonConvex2 { a: Vec::new() }
		} else {
			a.sort_by(|l, r| {
				let det = ccw(base, *r, *l);
				// base should be left-bottom or bottom-left.
				// More precisely, It doesn't matter if base is on convex hull.
				// If it isn't, for some order a[0]<a[1]<...<a[n-1], a[n-1]<a[0] is also true.
				// To prevent it, degree of (a[0]-base) to (a[-1]-base) should be less than 180.
				assert!(
					base <= *l && base <= *r
						|| (base.y, base.x) <= (l.y, l.x) && (base.y, base.x) <= (r.y, r.x)
				);
				if det.is_zero() {
					(*l - base)
						.lensq()
						.partial_cmp(&(*r - base).lensq())
						.unwrap()
				} else {
					det.partial_cmp(&T::zero()).unwrap()
				}
			});
			let mut s = Vec::new();
			for i in a {
				while let (Some(y), Some(z)) = (
					s.get(s.len().wrapping_sub(2)),
					s.get(s.len().wrapping_sub(1)),
				) {
					if ccw(*y, *z, i) > T::zero() {
						break;
					}
					s.pop();
				}
				s.push(i);
			}
			PolygonConvex2 { a: s }
		}
	}
	pub fn get_hull(&self) -> Vec<Vec2<T>> {
		self.a.clone()
	}
	pub fn contains(v: Vec2<T>) -> PointOnPolygon2 {
		assert!(false);
		let _ = v;
		PointOnPolygon2::INSIDE
	}
}

use binary_space_partition::{BspNode, Plane, PlaneCut};
use euclid::TypedPoint3D;
use euclid::approxeq::ApproxEq;
use num_traits::{Float, One, Zero};
use std::{fmt, ops};
use {Intersection, Polygon, Splitter};


impl<T: Copy + fmt::Debug + PartialOrd + ApproxEq<T> +
        ops::Sub<T, Output=T> + ops::Add<T, Output=T> +
        ops::Mul<T, Output=T> + ops::Div<T, Output=T> +
        Zero + One + Float,
     U> Plane for Polygon<T, U> {

    fn cut(&self, mut plane: Self) -> PlaneCut<Self> {
        let dist = self.signed_distance_sum_to(&plane);
        match self.intersect(&plane) {
            Intersection::Coplanar if dist.approx_eq(&T::zero()) => {
                PlaneCut::Sibling(plane)
            }
            Intersection::Coplanar | Intersection::Outside => {
                if dist > T::zero() {
                    PlaneCut::Cut {
                        front: vec![plane],
                        back: vec![],
                    }
                } else {
                    PlaneCut::Cut {
                        front: vec![],
                        back: vec![plane],
                    }
                }
            }
            Intersection::Inside(line) => {
                let (res_add1, res_add2) = plane.split(&line);
                let mut front = Vec::new();
                let mut back = Vec::new();

                for sub in Some(plane).into_iter().chain(res_add1).chain(res_add2) {
                    if self.signed_distance_sum_to(&sub) > T::zero() {
                        front.push(sub)
                    } else {
                        back.push(sub)
                    }
                }

                PlaneCut::Cut {
                    front: front,
                    back: back,
                }
            },
        }
    }

    fn is_aligned(&self, plane: &Self) -> bool {
        self.normal.dot(plane.normal) > T::zero()
    }
}


/// Binary Space Partitioning splitter, uses a BSP tree.
pub struct BspSplitter<T, U> {
    tree: BspNode<Polygon<T, U>>,
    result: Vec<Polygon<T, U>>,
}

impl<T, U> BspSplitter<T, U> {
    /// Create a new BSP splitter.
    pub fn new() -> Self {
        BspSplitter {
            tree: BspNode::new(),
            result: Vec::new(),
        }
    }
}

impl<T: Copy + fmt::Debug + PartialOrd + ApproxEq<T> +
        ops::Sub<T, Output=T> + ops::Add<T, Output=T> +
        ops::Mul<T, Output=T> + ops::Div<T, Output=T> +
        Zero + One + Float,
     U> Splitter<T, U> for BspSplitter<T, U> {

    fn reset(&mut self) {
        self.tree = BspNode::new();
    }

    fn add(&mut self, poly: Polygon<T, U>) {
        self.tree.insert(poly);
    }

    fn sort(&mut self, view: TypedPoint3D<T, U>) -> &[Polygon<T, U>] {
        let poly = Polygon {
            points: [TypedPoint3D::zero(); 4],
            normal: view,
            offset: T::zero(),
            anchor: 0,
        };
        self.tree.order(&poly, &mut self.result);
        &self.result
    }
}

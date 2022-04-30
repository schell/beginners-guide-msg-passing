# 🥸 Traits

A trait is a collection of methods for an unknown type.

Most languages have a similar analogue.

* Haskell has typeclasses
* Objective-C has interfaces
* OCaml has modules
* Javascript has prototypes

Traits allow us to write generic interfaces for types without having to
define the types that will implement these interfaces.

As an example we'll write a `Polygon` trait.

```rust
/// We can't talk about polygons without talking about points.
pub struct Point(f32, f32);

impl Point {
   pub fn distance_to(&self, other: &Point) -> f32 {
       ((other.0 - self.0).powf(2.0) + (other.1 - self.1).powf(2.0)).sqrt()
   }
}

pub trait Polygon {
    fn name(&self) -> String;

    fn points(&self) -> Vec<Point>;

    fn num_sides(&self) -> usize {
      self.points().len()
    }
}
```

Now we'll implement this trait for a few of our favorite polygons.

```rust
# pub trait Polygon {
#   fn name(&self) -> String;
#
#   fn points(&self) -> Vec<Point>;
#
#   fn num_sides(&self) -> usize {
#     self.points().len()
#   }
# }

# #[derive(Clone)]
# pub struct Point(f32, f32);

pub struct Triangle([Point; 3]);

impl Polygon for Triangle {
    fn name(&self) -> String {
        "triangle".to_string()
    }

    fn points(&self) -> Vec<Point> {
        self.0.to_vec()
    }
}

impl Polygon for Triangle {
    fn name(&self) -> String {
        "rectangle".to_string()
    }

    fn points(&self) -> Vec<Point> {
        self.0.to_vec()
    }

    /// Let's save some cycles and provide an "optimized" implementation of `num_sides`
    fn num_sides(&self) -> usize {
        4
    }
}
```

We know from maths that there are an INFINITE number of polygons. Now we can write code that
uses `Polygon`s without having to know _exactly what_ polygons we're using:


```rust
# pub trait Polygon {
#   fn name(&self) -> String;
#
#   fn points(&self) -> Vec<Point>;
#
#   fn num_sides(&self) -> usize {
#     self.points().len()
#   }
# }
#
# #[derive(Clone)]
# pub struct Point(f32, f32);
#
# impl Point {
#    pub fn distance_to(&self, other: &Point) -> f32 {
#        ((other.0 - self.0).powf(2.0) + (other.1 - self.1).powf(2.0)).sqrt()
#    }
# }
# pub struct Triangle([Point; 3]);
#
# impl Polygon for Triangle {
#     fn name(&self) -> String {
#         "rectangle".to_string()
#     }
#
#     fn points(&self) -> Vec<Point> {
#         self.0.to_vec()
#     }
#
#     fn num_sides(&self) -> usize {
#         4
#     }
# }
pub fn perimeter(poly: impl Polygon) -> f32 {
    let mut p = 0.0;
    let points = poly.points();
    for (p1, p2) in points.iter().zip(points.iter().skip(1)) {
        p += p1.distance_to(p2);
    }
    p
}
```

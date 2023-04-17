// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use std::{ops::Add, vec};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
    fn magnitude(self) -> f64 {
        f64::from(self.x.pow(2) + self.y.pow(2)).sqrt()
    }
    fn dist(self, other: &Point) -> f64 {
        f64::from((self.x - other.x).pow(2) + (self.y - other.y).pow(2)).sqrt()
    }
}

#[derive(Clone)]
pub struct Polygon {
    points: Vec<Point>,
    current_index: usize,
}

impl Polygon {
    fn new() -> Polygon {
        Polygon {
            points: vec![],
            current_index: 0,
        }
    }
    fn iter<'a>(&'a self) -> std::slice::Iter<'a, Point> {
        self.points.iter()
    }
    fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }
    fn left_most_point(self) -> Option<Point> {
        self.points.iter().min_by_key(|p| p.x).cloned()
    }
}

#[derive(Clone)]
pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    fn new(center: Point, radius: i32) -> Circle {
        Circle {
            center: center,
            radius: radius,
        }
    }
}

trait HasPerimeter {
    fn perimeter(self) -> f64;
}

impl HasPerimeter for Polygon {
    fn perimeter(self) -> f64 {
        if self.points.len() < 2 {
            return 0.0;
        }

        let points = self.points.clone();

        points
            .iter()
            .enumerate()
            .map(|(i, e)| return (e, &points[(i + 1) % points.len()]))
            .map(|(a, b)| return a.dist(b))
            .sum()
    }
}

impl HasPerimeter for Circle {
    fn perimeter(self) -> f64 {
        f64::from(2 * self.radius) * std::f64::consts::PI
    }
}

trait AsShape {
    fn as_shape(self) -> Shape;
}

impl AsShape for Polygon {
    fn as_shape(self) -> Shape {
        Shape::Polygon(self)
    }
}

impl AsShape for Circle {
    fn as_shape(self) -> Shape {
        Shape::Circle(self)
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    fn from(it: impl AsShape) -> Shape {
        it.as_shape()
    }
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(e) => e.clone().perimeter(),
            Shape::Circle(e) => e.clone().perimeter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(&p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

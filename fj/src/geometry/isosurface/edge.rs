use super::GridIndex;

#[derive(Debug, PartialEq)]
pub struct Edge<T> {
    pub a: T,
    pub b: T,
}

impl<T> Edge<T> {
    pub fn reverse(self) -> Self {
        Self {
            a: self.b,
            b: self.a,
        }
    }
}

impl Edge<GridIndex> {
    pub fn direction(&self) -> Direction {
        let direction = [
            self.b.x() as i32 - self.a.x() as i32,
            self.b.y() as i32 - self.a.y() as i32,
            self.b.z() as i32 - self.a.z() as i32,
        ];

        #[rustfmt::skip]
        let (axis, sign) = match direction {
            [ 0,  0, -1] => (Axis::Z, Sign::Neg),
            [ 0,  0,  1] => (Axis::Z, Sign::Pos),
            [ 0, -1,  0] => (Axis::Y, Sign::Neg),
            [ 0,  1,  0] => (Axis::Y, Sign::Pos),
            [-1,  0,  0] => (Axis::X, Sign::Neg),
            [ 1,  0,  0] => (Axis::X, Sign::Pos),

            direction => panic!(
                "Invalid direction ({:?}).\
                Only axis-aligned directions allowed.",
                direction
            ),
        };

        Direction { axis, sign }
    }
}

impl Edge<Value> {
    pub fn at_surface(&self) -> bool {
        let min = f32::min(self.a.value, self.b.value);
        let max = f32::max(self.a.value, self.b.value);

        min <= 0.0 && max > 0.0
    }
}

impl From<Edge<Value>> for Edge<GridIndex> {
    fn from(edge: Edge<Value>) -> Self {
        Self {
            a: edge.a.index,
            b: edge.b.index,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Value {
    pub index: GridIndex,
    pub value: f32,
}

pub struct Direction {
    pub axis: Axis,
    pub sign: Sign,
}

pub enum Axis {
    X,
    Y,
    Z,
}

pub enum Sign {
    Neg,
    Pos,
}

#[cfg(test)]
mod tests {
    use super::{Edge, Value};

    #[test]
    fn at_surface_should_detect_whether_edge_is_at_surface() {
        let inside_surface = Edge {
            a: Value {
                index: [0, 0, 0].into(),
                value: -0.2,
            },
            b: Value {
                index: [0, 0, 0].into(),
                value: -0.1,
            },
        };
        assert_eq!(inside_surface.at_surface(), false);
        assert_eq!(inside_surface.reverse().at_surface(), false);

        let outside_surface = Edge {
            a: Value {
                index: [0, 0, 0].into(),
                value: 0.1,
            },
            b: Value {
                index: [0, 0, 0].into(),
                value: 0.2,
            },
        };
        assert_eq!(outside_surface.at_surface(), false);
        assert_eq!(outside_surface.reverse().at_surface(), false);

        let through_surface = Edge {
            a: Value {
                index: [0, 0, 0].into(),
                value: -0.1,
            },
            b: Value {
                index: [0, 0, 0].into(),
                value: 0.1,
            },
        };
        assert_eq!(through_surface.at_surface(), true);
        assert_eq!(through_surface.reverse().at_surface(), true);

        let inside_to_surface = Edge {
            a: Value {
                index: [0, 0, 0].into(),
                value: -0.1,
            },
            b: Value {
                index: [0, 0, 0].into(),
                value: 0.0,
            },
        };
        assert_eq!(inside_to_surface.at_surface(), false);
        assert_eq!(inside_to_surface.reverse().at_surface(), false);

        let outside_to_surface = Edge {
            a: Value {
                index: [0, 0, 0].into(),
                value: 0.0,
            },
            b: Value {
                index: [0, 0, 0].into(),
                value: 0.1,
            },
        };
        assert_eq!(outside_to_surface.at_surface(), true);
        assert_eq!(outside_to_surface.reverse().at_surface(), true);
    }
}

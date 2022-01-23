use cgmath::{Vector2, Vector3};

pub struct Square {
    position_top_left: Vector2<f32>,
    dimension: Vector2<f32>,
    color: Vector3<f32>,
}

impl Square {
    pub fn new(position: Vector2<f32>, dimension: Vector2<f32>, color: Vector3<f32>) -> Self {
        Square {
            position_top_left: position,
            dimension,
            color,
        }
    }
}

pub struct SquareComposition {
    squares: Vec<Square>,
}

impl SquareComposition {
    pub fn new() -> Self {
        SquareComposition {
            squares: Vec::new(),
        }
    }
    pub fn add_square(&mut self, square: Square) {
        self.squares.push(square);
    }
    pub fn generate_colored_triangles(&self) -> (Vec<Vector2<f32>>, Vec<Vector3<f32>>, Vec<u32>) {
        let mut elements = Vec::new();
        let mut positions = Vec::new();
        let mut colors = Vec::new();

        let mut i = 0;
        for square in self.squares.iter() {
            // p1 - p2
            // |  \  |
            // p3 - p4

            let p1 = square.position_top_left;

            let mut p2 = square.position_top_left;
            p2.x += square.dimension.x;

            let mut p3 = square.position_top_left;
            p3.y += square.dimension.y;

            let p4 = square.position_top_left + square.dimension;

            positions.push(p1);
            colors.push(square.color);
            positions.push(p2);
            colors.push(square.color);
            positions.push(p3);
            colors.push(square.color);
            positions.push(p4);
            colors.push(square.color);

            elements.push(i);
            elements.push(i + 1);
            elements.push(i + 3);

            elements.push(i);
            elements.push(i + 3);
            elements.push(i + 2);
            i += 4;
        }

        (positions, colors, elements)
    }
}

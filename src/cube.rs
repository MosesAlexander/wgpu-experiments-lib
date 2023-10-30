use crate::Vertex;

pub struct Cube {
    pub vertices: Vec<f32>,
    pub indices: Vec<u16>,
    pub center: (f32, f32, f32),
}

impl Cube {
    pub fn new(side_length: f32, center: (f32, f32, f32)) -> Cube {
        // Because of textures, each vertex needs 3 copies in the current format
        // so that each face can have a proper texture
        let mut vertices_cube: Vec<f32> = Vec::with_capacity((3 + 2) * 3 * 6);

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsA,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsA,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsA,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopRight,
            (1.0, 1.0),
        ));

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsB,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsB,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsB,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopLeft,
            (1.0, 1.0),
        ));

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsC,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsC,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsC,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopLeft,
            (1.0, 1.0),
        ));

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsD,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsD,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsD,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopRight,
            (1.0, 1.0),
        ));

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsE,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsE,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsE,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomRight,
            (1.0, 1.0),
        ));

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsF,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsF,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsF,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomLeft,
            (1.0, 1.0),
        ));

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsG,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsG,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsG,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomLeft,
            (1.0, 1.0),
        ));

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsH,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsH,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::CoordsH,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomRight,
            (1.0, 1.0),
        ));

        // This is just hell, gotta find a generic way to produce cubes..
        let indices_cube: Vec<u16> = vec![
            9, 3, 0, 6, 9, 0, // first face
            12, 15, 21, 12, 21, 18, // second face
            2, 5, 17, 2, 17, 14, // third face
            8, 11, 23, 8, 23, 20, // fourth face
            1, 13, 19, 1, 19, 7, // fifth face
            22, 16, 4, 10, 22, 4, // sixth face
        ];

        Cube {
            vertices: vertices_cube,
            indices: indices_cube,
            center: center,
        }
    }

    pub fn generate_texture_coords(
        texture_corer: TextureCorner,
        scale_factor: (f32, f32),
    ) -> [f32; 2] {
        match texture_corer {
            TextureCorner::BottomLeft => [0.0, 0.0],
            TextureCorner::BottomRight => [1.0 * scale_factor.0, 0.0],
            TextureCorner::TopLeft => [0.0, 1.0 * scale_factor.1],
            TextureCorner::TopRight => [1.0 * scale_factor.0, 1.0 * scale_factor.1],
        }
    }

    pub fn generate_cube_corner_coords(
        center_point: (f32, f32, f32),
        side_length: f32,
        cube_corner: CubeCorner,
    ) -> [f32; 3] {
        match cube_corner {
            CubeCorner::CoordsA => [
                center_point.0 + side_length / 2.0,
                center_point.1 + side_length / 2.0,
                center_point.2 - side_length / 2.0,
            ],
            CubeCorner::CoordsB => [
                center_point.0 - side_length / 2.0,
                center_point.1 + side_length / 2.0,
                center_point.2 - side_length / 2.0,
            ],
            CubeCorner::CoordsC => [
                center_point.0 + side_length / 2.0,
                center_point.1 - side_length / 2.0,
                center_point.2 - side_length / 2.0,
            ],
            CubeCorner::CoordsD => [
                center_point.0 - side_length / 2.0,
                center_point.1 - side_length / 2.0,
                center_point.2 - side_length / 2.0,
            ],
            CubeCorner::CoordsE => [
                center_point.0 + side_length / 2.0,
                center_point.1 + side_length / 2.0,
                center_point.2 + side_length / 2.0,
            ],
            CubeCorner::CoordsF => [
                center_point.0 - side_length / 2.0,
                center_point.1 + side_length / 2.0,
                center_point.2 + side_length / 2.0,
            ],
            CubeCorner::CoordsG => [
                center_point.0 + side_length / 2.0,
                center_point.1 - side_length / 2.0,
                center_point.2 + side_length / 2.0,
            ],
            CubeCorner::CoordsH => [
                center_point.0 - side_length / 2.0,
                center_point.1 - side_length / 2.0,
                center_point.2 + side_length / 2.0,
            ],
        }
    }
}
pub enum CubeCorner {
    CoordsA,
    CoordsB,
    CoordsC,
    CoordsD,
    CoordsE,
    CoordsF,
    CoordsG,
    CoordsH,
}

pub enum TextureCorner {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

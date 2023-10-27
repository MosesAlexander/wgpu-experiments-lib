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
            CubeCorner::COORDS_A,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_A,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_A,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopRight,
            (1.0, 1.0),
        ));

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_B,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_B,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_B,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopLeft,
            (1.0, 1.0),
        ));

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_C,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_C,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_C,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopLeft,
            (1.0, 1.0),
        ));

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_D,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_D,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_D,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopRight,
            (1.0, 1.0),
        ));

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_E,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_E,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_E,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomRight,
            (1.0, 1.0),
        ));

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_F,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_F,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::TopRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_F,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomLeft,
            (1.0, 1.0),
        ));

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_G,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_G,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_G,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomLeft,
            (1.0, 1.0),
        ));

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_H,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomLeft,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_H,
        ));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(
            TextureCorner::BottomRight,
            (1.0, 1.0),
        ));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(
            center,
            side_length,
            CubeCorner::COORDS_H,
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
            CubeCorner::COORDS_A => [
                center_point.0 + side_length / 2.0,
                center_point.1 + side_length / 2.0,
                center_point.2 - side_length / 2.0,
            ],
            CubeCorner::COORDS_B => [
                center_point.0 - side_length / 2.0,
                center_point.1 + side_length / 2.0,
                center_point.2 - side_length / 2.0,
            ],
            CubeCorner::COORDS_C => [
                center_point.0 + side_length / 2.0,
                center_point.1 - side_length / 2.0,
                center_point.2 - side_length / 2.0,
            ],
            CubeCorner::COORDS_D => [
                center_point.0 - side_length / 2.0,
                center_point.1 - side_length / 2.0,
                center_point.2 - side_length / 2.0,
            ],
            CubeCorner::COORDS_E => [
                center_point.0 + side_length / 2.0,
                center_point.1 + side_length / 2.0,
                center_point.2 + side_length / 2.0,
            ],
            CubeCorner::COORDS_F => [
                center_point.0 - side_length / 2.0,
                center_point.1 + side_length / 2.0,
                center_point.2 + side_length / 2.0,
            ],
            CubeCorner::COORDS_G => [
                center_point.0 + side_length / 2.0,
                center_point.1 - side_length / 2.0,
                center_point.2 + side_length / 2.0,
            ],
            CubeCorner::COORDS_H => [
                center_point.0 - side_length / 2.0,
                center_point.1 - side_length / 2.0,
                center_point.2 + side_length / 2.0,
            ],
        }
    }
}
pub enum CubeCorner {
    COORDS_A,
    COORDS_B,
    COORDS_C,
    COORDS_D,
    COORDS_E,
    COORDS_F,
    COORDS_G,
    COORDS_H,
}

pub enum TextureCorner {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

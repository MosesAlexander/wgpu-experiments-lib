pub struct Model {
    pub meshes: Vec<Mesh>,
}

// Models can be made up of multiple meshes
impl Model {
    pub fn new() -> Model {
        Model {
            meshes: Vec::new(),
            program: None,
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }
}

pub struct Mesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u16>,
}

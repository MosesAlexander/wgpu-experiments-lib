// The world is made out of terrain, and objects on top of that terrain
// In the beginning the terrain is flat, but eventually we might have to support elevation.
struct World {
    terrain: Vec<TerrainBlock>,
    objects: Vec<WObject>,
}

enum WObject {}
enum TerrainBlock {}
pub trait WorldObject {}

#[cfg(test)]
mod tests {
    use crate::world::WObject;

    #[test]
    fn create_world() {
        let world: WObject;
        println!("Whaddup!");
    }
}

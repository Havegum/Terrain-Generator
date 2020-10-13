fn main() {
    let mut terrain_gen = terrain_generator::terrain_generator::TerrainGenerator::new(None);

    let points = 2u32.pow(13);
    let radius = (500.0 / points as f64).sqrt() / 10.0;

    let world = terrain_gen.world(radius, 30.0);

    println!("{:?}", world);
}

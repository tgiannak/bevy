use bevy::prelude::*;
use std::{fs::File, io::Read};

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup)
        .run();
}

fn setup(world: &mut World, resources: &mut Resources) {
    let mut textures = resources.get_mut::<Assets<Texture>>().unwrap();
    let font_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/fonts/FiraSans-Bold.ttf"
    );
    let mut font_file = File::open(&font_path).unwrap();
    let mut buffer = Vec::new();
    font_file.read_to_end(&mut buffer).unwrap();
    let font = Font::try_from_bytes(buffer).unwrap();

    let texture = font.render_text("Hello from Bevy!", Color::rgba(0.9, 0.9, 0.9, 1.0), 500, 60);
    let half_width = texture.width as f32 / 2.0;
    let half_height = texture.height as f32 / 2.0;
    let texture_handle = textures.add(texture);
    let mut color_materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
    world
        .build()
        // 2d camera
        .add_entity(Camera2dEntity::default())
        // texture
        .add_entity(UiEntity {
            node: Node::new(
                math::vec2(0.0, 0.0),
                Anchors::CENTER,
                Margins::new(-half_width, half_width, -half_height, half_height),
            ),
            material: color_materials.add(ColorMaterial::texture(texture_handle)),
            ..Default::default()
        });
}
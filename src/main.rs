extern crate amethyst;

use amethyst::{
    prelude::*,
    core::Transform,
    assets::{AssetStorage, Loader},
    renderer::{PngFormat, Texture, TextureMetadata, TextureHandle, DisplayConfig,
        DrawFlat2D, Pipeline, RenderBundle, Stage, ColorMask, ALPHA, Transparent},
    utils::application_root_dir,
};


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        application_root_dir()
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()
                .with_transparency(ColorMask::all(), ALPHA, None))
    );

    let game_data =
        GameDataBuilder::default()
            .with_bundle(
                RenderBundle::new(pipe, Some(config))
                    .with_sprite_sheet_processor()
                    .with_sprite_visibility_sorting(&[]),
            )?;

    let mut game = Application::new("./", ExampleState, game_data)?;

    game.run();

    Ok(())
}

pub fn load_texture<N>(name: N, world: &World) -> TextureHandle where N: Into<String> {
    let loader = world.read_resource::<Loader>();
    loader.load(
        name,
        PngFormat,
        TextureMetadata::srgb(),
        (),
        &world.read_resource::<AssetStorage<Texture>>()
    )
}

fn init_image(world: &mut World, texture_handle: &TextureHandle) {
    let mut transform = Transform::default();
    transform.set_x(0.0);
    transform.set_y(0.0);

    world
        .create_entity()
        .with(transform)
        .with(texture_handle.clone())
        .with(Transparent)
        .build();
}

#[derive(Debug)]
struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let path = "texture/spritesheet.png";
        let texture_handle = load_texture(path, world);

        init_image(world, &texture_handle);
    }
}
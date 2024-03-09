use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Invaders".into(),
                resolution: (600., 666.).into(),
                position: WindowPosition::At(IVec2::new(0, 400)), // FIXME: dev only
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, startup_system)
        .run();
}

fn startup_system(mut commands: Commands) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // add rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(150., 150.)),
            ..Default::default()
        },
        ..Default::default()
    });
}

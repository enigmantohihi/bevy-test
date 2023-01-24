#![allow(unused)]

use bevy::prelude::*;

fn main() {
    println!("Hello, world!");
    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .insert_resource(WindowDescriptor {
            title: "UNKO".to_string(),
            width: 600.0,
            height: 400.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // add rectangle
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(150., 150.)),
            ..Default::default()
        },
        ..Default::default()
    });
}
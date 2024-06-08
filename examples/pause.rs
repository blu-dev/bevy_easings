use bevy::{color::palettes, prelude::*};

use bevy_easings::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_easings::EasingsPlugin)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, pause)
        .insert_resource(Time::<Fixed>::from_seconds(0.25))
        .run();

    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                color: palettes::basic::RED.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(-500., 0., 0.)).ease_to(
            Transform::from_translation(Vec3::new(500., 0., 0.)),
            bevy_easings::EaseFunction::QuadraticInOut,
            bevy_easings::EasingType::PingPong {
                duration: std::time::Duration::from_millis(500),
                pause: Some(std::time::Duration::from_millis(100)),
            },
        ),
    ));
}

fn pause(mut query: Query<&mut bevy_easings::EasingComponent<Transform>>) {
    for mut easing in query.iter_mut() {
        easing.state = !easing.state;
    }
}

use bevy::prelude::*;
use bevy_svg::prelude::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "2d_parent_child_translation_check".to_string(),
                width: 600.0,
                height: 600.0,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(common::CommonPlugin)
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup)
        .add_system(svg_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg = asset_server.load("box.svg");
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            Svg2dBundle {
                svg: svg.clone(),
                origin: Origin::Center,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            Direction::Left,
        ))
        .with_children(|parent| {
            parent.spawn((
                Svg2dBundle {
                    svg,
                    origin: Origin::Center,
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.0),
                        scale: Vec3::new(0.5, 0.5, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                // Direction::Up,
            ));
        });
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Still
}

fn svg_movement(
    time: Res<Time>,
    mut svg_position: Query<(&mut Direction, &mut Transform), With<Handle<Svg>>>,
) {
    for (mut direction, mut transform) in &mut svg_position {
        match *direction {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
            Direction::Right => transform.translation.x += 150. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 150. * time.delta_seconds(),
            _ => {}
        }

        match *direction {
            Direction::Up | Direction::Down => {
                if transform.translation.y > 200. {
                    *direction = Direction::Down;
                } else if transform.translation.y < -200. {
                    *direction = Direction::Up;
                }
            },
            Direction::Left | Direction::Right => {
                if transform.translation.x > 200. {
                    *direction = Direction::Left;
                } else if transform.translation.x < -200. {
                    *direction = Direction::Right;
                }
            },
            _ => {}
        }
    }
}

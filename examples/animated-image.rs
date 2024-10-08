use bevy::prelude::*;

use vleue_kinetoscope::{AnimatedImageBundle, AnimatedImageController, AnimatedImagePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AnimatedImagePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (log_updates, reset))
        .run();
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum Image {
    Gif,
    Webp,
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Image::Gif => write!(f, "GIF"),
            Image::Webp => write!(f, "WebP"),
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Query<&Window>) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::None,
            ..default()
        },
        ..default()
    });

    let window_width = window.single().width() / 2.0;

    let mut thing = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(300.0, 200.0)),
            ..default()
        },
        ..default()
    });
    
    thing.with_children(|parent| {
        parent.spawn(AnimatedImageBundle {
            animated_image: asset_server.load("4x_om.webp"),
            ..default()
        },);
    });
}

fn log_updates(
    mut texts: Query<(&mut Text, &Image)>,
    playing_images: Query<(Ref<AnimatedImageController>, &Image)>,
) {
    for (animated_image, image_kind) in &playing_images {
        if animated_image.is_changed() {
            for (mut text, text_kind) in &mut texts {
                if image_kind != text_kind {
                    continue;
                }
                text.sections[2].value = format!("{}", animated_image.play_count());
                text.sections[4].value = format!("{:>4}", animated_image.current_frame());
                text.sections[6].value = format!("{}", animated_image.frame_count() as i32 - 1);
            }
        }
    }
    // let gif = gif.single();
}

fn reset(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut playing_images: Query<&mut AnimatedImageController>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut animated_image in &mut playing_images {
            animated_image.reset();
        }
    }
}

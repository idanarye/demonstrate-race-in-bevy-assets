// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy::text::DEFAULT_FONT_HANDLE;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (ui_update_info, ui_handle_button, populate_sprites))
        .run();
}

#[derive(Component)]
struct ReloadableSprite;

#[derive(Component)]
struct InfoText;

#[derive(Component)]
struct RecreateButton;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(ReloadableSprite);
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|children| {
            let text_style = TextStyle {
                font: DEFAULT_FONT_HANDLE.typed(),
                font_size: 32.0,
                color: Color::BLACK,
            };
            children.spawn(InfoText).insert(TextBundle::from_sections([
                TextSection::new("Entity: ", text_style.clone()),
                TextSection::from_style(text_style.clone()),
                TextSection::new("\nTexture: ", text_style.clone()),
                TextSection::from_style(text_style.clone()),
                TextSection::new("\nTexture Load Status: ", text_style.clone()),
                TextSection::from_style(text_style.clone()),
            ]));
            children
                .spawn(RecreateButton)
                .insert(ButtonBundle::default())
                .with_children(|children| {
                    children.spawn(TextBundle::from_section("Recreate", text_style.clone()));
                });
        });
}

fn ui_update_info(
    mut ui_query: Query<&mut Text, With<InfoText>>,
    sprite_query: Query<(Entity, &Handle<Image>), With<ReloadableSprite>>,
    asset_server: Res<AssetServer>,
) {
    let mut text = ui_query.single_mut();
    let Ok((entity, texture)) = sprite_query.get_single() else { return };
    text.sections[1].value = format!("{entity:?}");
    text.sections[3].value = format!("{texture:?}");
    let load_state = asset_server.get_load_state(texture);
    text.sections[5].value = format!("{load_state:?}");
}

fn ui_handle_button(
    ui_query: Query<&Interaction, (Changed<Interaction>, With<RecreateButton>)>,
    sprite_query: Query<Entity, With<ReloadableSprite>>,
    mut commands: Commands,
) {
    let Ok(interaction) = ui_query.get_single() else { return };
    if *interaction != Interaction::Pressed {
        return;
    }
    for entity in sprite_query.iter() {
        commands.entity(entity).despawn_recursive();
        commands.spawn(ReloadableSprite);
    }
}

fn populate_sprites(
    query: Query<Entity, (With<ReloadableSprite>, Without<Handle<Image>>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(SpriteBundle {
            texture: asset_server.load("icon.png"),
            ..Default::default()
        });
    }
}

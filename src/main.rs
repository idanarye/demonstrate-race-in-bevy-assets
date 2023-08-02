// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EguiPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (ui_system, populate_sprites))
        .run();
}

#[derive(Component)]
struct ReloadableSprite;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(ReloadableSprite);
}

fn ui_system(
    mut contexts: EguiContexts,
    query: Query<(Entity, &Handle<Image>), With<ReloadableSprite>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    egui::Window::new("UI").show(contexts.ctx_mut(), |ui| {
        for (entity, texture) in query.iter() {
            ui.label(format!("Entity: {entity:?}"));
            ui.label(format!("Texture: {texture:?}"));
            let load_state = asset_server.get_load_state(texture);
            ui.label(format!("Texture Load Status: {load_state:?}"));
            if ui.button("Recreate").clicked() {
                commands.entity(entity).despawn_recursive();
                commands.spawn(ReloadableSprite);
            }
        }
    });
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

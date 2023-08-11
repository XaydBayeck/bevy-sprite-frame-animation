use bevy::{prelude::*, window::close_on_esc};
use bevy_editor_pls::EditorPlugin;
use bevy_sprite_frame_animation::animation::{
    AnimationTimer, Indices, PlayController, SpriteFrameAnimationPlugin, SpriteSheetAnimationBundle,
};

#[cfg(feature = "debug")]
mod debug;
#[cfg(feature = "debug")]
use debug::{AnimationPlayState, DebugPlugin};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(SpriteFrameAnimationPlugin::<TextureAtlasSprite, Indices>::default())
        .add_systems(Startup, (set_camera, set_animator))
        .add_systems(Update, close_on_esc);

    #[cfg(feature = "debug")]
    app.add_plugins((
        EditorPlugin::default(),
        DebugPlugin::<Indices, TextureAtlasSprite>::default(),
    ));

    app.run();
}

fn set_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Name::new("MainCamera")));
}

fn set_animator(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 6, 10, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let (first, last) = (0, 5);
    let node = Indices::new(first, last);
    let mut id = commands.spawn(SpriteSheetAnimationBundle {
        controller: PlayController::new(node, false, true),
        timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Once)),
        sprite_atlas: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
    });

    #[cfg(feature = "debug")]
    id.insert(AnimationPlayState::Ready);
}

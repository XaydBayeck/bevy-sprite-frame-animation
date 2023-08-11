use std::marker::PhantomData;

use bevy::{prelude::*, reflect::TypePath};

mod controller;
mod indices;
mod traits;

pub use controller::PlayController;
pub use traits::*;

pub use indices::Indices;

#[derive(Default)]
pub struct SpriteFrameAnimationPlugin<FD, FQ>(PhantomData<(FD, FQ)>);

impl<FD, FQ> Plugin for SpriteFrameAnimationPlugin<FD, FQ>
where
    FD: FrameDiplayer + Component + TypePath,
    FQ: FrameQue<FD> + TypePath + FromReflect,
    PlayController<FQ, FD>: Component,
{
    fn name(&self) -> &str {
        "Sprite based frame animation plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_event::<AnimationFinished>()
            .add_systems(Update, sprite_frame_animation_play_control);

        #[cfg(feature = "debug")]
        app.register_type::<AnimationTimer>()
            .register_type::<PlayController<FQ, FD>>();
    }
}

#[derive(Debug, Clone, Copy, Event)]
pub struct AnimationFinished(pub Entity);

#[derive(Bundle)]
pub struct SpriteSheetAnimationBundle<FQ>
where
    FQ: FrameQue<TextureAtlasSprite>,
{
    pub controller: PlayController<FQ, TextureAtlasSprite>,
    pub timer: AnimationTimer,
    pub sprite_atlas: SpriteSheetBundle,
}

#[derive(Component, Deref, DerefMut, Reflect)]
pub struct AnimationTimer(pub Timer);

pub fn sprite_frame_animation_play_control<FD, FQ>(
    time: Res<Time>,
    mut event: EventWriter<AnimationFinished>,
    mut query: Query<(
        Entity,
        &mut PlayController<FQ, FD>,
        &mut AnimationTimer,
        &mut FD,
    )>,
) where
    FD: FrameDiplayer + Component,
    FQ: FrameQue<FD>,
    PlayController<FQ, FD>: Component,
{
    for (id, mut controller, mut timer, mut displayer) in &mut query {
        if controller.playing() {
            if timer.tick(time.delta()).finished() {
                if let Some(frame) = controller.next_frame() {
                    displayer.set_frame(frame);
                    timer.reset();
                    timer.unpause();
                } else {
                    event.send(AnimationFinished(id))
                }
            }
        }
        if controller.finished() {
            controller.stop();
        } else if controller.ready() {
            if controller.repeat {
                controller.play();
            }
        }
    }
}

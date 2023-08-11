use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_sprite_frame_animation::animation::{FrameDiplayer, FrameQue, PlayController};

use AnimationPlayState::*;

#[derive(Default)]
pub struct DebugPlugin<FQ, FD>(PhantomData<(FQ, FD)>);

impl<FD, FQ> Plugin for DebugPlugin<FQ, FD>
where
    FD: Component + FrameDiplayer,
    FQ: FrameQue<FD>,
{
    fn name(&self) -> &str {
        "Debug Plugin"
    }

    fn build(&self, app: &mut App) {
        app.register_type::<AnimationPlayState>()
            .add_systems(Update, update_play_states::<FD, FQ>);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
pub enum AnimationPlayState {
    Ready,
    Playing,
    Puase,
    Finished,
}

fn update_play_states<FD, FQ>(mut query: Query<(&PlayController<FQ, FD>, &mut AnimationPlayState)>)
where
    FD: Component + FrameDiplayer,
    FQ: FrameQue<FD>,
{
    for (controller, mut state) in &mut query {
        *state = if controller.playing() {
            Playing
        } else if controller.ready() {
            Ready
        } else if controller.finished() {
            Finished
        } else {
            Puase
        }
    }
}

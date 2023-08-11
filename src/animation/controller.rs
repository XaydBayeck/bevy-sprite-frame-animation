use std::marker::PhantomData;

use bevy::{prelude::Component, reflect::Reflect};

use super::{FrameDiplayer, FrameQue};

#[derive(Debug, Component, Reflect)]
pub struct PlayController<FQ, FD> {
    frame_queue: FQ,
    playing: bool,
    pub play_back: bool,
    pub interruptable: bool,
    pub repeat: bool,
    #[reflect(ignore)]
    _fd: PhantomData<FD>,
}

impl<FD, FQ> PlayController<FQ, FD>
where
    FD: FrameDiplayer,
    FQ: FrameQue<FD>,
{
    pub fn new(frame_queue: FQ, interruptable: bool, repeat: bool) -> Self {
        Self {
            frame_queue,
            playing: true,
            play_back: false,
            interruptable,
            repeat,
            _fd: PhantomData,
        }
    }

    pub fn set_repeat(&mut self, repeat: bool) -> &mut Self {
        self.repeat = repeat;
        self
    }

    pub fn set_play_back(&mut self, play_back:bool) -> &mut Self {
        self.play_back = play_back;
        self
    }

    pub fn set_interruptable(&mut self, interruptable: bool) -> &mut Self {
        self.interruptable = interruptable;
        self
    }

    pub fn playing(&self) -> bool {
        self.playing
    }

    pub fn puase(&mut self) -> &mut Self {
        self.playing = false;
        self
    }

    pub fn play(&mut self) -> &mut Self {
        self.playing = true;
        self
    }

    pub fn ready(&self) -> bool {
        self.frame_queue.ready(self.play_back)
    }

    pub fn finished(&self) -> bool {
        self.frame_queue.finished(self.play_back)
    }

    pub fn next_frame(&mut self) -> Option<FD::Frame> {
        self.frame_queue.next(self.play_back)
    }

    pub fn reset(&mut self) {
        self.frame_queue.reset(self.play_back)
    }

    pub fn stop(&mut self) {
        self.reset();
        self.puase();
    }
}

use bevy::{reflect::Reflect, sprite::TextureAtlasSprite};

use super::{FrameDiplayer, FrameQue};

impl FrameDiplayer for TextureAtlasSprite {
    type Frame = usize;

    fn set_frame(&mut self, frame: Self::Frame) {
        self.index = frame;
    }
}

#[derive(Debug, Default, Reflect)]
pub struct Indices {
    index: usize,
    first: usize,
    last: usize,
}

impl FrameQue<TextureAtlasSprite> for Indices {
    fn frame(&self) -> Option<usize> {
        self.index.checked_sub(1)
    }

    fn next_frame(&mut self, play_back: bool) {
        if self.ready(play_back) {
            self.index = if play_back { self.last } else { self.first + 2 };
        } else if !self.finished(play_back) {
            self.index = if play_back {
                self.index.checked_sub(1).unwrap_or(0)
            } else {
                self.index + 1
            };
        }
    }

    fn reset(&mut self, play_back: bool) {
        self.index = if play_back { self.last } else { self.first } + 1;
    }

    fn ready(&self, play_back: bool) -> bool {
        if play_back {
            self.index >= self.last + 1
        } else {
            self.index <= self.first + 1
        }
    }

    fn finished(&self, play_back: bool) -> bool {
        if play_back {
            self.index <= self.first + 1
        } else {
            self.index >= self.last + 1
        }
    }
}

impl Indices {
    pub fn new(first: usize, last: usize) -> Self {
        Self {
            index: first + 1,
            first,
            last,
        }
    }
}

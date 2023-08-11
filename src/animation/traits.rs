pub trait FrameDiplayer {
    type Frame;

    fn set_frame(&mut self, frame: Self::Frame);
}

pub trait FrameQue<FD: FrameDiplayer>: Send + Sync + 'static {
    fn frame(&self) -> Option<FD::Frame>;

    fn next_frame(&mut self, play_back: bool);

    fn reset(&mut self, play_back: bool);

    fn ready(&self, play_back: bool) -> bool;

    fn finished(&self, play_back: bool) -> bool;

    fn next(&mut self, play_back: bool) -> Option<FD::Frame> {
        if self.finished(play_back) {
            None
        } else {
            self.next_frame(play_back);
            self.frame()
        }
    }
}

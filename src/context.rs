use anyhow::Context;
use tts::Tts;
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

const WINDOW_TITLE: &str = concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"));

pub struct GameContext {
    win: Window,
    speaker: Tts,
}

impl GameContext {
    pub fn new(event_loop: &EventLoop<()>) -> anyhow::Result<Self> {
        let win = WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .with_inner_size(LogicalSize::new(640, 480))
            .with_resizable(false)
            .build(&event_loop)
            .context("Could not create window")?;
        Ok(Self {
            win,
            speaker: Tts::default().context("Could not initialize TTS engine")?,
        })
    }
}

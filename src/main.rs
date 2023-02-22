mod context;
mod state_manager;

use winit::event_loop::EventLoop;

use crate::{context::GameContext, state_manager::StateManager};

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::<()>::new();
    let ctx = GameContext::new(&event_loop)?;
    let states = StateManager::<GameContext>::new(4);
    event_loop.run(|_, _, _| {});
}

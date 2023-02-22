mod context;
mod state_manager;

use winit::event_loop::EventLoop;

use crate::{context::GameContext, state_manager::StateManager};

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::<()>::new();
    let mut ctx = GameContext::new(&event_loop)?;
    let mut states = StateManager::<GameContext>::new(4);
    event_loop.run(move |event, _, _| {
        if ctx.feed_event(&event) {
            // Todo: Better error handling
            if !states.on_update(&mut ctx).unwrap() {
                std::process::exit(0);
            }
        }
    });
}

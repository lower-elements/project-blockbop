mod context;
mod menu;
mod sound;
mod state_manager;

use winit::event_loop::EventLoop;

use crate::{
    context::GameContext,
    menu::{MenuBuilder, MenuItem},
    state_manager::{StateManager, Transition},
};

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::<()>::new();
    let mut ctx = GameContext::new(&event_loop)?;
    let mut states = StateManager::<GameContext>::new(4);
    let main_menu = MenuBuilder::new("Main")
        .item(MenuItem::new("Exit").on_activate(|_, _| Ok(Transition::Pop(1))))
        .build();
    let main_menu = Box::new(main_menu);
    states.push_state(main_menu, &mut ctx)?;
    event_loop.run(move |event, _, _| {
        if ctx.feed_event(&event) {
            // Todo: Better error handling
            if !states.on_update(&mut ctx).unwrap() {
                std::process::exit(0);
            }
        }
    });
}

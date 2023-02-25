use winit::event::VirtualKeyCode;

use crate::{
    context::GameContext,
    state_manager::{State, Transition},
};

type MenuCallback =
    Box<dyn Fn(&mut GameContext, String) -> anyhow::Result<Transition<GameContext>>>;

pub struct Menu {
    title: String,
    items: Vec<String>,
    selected: Option<usize>,
    callback: Option<MenuCallback>,
}

impl Menu {
    pub fn new<I, S>(title: String, i: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            title,
            items: i.into_iter().map(Into::into).collect(),
            selected: None,
            callback: None,
        }
    }

    pub fn set_callback(mut self, callback: MenuCallback) -> Self {
        self.callback = Some(callback);
        self
    }
    fn next_item(&mut self, ctx: &mut GameContext) -> anyhow::Result<()> {
        let selected = if let Some(idx) = self.selected {
            (idx + 1) % self.items.len()
        } else {
            0
        };
        ctx.speaker.speak(
            format!(
                "{}. {} of {}",
                self.items[selected],
                selected + 1,
                self.items.len()
            ),
            true,
        )?;
        self.selected = Some(selected);
        Ok(())
    }

    fn previous_item(&mut self, ctx: &mut GameContext) -> anyhow::Result<()> {
        let selected = if let Some(idx) = self.selected {
            (idx + self.items.len() - 1) % self.items.len()
        } else {
            self.items.len() - 1
        };
        ctx.speaker.speak(
            format!(
                "{}. {} of {}",
                self.items[selected],
                selected + 1,
                self.items.len()
            ),
            true,
        )?;
        self.selected = Some(selected);
        Ok(())
    }

    fn activate_item(&mut self, ctx: &mut GameContext) -> anyhow::Result<Transition<GameContext>> {
        // Returns transition to be able to make it so some items when clicked can pop the state or add new items without needing to go insane about things.
        // It also means that some items could be clicked and just print a message or do something, But not destroy the menu entirely.
        if let Some(idx) = self.selected {
            if let Some(cb) = &self.callback {
                let txt = self.items[idx].clone();
                cb(ctx, txt)
            } else {
                Ok(Transition::None)
            }
        } else {
            Ok(Transition::None)
        }
    }
}

impl State<GameContext> for Menu {
    fn on_push(&mut self, ctx: &mut GameContext) -> anyhow::Result<()> {
        ctx.speaker.speak(
            format!(
                "{} Menu. {} item{}",
                self.title,
                self.items.len(),
                if self.items.len() > 1 { 's' } else { ' ' }
            ),
            true,
        )?;
        Ok(())
    }

    fn on_update(
        &mut self,
        ctx: &mut GameContext,
        depth: usize,
    ) -> anyhow::Result<Transition<GameContext>> {
        // If we're not the active state, return immediately
        if depth != 0 {
            return Ok(Transition::None);
        }

        if ctx.input.close_requested() || ctx.input.key_pressed(VirtualKeyCode::Escape) {
            return Ok(Transition::Pop(1));
        }

        // Input
        if ctx.input.key_pressed_os(VirtualKeyCode::Down) {
            self.next_item(ctx)?;
        }
        if ctx.input.key_pressed_os(VirtualKeyCode::Up) {
            self.previous_item(ctx)?;
        }
        if ctx.input.key_pressed(VirtualKeyCode::Return) {
            return self.activate_item(ctx);
        }

        Ok(Transition::None)
    }
}

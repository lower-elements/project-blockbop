use winit::event::VirtualKeyCode;

use crate::{
    context::GameContext,
    state_manager::{State, Transition},
};

pub struct Menu {
    title: String,
    items: Vec<String>,
    selected: Option<usize>,
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
        }
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

    fn activate_item(&mut self, _ctx: &mut GameContext) -> anyhow::Result<()> {
        todo!("Activating menu items")
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
            self.activate_item(ctx)?;
        }

        Ok(Transition::None)
    }
}

use std::string::ToString;

use winit::event::VirtualKeyCode;

use crate::{
    context::GameContext,
    state_manager::{State, Transition},
};

pub type MenuItemCallback<MCtx> =
    fn(&mut MenuItem<MCtx>, &mut GameContext, &mut MCtx) -> anyhow::Result<Transition<GameContext>>;

pub struct MenuItem<MCtx> {
    text: String,
    activate_callback: Option<MenuItemCallback<MCtx>>,
}

impl<MCtx> MenuItem<MCtx> {
    pub fn new<S: ToString>(text: S) -> Self {
        Self {
            text: text.to_string(),
            activate_callback: None,
        }
    }

    pub fn on_activate(mut self, activate_callback: MenuItemCallback<MCtx>) -> Self {
        self.activate_callback = Some(activate_callback);
        self
    }

    pub fn speak(
        &self,
        ctx: &mut GameContext,
        pos: usize,
        len: usize,
    ) -> Result<Option<tts::UtteranceId>, tts::Error> {
        ctx.speaker
            .speak(format!("{}. {} of {}", self.text, pos, len), true)
    }
}

pub struct MenuBuilder<MCtx> {
    title: String,
    items: Vec<MenuItem<MCtx>>,
    menu_context: MCtx,
}

impl<MCtx> MenuBuilder<MCtx> {
    pub fn new<S: ToString>(title: S, menu_context: MCtx) -> Self {
        Self {
            title: title.to_string(),
            items: Vec::new(),
            menu_context,
        }
    }

    pub fn build(self) -> Menu<MCtx> {
        Menu {
            title: self.title,
            items: self.items,
            menu_context: self.menu_context,
            selected: None,
        }
    }

    pub fn item(mut self, item: MenuItem<MCtx>) -> Self {
        self.items.push(item);
        self
    }
}

pub struct Menu<MCtx> {
    title: String,
    items: Vec<MenuItem<MCtx>>,
    menu_context: MCtx,
    selected: Option<usize>,
}

impl<MCtx> Menu<MCtx> {
    fn next_item(&mut self, ctx: &mut GameContext) -> anyhow::Result<()> {
        let selected = if let Some(idx) = self.selected {
            (idx + 1) % self.items.len()
        } else {
            0
        };
        self.items[selected].speak(ctx, selected + 1, self.items.len())?;
        self.selected = Some(selected);
        Ok(())
    }

    fn previous_item(&mut self, ctx: &mut GameContext) -> anyhow::Result<()> {
        let selected = if let Some(idx) = self.selected {
            (idx + self.items.len() - 1) % self.items.len()
        } else {
            self.items.len() - 1
        };
        self.items[selected].speak(ctx, selected + 1, self.items.len())?;
        self.selected = Some(selected);
        Ok(())
    }

    fn activate_item(&mut self, ctx: &mut GameContext) -> anyhow::Result<Transition<GameContext>> {
        let Some(selected) = self.selected else { return Ok(Transition::None); };
        let Some(cb) = self.items[selected].activate_callback else { return Ok(Transition::None); };
        cb(&mut self.items[selected], ctx, &mut self.menu_context)
    }
}

impl<MCtx> State<GameContext> for Menu<MCtx> {
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

/// A game state
pub trait State<Ctx> {
    // Called once per frame to update the `State`. `ctx` is the context passed to the
    // [`StateManager`], `depth` is the number of states above this one in the stack.
    fn on_update(&mut self, _ctx: &mut Ctx, _depth: usize) -> anyhow::Result<Transition<Ctx>> {
        Ok(Transition::None)
    }

    // Called just before the `State` is first pushed to the state stack
    fn on_push(&mut self, _ctx: &mut Ctx) -> anyhow::Result<()> {
        Ok(())
    }

    // Called just after the `State` is popped from the state stack
    fn on_pop(&mut self, _ctx: &mut Ctx) -> anyhow::Result<()> {
        Ok(())
    }
}

/// A state transition, returned by [`State::on_update`]
pub enum Transition<Ctx> {
    // No change
    None,
    /// The specified state is pushed onto the state stack
    Push(Box<dyn State<Ctx>>),
    /// The specified number of states are popped from the state stack
    Pop(usize),
    /// The specified number of states are popped from the state stack, then the specified state is
    /// pushed onto the state stack
    Replace(usize, Box<dyn State<Ctx>>),
}

/// Manages a stack of [`State`] trait objects, dispatching updates to them
#[derive(Default)]
pub struct StateManager<Ctx> {
    /// The stack of states
    states: Vec<Box<dyn State<Ctx>>>,
}

impl<Ctx> StateManager<Ctx> {
    /// Create a new `StateManager`. `suggested_capacity` will be the initial capacity of the state
    /// stack.
    pub fn new(suggested_capacity: usize) -> Self {
        Self {
            states: Vec::with_capacity(suggested_capacity),
        }
    }

    /// Push the specified state onto the state stack.
    #[inline]
    pub fn push_state(
        &mut self,
        mut state: Box<dyn State<Ctx>>,
        ctx: &mut Ctx,
    ) -> anyhow::Result<()> {
        state.on_push(ctx)?;
        self.states.push(state);
        Ok(())
    }

    /// Pop the specified number of states from the state stack.
    pub fn pop_states(&mut self, num_states: usize, ctx: &mut Ctx) -> anyhow::Result<()> {
        for _ in 0..num_states {
            let mut state = self.states.pop().expect("Tried to pop too many states");
            state.on_pop(ctx)?;
        }
        Ok(())
    }

    /// Update the [`State`]s in the state stack, modifying the stack if necessary.
    pub fn on_update(&mut self, ctx: &mut Ctx) -> anyhow::Result<()> {
        // A single transition, which will be returned by the top state
        let mut pending_transition = Transition::None;
        // Depth of the current state (I.E. how many states are above it)
        let mut depth = 0;

        for state in self.states.iter_mut() {
            pending_transition = state.on_update(ctx, depth)?;
            depth += 1;
        }

        self.apply_pending_transition(pending_transition, ctx)
    }

    /// Internal function to perform a [`Transition`] on a `StateManager`.
    fn apply_pending_transition(
        &mut self,
        transition: Transition<Ctx>,
        ctx: &mut Ctx,
    ) -> anyhow::Result<()> {
        match transition {
            Transition::None => Ok(()),
            Transition::Push(state) => self.push_state(state, ctx),
            Transition::Pop(num_states) => self.pop_states(num_states, ctx),
            Transition::Replace(num_states, state) => {
                self.pop_states(num_states, ctx)?;
                self.push_state(state, ctx)
            }
        }
    }
}

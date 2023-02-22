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

/// A game state
pub trait State<Ctx> {
    fn on_update(&mut self, _ctx: &mut Ctx) -> anyhow::Result<Transition<Ctx>> {
        Ok(Transition::None)
    }
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
    pub fn push_state(&mut self, state: Box<dyn State<Ctx>>) {
        self.states.push(state);
    }

    /// Pop the specified number of states from the state stack.
    pub fn pop_states(&mut self, num_states: usize) {
        let new_len = self
            .states
            .len()
            .checked_sub(num_states)
            .expect("Tried to remove too many states");
        self.states.truncate(new_len);
    }

    /// Update the [`State`]s in the state stack, modifying the stack if necessary.
    pub fn on_update(&mut self, ctx: &mut Ctx) -> anyhow::Result<()> {
        // A single transition, which will be returned by the top state
        let mut pending_transition = Transition::None;

        for state in self.states.iter_mut() {
            pending_transition = state.on_update(ctx)?;
        }

        self.apply_pending_transition(pending_transition);
        Ok(())
    }

    /// Internal function to perform a [`Transition`] on a `StateManager`.
    fn apply_pending_transition(&mut self, transition: Transition<Ctx>) {
        match transition {
            Transition::None => {}
            Transition::Push(state) => self.push_state(state),
            Transition::Pop(num_states) => self.pop_states(num_states),
            Transition::Replace(num_states, state) => {
                self.pop_states(num_states);
                self.push_state(state);
            }
        }
    }
}

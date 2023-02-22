use alto::{Alto, OutputDevice};
use anyhow::Context;

pub struct SoundManager {
    ctx: alto::Context,
    device: OutputDevice,
    alto: Alto,
}

impl SoundManager {
    pub fn new() -> anyhow::Result<Self> {
        let alto = Alto::load_default().context("Could not load OpenAL library")?;
        let device = alto
            .open(None)
            .context("Could not open audio output device")?;
        let ctx = device
            .new_context(None)
            .context("Could not create audio context")?;
        Ok(Self { alto, device, ctx })
    }
}

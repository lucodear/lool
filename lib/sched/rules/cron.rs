use {
    core::fmt,
    croner::Cron as Croner,
    eyre::Result,
    std::{
        fmt::Debug,
        ops::{Deref, DerefMut},
    },
};

#[derive(Clone)]
pub struct Cron(Croner);

impl Cron {
    /// 🧉 » create a new `Cron` scheduling rule
    pub fn new(pattern: &str) -> Result<Self> {
        let cron = Croner::new(pattern).parse()?;
        Ok(Self(cron))
    }
}

impl Debug for Cron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cron").field("expression", &self.0.pattern).finish()
    }
}

impl Deref for Cron {
    type Target = Croner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Cron {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

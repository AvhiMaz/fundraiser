use super::{DataLen, Initialized};

#[repr(C)]
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Contribute {
    is_initialized: bool,
    amount: u64,
}

impl DataLen for Contribute {
    const LEN: usize = core::mem::size_of::<Contribute>();
}

impl Initialized for Contribute {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Contribute {
    pub fn initialize(&mut self, amount: u64) {
        self.is_initialized = true;
        self.amount = amount;
    }
}

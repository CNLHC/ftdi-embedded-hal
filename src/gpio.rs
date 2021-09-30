use crate::{FtInner, PinUse};
use ftdi_mpsse::{MpsseCmdBuilder, MpsseCmdExecutor};
use std::{cell::RefCell, sync::Mutex};
use crate::error::FtHalError;
use crate::error::Result;

/// FTDI output pin.
///
/// This is created by calling [`FtHal::ad0`] - [`FtHal::ad7`].
///
/// [`FtHal::ad0`]: crate::FtHal::ad0
/// [`FtHal::ad7`]: crate::FtHal::ad7
#[derive(Debug)]
pub struct OutputPin<'a, Device: MpsseCmdExecutor>
    where FtHalError: From<<Device as MpsseCmdExecutor>::Error>,
{
    /// Parent FTDI device.
    mtx: &'a Mutex<RefCell<FtInner<Device>>>,
    /// GPIO pin index.  0-7 for the FT232H.
    idx: u8,
}

impl<'a, Device: MpsseCmdExecutor> OutputPin<'a, Device>
    where FtHalError: From<<Device as MpsseCmdExecutor>::Error>,
{
    pub(crate) fn new(mtx: &'a Mutex<RefCell<FtInner<Device>>>, idx: u8) -> OutputPin<'a, Device> {
        let lock = mtx.lock().expect("Failed to aquire FTDI mutex");
        let mut inner = lock.borrow_mut();
        inner.allocate_pin(idx, PinUse::Output);
        OutputPin { mtx, idx }
    }

    pub(crate) fn set(&self, state: bool) -> Result<()> {
        let lock = self.mtx.lock().expect("Failed to aquire FTDI mutex");
        let mut inner = lock.borrow_mut();

        if state {
            inner.value |= self.mask();
        } else {
            inner.value &= !self.mask();
        };

        let cmd: MpsseCmdBuilder = MpsseCmdBuilder::new()
            .set_gpio_lower(inner.value, inner.direction)
            .send_immediate();
        inner.ft.send(cmd.as_slice())?;

        Ok(())
    }
}

impl<'a, Device: MpsseCmdExecutor> OutputPin<'a, Device>
    where FtHalError: From<<Device as MpsseCmdExecutor>::Error>,
{
    /// Convert the GPIO pin index to a pin mask
    pub(crate) fn mask(&self) -> u8 {
        1 << self.idx
    }
}

impl<'a, Device: MpsseCmdExecutor> embedded_hal::digital::v2::OutputPin for OutputPin<'a, Device>
    where FtHalError: From<<Device as MpsseCmdExecutor>::Error>,
{
    type Error = FtHalError;

    fn set_low(&mut self) -> Result<()> {
        self.set(false)
    }

    fn set_high(&mut self) -> Result<()> {
        self.set(true)
    }
}

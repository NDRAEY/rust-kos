use super::console_dll;
use super::console_shell;

use super::dll;
use super::error::*;

pub enum Console {
    Shell(console_shell::Console),
    Dll(console_dll::Console),
}

impl Console {
    pub fn new() -> SystemResult<Self> {
        let shell_console = console_shell::Console::new();

        if let Some(con) = shell_console {
            return Ok(Self::Shell(con));
        }

        console_dll::Console::init_from_lib().ok_or(Error::Io.into()).map(Self::Dll)
    }

    pub fn write(&mut self, data: &[u8]) -> SystemResult<()> {
        match self {
            Self::Shell(sh) => sh.write(data),
            Self::Dll(lib) => lib.write(data),
        }

        Ok(())
    }
}
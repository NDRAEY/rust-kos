use crate::ffi::OsString;
use crate::fmt;

use core::iter::{Chain, Once};
use core::str::Split;

pub struct Args {
    it: Chain<Once<&'static str>, Split<'static, char>>
}

pub fn args() -> Args {
    // Check UTF-8?
    let app_path = crate::sys::pal::application_path().to_str().unwrap();

    let args = crate::sys::pal::command_line().to_str().unwrap();
    
    // TODO: Do proper parsing of \" symbols to allow whitespace-containing arguments.
    let args_iter = args.split(' ').into_iter();

    Args {
        it: crate::iter::once(app_path).chain(args_iter).into_iter()
    }
}

impl fmt::Debug for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().finish()
    }
}

impl Iterator for Args {
    type Item = OsString;

    #[inline]
    fn next(&mut self) -> Option<OsString> {
        self.it.next().map(|x| x.into())
    }

    // #[inline]
    // fn size_hint(&self) -> (usize, Option<usize>) {
    //     (0, Some(0))
    // }
}

impl DoubleEndedIterator for Args {
    #[inline]
    fn next_back(&mut self) -> Option<OsString> {
        None
    }
}

impl ExactSizeIterator for Args {
    #[inline]
    fn len(&self) -> usize {
        0
    }
}

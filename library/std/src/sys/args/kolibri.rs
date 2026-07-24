use crate::ffi::OsString;
use crate::fmt;

pub struct Args {
    args: crate::vec::IntoIter<OsString>
}

pub fn args() -> Args {
    // Check UTF-8?
    let app_path = crate::sys::pal::application_path().to_str().unwrap();

    let args = crate::sys::pal::command_line().to_str().unwrap();
    
    // TODO: Do proper parsing of \" symbols to allow whitespace-containing arguments.
    let normalized_args = args.trim();

    // If command line arguments are empty, no not add them into the iterator.
    let args: Vec<OsString> = if normalized_args.is_empty() {
        crate::iter::once(app_path).map(|x| OsString::from(x)).collect()
    } else {
        crate::iter::once(app_path).chain(normalized_args.split(' ').into_iter()).map(|x| OsString::from(x)).collect()
    };

    Args {
        args: args.into_iter()
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
        self.args.next().map(|x| x.into())
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

use termcolor::{ColorSpec, StandardStream, WriteColor};

pub trait ColorExt {
    fn with_color<F, T>(&mut self, spec: &ColorSpec, f: F) -> T
    where
        F: FnOnce() -> T;
}

impl ColorExt for StandardStream {
    fn with_color<F, T>(&mut self, spec: &ColorSpec, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.set_color(spec).unwrap();
        let v = f();
        self.reset().unwrap();
        v
    }
}

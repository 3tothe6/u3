use termcolor::{ColorSpec, StandardStream, WriteColor};

pub trait ColorExt {
    fn with_color<F, T>(&mut self, spec: &ColorSpec, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T;
}

impl ColorExt for StandardStream {
    fn with_color<F, T>(&mut self, spec: &ColorSpec, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        self.set_color(spec).unwrap();
        let v = f(self);
        self.reset().unwrap();
        v
    }
}

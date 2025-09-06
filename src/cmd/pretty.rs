use super::*;

pub struct Pretty<C> {
    pub(super) inner: C,
}

impl<C: StatusExt> StatusExt for Pretty<C> {
    fn status(&mut self) -> ExitStatus {
        todo!()
    }
}

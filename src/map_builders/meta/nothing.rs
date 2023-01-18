pub struct Nothing {}

impl super::MetaMapBuilder for Nothing {
    fn build_map(&mut self, _: &mut super::BuilderMap) {}
}

impl Nothing {
    #[allow(dead_code)]
    pub fn new() -> Box<Nothing> {
        Box::new(Nothing {})
    }
}

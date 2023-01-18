pub struct ForestBuilder {}

impl super::MetaMapBuilder for ForestBuilder {
    fn build_map(&mut self, _: &mut super::BuilderMap) {}
}

impl ForestBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<ForestBuilder> {
        Box::new(ForestBuilder {})
    }
}

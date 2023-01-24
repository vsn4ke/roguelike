use super::{
    prefab_local::{char_to_map, char_to_vec},
    Description, X, Y,
};

#[allow(dead_code)]
pub struct PrefabSectionBuilder {
    section: PrefabSection,
    description: Vec<Description>,
}

#[derive(Clone, Copy)]
pub struct PrefabSection {
    pub template: &'static str,
    pub width: usize,
    pub height: usize,
    pub placement: (X, Y),
}
impl PrefabSectionBuilder {
    pub fn new(section: PrefabSection, description: Vec<Description>) -> Box<Self> {
        Box::new(Self {
            section,
            description,
        })
    }
}

impl super::MetaMapBuilder for PrefabSectionBuilder {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let char_vec = char_to_vec(self.section.template);

        let chunk_x = match self.section.placement.0 {
            X::Center => data.map.width as usize / 2 - self.section.width / 2,
            X::Left => 0,
            X::Right => data.map.width as usize - 1 - self.section.width,
        };

        let chunk_y = match self.section.placement.1 {
            Y::Bottom => data.map.height as usize - 1 - self.section.height,
            Y::Center => data.map.height as usize / 2 - self.section.height / 2,
            Y::Top => 0,
        };

        data.spawn_list.retain(|i| {
            let x = i.0 % data.map.width as usize;
            let y = i.0 / data.map.width as usize;
            x < chunk_x
                || x > chunk_x + self.section.width
                || y < chunk_y
                || y > chunk_y + self.section.height
        });

        let mut i = 0;
        for y in 0..self.section.height {
            for x in 0..self.section.width {
                if x < data.map.width as usize && y < data.map.height as usize {
                    let idx = data
                        .map
                        .coord_to_index((x + chunk_x) as i32, (y + chunk_y) as i32);
                    char_to_map(char_vec[i], idx, data, &self.description);
                }
                i += 1;
            }
        }

        data.take_snapshot();
    }
}

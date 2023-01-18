use bracket_lib::terminal::{DistanceAlg, Point};

#[allow(dead_code)]
pub enum Sorter {
    LeftMost,
    RightMost,
    TopMost,
    BottomMost,
    Central,
}

pub struct RoomSorter {
    sort_by: Sorter,
}

impl super::MetaMapBuilder for RoomSorter {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        data.rooms
            .as_mut()
            .unwrap()
            .sort_by(|a, b| match self.sort_by {
                Sorter::LeftMost => a.x1.cmp(&b.x1),
                Sorter::RightMost => b.x2.cmp(&a.x2),
                Sorter::TopMost => b.y1.cmp(&a.y1),
                Sorter::BottomMost => b.y2.cmp(&a.y2),
                Sorter::Central => {
                    let map_center = Point::new(data.map.width / 2, data.map.height / 2);
                    let da = DistanceAlg::PythagorasSquared.distance2d(a.center(), map_center);
                    let db = DistanceAlg::PythagorasSquared.distance2d(b.center(), map_center);
                    da.partial_cmp(&db).unwrap()
                }
            })
    }
}

impl RoomSorter {
    #[allow(dead_code)]
    pub fn new(sort_by: Sorter) -> Box<RoomSorter> {
        Box::new(RoomSorter { sort_by })
    }
}

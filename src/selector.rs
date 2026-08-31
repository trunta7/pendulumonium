pub enum ActiveSelector {
    RectSel,
    PointSel,
}
impl Default for ActiveSelector {
    fn default() -> Self {
        Self::RectSel
    }
}

// handles different selector and decides active selector
pub struct Selector {
    pub rect: RectSelector,
    pub point: PointSelector,
    pub active: ActiveSelector,
}
impl Default for  Selector {
    fn default() -> Self {
        Self {
            rect: RectSelector::default(),
            point: PointSelector::default(),
            active: ActiveSelector::default(),
        }
    }
}

// selection mode based on the rectangle bound by two points on the angle space
pub struct RectSelector {

}
impl Default for RectSelector {
    fn default() -> Self {
        Self {

        }
    }
}

// selection mode based on individual points selected on the angle space
pub struct PointSelector {

}
impl Default for PointSelector {
    fn default() -> Self {
        Self {

        }
    }
}
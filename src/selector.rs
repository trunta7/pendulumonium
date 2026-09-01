use std::{cmp::min, f64::consts::{PI, TAU}};

use eframe::egui::{Pos2, accesskit::Point};

use crate::rk8solver::State;

const SEL_WIDTH: f32 = 200.0;

#[derive(PartialEq)]
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
impl Selector {
    pub fn add_selection(&mut self, point: Pos2) {
        match self.active {
            ActiveSelector::PointSel => {
                return self.point.add_selection(point);
            }
            ActiveSelector::RectSel => {
                return self.rect.add_selection(point);
            }
        }
    }

    pub fn get_selection(&self) -> Vec<State> {
        match self.active {
            ActiveSelector::PointSel => {
                return self.point.get_selection();
            }
            ActiveSelector::RectSel => {
                return self.rect.get_selection();
            }
        }
    }

    pub fn get_points(&self) -> Vec<Pos2> {
        match self.active {
            ActiveSelector::PointSel => {
                return self.point.get_points();
            }
            ActiveSelector::RectSel => {
                return self.rect.get_points();
            }
        }
    }
}

// selection mode based on the rectangle bound by two points on the angle space
// point should be scaled from 0 to 1
pub struct RectSelector {
    pub width: usize,
    points: [Pos2; 2],
}
impl Default for RectSelector {
    fn default() -> Self {
        Self {
            points: [Pos2 { x:0.0, y:0.0 }, Pos2 { x:1.0, y:1.0 }],
            width: 100,
        }
    }
}
impl RectSelector {
    pub fn add_selection(&mut self, point: Pos2) {
        self.points[0] = self.points[1];
        self.points[1] = point;
    }

    pub fn get_selection(&self) -> Vec<State> {
        let min_x = self.points[0].x.min(self.points[1].x);
        let min_y = self.points[0].y.min(self.points[1].y);
        let max_x = self.points[0].x.max(self.points[1].x);
        let max_y = self.points[0].y.max(self.points[1].y);

        let x_range = max_x - min_x;
        let y_range = max_y - min_y;

        let x_scale = x_range / self.width as f32;
        let y_scale = y_range / self.get_height() as f32;

        let mut rec_points: Vec<Pos2> = Vec::new();

        for y_ind in 0..self.get_height() {
            for x_ind in 0..self.width {
                let x = min_x + x_ind as f32 * x_scale;
                let y = min_y + y_ind as f32 * y_scale;
                rec_points.push(Pos2 {x: x, y: y});
            }
        }

        rec_points
        .iter()
        .map(|p| state_from_point(p))
        .collect()
    }

    pub fn get_height(&self) -> usize {
        let min_x = self.points[0].x.min(self.points[1].x);
        let min_y = self.points[0].y.min(self.points[1].y);
        let max_x = self.points[0].x.max(self.points[1].x);
        let max_y = self.points[0].y.max(self.points[1].y);
        let x_range = max_x - min_x;
        let y_range = max_y - min_y;

        // determining height to scale of the width
        let y_scale = y_range / x_range;
        let height = (y_scale * self.width as f32) as usize;

        // scale to even number
        if height % 2 == 1 {
            return height + 1;
        }

        return height;
    }

    pub fn get_points(&self) -> Vec<Pos2> {
        let min_x = self.points[0].x.min(self.points[1].x);
        let min_y = self.points[0].y.min(self.points[1].y);
        let max_x = self.points[0].x.max(self.points[1].x);
        let max_y = self.points[0].y.max(self.points[1].y);

        vec![
            Pos2 {x:min_x, y: min_y},
            Pos2 {x:max_x, y: max_y}
        ]
    }
}

// selection mode based on individual points selected on the angle space
// point should be scaled from 0 to 1
pub struct PointSelector {
    points: Vec<Pos2>
}
impl Default for PointSelector {
    fn default() -> Self {
        Self {
            points: Vec::new()
        }
    }
}
impl PointSelector {
    pub fn add_selection(&mut self, point: Pos2) {
        self.points.push(point);
    }

    pub fn get_selection(&self) -> Vec<State> {
        self.points
        .iter()
        .map(|p| state_from_point(p))
        .collect()
    }

    pub fn get_points(&self) -> Vec<Pos2> {
        self.points.iter().cloned().collect()
    }
}

fn state_from_point(point: &Pos2) -> State {
    let t1 = point.x as f64 * TAU - PI;
    let t2 = point.y as f64 * TAU - PI;

    State { 
        theta1: t1, 
        theta2: t2, 
        omega1: 0.0, 
        omega2: 0.0, 
    }
}
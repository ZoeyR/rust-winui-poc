use bindings::Windows::UI::Xaml::{HorizontalAlignment, VerticalAlignment};

#[derive(Debug)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Stretch,
    Top,
    Bottom,
}

impl Into<HorizontalAlignment> for Alignment {
    fn into(self) -> HorizontalAlignment {
        unimplemented!()
    }
}

impl Into<VerticalAlignment> for Alignment {
    fn into(self) -> VerticalAlignment {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum GridUnitType {
    Star,
    Pixel(f64),
    Stretch,
}

#[derive(Debug)]
pub struct Margin {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}

pub mod utils;

use bindings::Windows::UI::Xaml::{
    Controls::{Button, ComboBox, Page},
    UIElement,
};
use std::{collections::HashMap, fs::File};
use xml::{attribute::OwnedAttribute, reader::XmlEvent, EventReader};

pub fn parse(file: &str, page: &mut Page, elements: &mut HashMap<String, Element>) {
    let file = File::open(file).unwrap();
    let mut reader = EventReader::new(file);
    loop {
        match reader.next().unwrap() {
            XmlEvent::StartDocument { .. } => {}
            XmlEvent::EndDocument => break,
            XmlEvent::StartElement {
                name,
                attributes,
                namespace: _,
            } => {
                let element =
                    generate_element(elements, &mut reader, &name.local_name, attributes).unwrap();
                page.SetContent(element).unwrap();
            }
            x => {
                println!("Ignored: {:?}", x);
            }
        }
    }
}

pub(self) mod prelude {
    pub use super::utils::*;
    pub use super::Element;
    pub use bindings::Windows::UI::Xaml::UIElement;
    pub use std::collections::HashMap;
    pub use xml::{attribute::OwnedAttribute, reader::XmlEvent};

    pub type Reader = xml::EventReader<std::fs::File>;
}

mod button;
mod combo;
mod grid;
mod stack;
mod textblock;

fn generate_element(
    elements: &mut HashMap<String, Element>,
    reader: &mut EventReader<File>,
    name: &str,
    attributes: Vec<OwnedAttribute>,
) -> windows::Result<UIElement> {
    match name {
        "Grid" => grid::build(elements, reader, attributes),
        "Button" => button::build(elements, reader, attributes),
        "Text" => textblock::build(elements, reader, attributes),
        "Stack" => stack::build(elements, reader, attributes),
        "Combo" => combo::build(elements, reader, attributes),
        x => {
            panic!("Unknown element: {:?}", x);
        }
    }
}

pub enum Element {
    Button(Button),
    ComboBox(ComboBox),
}

impl Element {
    pub fn as_button(&self) -> Option<Button> {
        match self {
            Self::Button(b) => Some(b.clone()),
            _ => None,
        }
    }
    pub fn as_combo_box(&self) -> Option<ComboBox> {
        match self {
            Self::ComboBox(c) => Some(c.clone()),
            _ => None,
        }
    }
}

impl From<Button> for Element {
    fn from(b: Button) -> Self {
        Self::Button(b)
    }
}

impl From<ComboBox> for Element {
    fn from(c: ComboBox) -> Self {
        Self::ComboBox(c)
    }
}

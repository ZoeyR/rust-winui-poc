use super::prelude::*;
use bindings::Windows::UI::Xaml::Controls::ComboBox;
use std::convert::TryInto;
use windows::{HString, Object};

pub fn build(
    elements: &mut HashMap<String, Element>,
    reader: &mut Reader,
    attributes: Vec<OwnedAttribute>,
) -> windows::Result<UIElement> {
    let combo = ComboBox::new()?;
    for attr in attributes {
        match attr.name.local_name.as_str() {
            "id" => {
                elements.insert(attr.value.clone(), combo.clone().into());
            }
            "font_size" => combo.SetFontSize(parse_f64(&attr.value)?)?,
            "width" => combo.SetWidth(parse_f64(&attr.value)?)?,
            "halign" => combo.SetHorizontalAlignment(parse_horizontal_align(&attr.value)?)?,
            x => panic!("Unknown combo attribute: {:?}", x),
        }
    }
    loop {
        match reader.next().unwrap() {
            XmlEvent::EndElement { .. } => break,
            x => panic!("Unknown combo child: {:?}", x),
        }
    }
    Ok(combo.into())
}

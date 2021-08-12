use super::prelude::*;
use bindings::Windows::UI::Xaml::Controls::{Grid, StackPanel};

pub fn build(
    elements: &mut HashMap<String, Element>,
    reader: &mut Reader,
    attributes: Vec<OwnedAttribute>,
) -> windows::Result<UIElement> {
    let stack = StackPanel::new()?;
    for attr in attributes {
        match attr.name.local_name.as_str() {
            "halign" => stack.SetHorizontalAlignment(parse_horizontal_align(&attr.value)?)?,
            "valign" => stack.SetVerticalAlignment(parse_vertical_align(&attr.value)?)?,
            "row" => {
                let row = parse_i32(&attr.value)?;
                Grid::SetRow(&stack, row)?;
            }
            "col" => {
                let column = parse_i32(&attr.value)?;
                Grid::SetColumn(&stack, column)?;
            }
            x => panic!("Unknown stack attribute: {:?}", x),
        }
    }
    loop {
        match reader.next().unwrap() {
            XmlEvent::EndElement { .. } => break,
            XmlEvent::StartElement {
                name,
                attributes,
                namespace: _,
            } => {
                let child =
                    super::generate_element(elements, reader, &name.local_name, attributes)?;
                stack.Children()?.Append(child)?;
            }
            XmlEvent::Whitespace(_) => {}
            x => panic!("Unknown stack child: {:?}", x),
        }
    }
    Ok(stack.into())
}

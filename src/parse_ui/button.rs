use bindings::Windows::UI::Xaml::Controls::Button;
use super::prelude::*;

pub fn build(
    elements: &mut HashMap<String, Element>,
    reader: &mut Reader,
    attributes: &[OwnedAttribute],
) -> windows::Result<UIElement>
{
    let button = Button::new()?;
    for attr in attributes {
        match attr {
            x => panic!("Unknown button attribute: {:?}", x.name.local_name),
        }
    }
    loop {
        match reader.next().unwrap() {
            XmlEvent::EndElement { .. } => break,
            x => panic!("Unknown button child: {:?}", x),
        }
    }
    Ok(button.into())
}


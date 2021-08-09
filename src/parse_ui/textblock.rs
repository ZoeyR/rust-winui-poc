use bindings::Windows::UI::Xaml::Controls::TextBlock;

use super::prelude::*;

pub fn build(
    elements: &mut HashMap<String, Element>,
    reader: &mut Reader,
    attributes: &[OwnedAttribute],
) -> windows::Result<UIElement>
{
    let textblock = TextBlock::new()?;
    for attr in attributes {
        match attr {
            x => panic!("Unknown textblock attribute: {:?}", x.name.local_name),
        }
    }
    loop {
        match reader.next().unwrap() {
            XmlEvent::EndElement { .. } => break,
            x => panic!("Unknown button child: {:?}", x),
        }
    }
    Ok(textblock.into())
}
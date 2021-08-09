use bindings::Windows::UI::Xaml::{Controls::{Grid, RowDefinition}, GridLength, GridUnitType};
use super::prelude::*;

pub fn build(
    elements: &mut HashMap<String, Element>,
    reader: &mut Reader,
    attributes: &[OwnedAttribute],
) -> windows::Result<UIElement>
{
    let grid = Grid::new()?;
    for attr in attributes {
        match attr {
            x => panic!("Unknown grid attribute: {:?}", x.name.local_name),
        }
    }
    loop {
        match reader.next().unwrap() {
            XmlEvent::EndElement { .. } => break,
            XmlEvent::StartElement { name, attributes, namespace: _ } => {
                if name.local_name == "row" {
                    let row = build_row(&attributes)?;
                    grid.RowDefinitions()?.Append(&row)?;
                    assert!(matches!(reader.next().unwrap(), XmlEvent::EndElement { .. }));
                } else {
                    let element = super::generate_element(elements, reader, &name.local_name, &attributes)?;
                    grid.Children()?.Append(element)?;
                }
            },
            XmlEvent::Whitespace(_) => {},
            x => panic!("Unknown button child: {:?}", x),
        }
    }
    Ok(grid.into())
}

fn build_row(attributes: &[OwnedAttribute]) -> windows::Result<RowDefinition> {
    let row = RowDefinition::new()?;
    for attr in attributes {
        match attr.name.local_name.as_str() {
            "height" => {
                let height = match grid_length(&attr.value) {
                    Some(height) => height,
                    None => panic!("Unknown grid unit type {:?}", attr.value)
                };
                row.SetHeight(height)?;
            },
            x => panic!("Unknown row attribute: {:?}", x),
        }
    }

    Ok(row)
}

fn grid_length(length: &str) -> Option<GridLength> {
    match length {
        "*" => Some(GridLength { Value: 1.0, GridUnitType: GridUnitType::Star }),
        "auto" => Some(GridLength { Value: 1.0, GridUnitType: GridUnitType::Auto }),
        x  => {
            let val: f64 = x.parse().ok()?;
            Some(GridLength { Value: val, GridUnitType: GridUnitType::Pixel })
        },
    }
}
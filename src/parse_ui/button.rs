use super::prelude::*;
use bindings::Windows::UI::Xaml::{
    Controls::{Button, Grid},
    Thickness,
};

pub fn build(
    elements: &mut HashMap<String, Element>,
    reader: &mut Reader,
    attributes: Vec<OwnedAttribute>,
) -> windows::Result<UIElement> {
    let button = Button::new()?;
    let mut margin = Thickness::default();
    for attr in attributes {
        match attr.name.local_name.as_str() {
            "id" => {
                elements.insert(attr.value.clone(), button.clone().into());
            }
            "margin_bottom" => margin.Bottom = parse_f64(&attr.value)?,
            "margin_right" => margin.Right = parse_f64(&attr.value)?,
            "margin_left" => margin.Left = parse_f64(&attr.value)?,
            "margin_top" => margin.Top = parse_f64(&attr.value)?,
            "margin" => margin = parse_margin(&attr.value)?,
            "row" => {
                let row = parse_i32(&attr.value)?;
                Grid::SetRow(&button, row)?;
            }
            "min_width" => button.SetMinWidth(parse_f64(&attr.value)?)?,
            "enabled" => button.SetIsEnabled(parse_boolean(&attr.value)?)?,
            "halign" => button.SetHorizontalAlignment(parse_horizontal_align(&attr.value)?)?,
            "valign" => button.SetVerticalAlignment(parse_vertical_align(&attr.value)?)?,
            "col" => {
                let column = parse_i32(&attr.value)?;
                Grid::SetColumn(&button, column)?;
            }
            x => panic!("Unknown button attribute: {:?}", x),
        }
    }
    button.SetMargin(margin)?;
    loop {
        match reader.next().unwrap() {
            XmlEvent::EndElement { .. } => break,
            XmlEvent::Characters(content) => {
                button.SetContent(get_inspectable(content.trim())?)?;
            }
            x => panic!("Unknown button child: {:?}", x),
        }
    }
    Ok(button.into())
}

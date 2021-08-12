use bindings::Windows::UI::Xaml::{
    Controls::{Grid, TextBlock},
    Thickness,
};

use super::prelude::*;

pub fn build(
    _elements: &mut HashMap<String, Element>,
    reader: &mut Reader,
    attributes: Vec<OwnedAttribute>,
) -> windows::Result<UIElement> {
    let textblock = TextBlock::new()?;
    let mut margin = Thickness::default();
    for attr in attributes {
        match attr.name.local_name.as_str() {
            "font_size" => textblock.SetFontSize(parse_f64(&attr.value)?)?,
            "halign" => textblock.SetHorizontalAlignment(parse_horizontal_align(&attr.value)?)?,
            "margin_bottom" => margin.Bottom = parse_f64(&attr.value)?,
            "margin_right" => margin.Right = parse_f64(&attr.value)?,
            "margin_left" => margin.Left = parse_f64(&attr.value)?,
            "margin_top" => margin.Top = parse_f64(&attr.value)?,
            "margin" => margin = parse_margin(&attr.value)?,
            "row" => {
                let row = parse_i32(&attr.value)?;
                Grid::SetRow(&textblock, row)?;
            }
            "col" => {
                let column = parse_i32(&attr.value)?;
                Grid::SetColumn(&textblock, column)?;
            }
            x => panic!("Unknown textblock attribute: {:?}", x),
        }
    }
    textblock.SetMargin(margin)?;
    loop {
        match reader.next().unwrap() {
            XmlEvent::EndElement { .. } => break,
            XmlEvent::Characters(content) => {
                textblock.SetText(content.trim())?;
            }
            x => panic!("Unknown button child: {:?}", x),
        }
    }
    Ok(textblock.into())
}

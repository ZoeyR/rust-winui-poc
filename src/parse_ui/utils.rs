use bindings::Windows::UI::Xaml::{HorizontalAlignment, Thickness, VerticalAlignment};
use std::convert::TryInto;
use windows::{Error, HString, Object, Result, HRESULT};

pub fn parse_f64(s: &str) -> Result<f64> {
    s.parse().map_err(|e| {
        Error::new(
            HRESULT(0x80070057), // E_INVALIDARG
            &format!("Could not parse {:?} as a value number: {:?}", s, e),
        )
    })
}

pub fn parse_i32(s: &str) -> Result<i32> {
    s.parse().map_err(|e| {
        Error::new(
            HRESULT(0x80070057), // E_INVALIDARG
            &format!("Could not parse {:?} as a value number: {:?}", s, e),
        )
    })
}

pub fn parse_horizontal_align(s: &str) -> Result<HorizontalAlignment> {
    let s = s.to_lowercase();
    match s.trim() {
        "left" => Ok(HorizontalAlignment::Left),
        "center" => Ok(HorizontalAlignment::Center),
        "right" => Ok(HorizontalAlignment::Right),
        "stretch" => Ok(HorizontalAlignment::Stretch),
        _ => {
            Err(Error::new(
                HRESULT(0x80070057), // E_INVALIDARG
                &format!(
                    "Invalid halign value: {:?}, expected left, center, right or stretch",
                    s
                ),
            ))
        }
    }
}

pub fn parse_boolean(s: &str) -> Result<bool> {
    let s = s.to_lowercase();
    match s.trim() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => {
            Err(Error::new(
                HRESULT(0x80070057), // E_INVALIDARG
                &format!("Invalid boolean value: {:?}, expected true or false", s),
            ))
        }
    }
}

pub fn parse_vertical_align(s: &str) -> Result<VerticalAlignment> {
    let s = s.to_lowercase();
    match s.trim() {
        "top" => Ok(VerticalAlignment::Top),
        "center" => Ok(VerticalAlignment::Center),
        "bottom" => Ok(VerticalAlignment::Bottom),
        "stretch" => Ok(VerticalAlignment::Stretch),
        _ => {
            Err(Error::new(
                HRESULT(0x80070057), // E_INVALIDARG
                &format!(
                    "Invalid valign value: {:?}, expected top, center, bottom or stretch",
                    s
                ),
            ))
        }
    }
}

pub fn parse_margin(s: &str) -> Result<Thickness> {
    let parts: std::result::Result<Vec<f64>, _> = s.split(',').map(|s| s.trim().parse()).collect();
    match parts {
        Ok(parts) if parts.len() == 4 => Ok(Thickness {
            Left: parts[0],
            Top: parts[1],
            Right: parts[2],
            Bottom: parts[3],
        }),
        _ => Err(Error::new(
            HRESULT(0x80070057), // E_INVALIDARG
            &format!(
                "Invalid margin value: {:?}, expected 4 values seperated by comma",
                s
            ),
        )),
    }
}

pub fn get_text_object<T>(text: T) -> Result<Object>
where
    HString: From<T>,
{
    HString::from(text).try_into()
}

use bindings::Windows::UI::Xaml::Controls::*;
use bindings::Windows::UI::Xaml::*;
use windows::{HString, Object};

use std::convert::TryInto;

pub struct CompletionPage {
    root: Page,
    reboot_button: Button,
    reboot_later_button: Button,
}

impl CompletionPage {
    pub fn new() -> windows::Result<CompletionPage> {
        let page = CompletionPage {
            root: Page::new()?,
            reboot_button: Button::new()?,
            reboot_later_button: Button::new()?,
        };

        init(&page)?;
        Ok(page)
    }

    pub fn page(&self) -> &Page {
        &self.root
    }
}

fn init(page: &CompletionPage) -> windows::Result<()> {
    layout_buttons(&page.reboot_button, &page.reboot_later_button)?;
    let stack = create_stack()?;
    let grid = create_grid(&stack, &page.reboot_button, &page.reboot_later_button)?;

    page.root.SetContent(grid)?;

    Ok(())
}

fn create_grid(
    stack: &StackPanel,
    reboot_button: &Button,
    reboot_later_button: &Button,
) -> windows::Result<Grid> {
    let grid = Grid::new()?;
    let row_1 = RowDefinition::new()?;
    row_1.SetHeight(GridLength {
        Value: 1.0,
        GridUnitType: GridUnitType::Star,
    })?;

    let row_2 = RowDefinition::new()?;
    row_2.SetHeight(GridLength {
        Value: 1.0,
        GridUnitType: GridUnitType::Auto,
    })?;

    grid.RowDefinitions()?.Append(&row_1)?;
    grid.RowDefinitions()?.Append(&row_2)?;

    grid.Children()?.Append(stack)?;
    grid.Children()?.Append(reboot_button)?;
    grid.Children()?.Append(reboot_later_button)?;

    Ok(grid)
}

fn create_stack() -> windows::Result<StackPanel> {
    let stack = StackPanel::new()?;
    stack.SetVerticalAlignment(VerticalAlignment::Center)?;
    Grid::SetRow(&stack, 0)?;

    let title = TextBlock::new().unwrap();
    title.SetFontSize(21.0)?;
    title.SetText("That's it!")?;
    title.SetHorizontalAlignment(HorizontalAlignment::Center)?;

    let subtitle = TextBlock::new().unwrap();
    subtitle.SetFontSize(11.0)?;
    subtitle.SetText("Your language resources are now installed. To finish setup you will need to restart your computer. You can either restart now or later.")?;
    subtitle.SetHorizontalAlignment(HorizontalAlignment::Center)?;
    subtitle.SetTextWrapping(TextWrapping::Wrap)?;
    subtitle.SetMargin(Thickness {
        Left: 32.0,
        Top: 16.0,
        Right: 32.0,
        Bottom: 64.0,
    })?;

    stack.Children()?.Append(&title)?;
    stack.Children()?.Append(&subtitle)?;

    Ok(stack)
}

fn layout_buttons(reboot_button: &Button, reboot_later_button: &Button) -> windows::Result<()> {
    let objtext: Object = HString::from("Reboot Now").try_into()?;
    reboot_button.SetContent(objtext)?;
    reboot_button.SetHorizontalAlignment(HorizontalAlignment::Right)?;
    reboot_button.SetVerticalAlignment(VerticalAlignment::Bottom)?;
    reboot_button.SetMargin(Thickness {
        Left: 12.0,
        Top: 8.0,
        Right: 12.0,
        Bottom: 8.0,
    })?;
    reboot_button.SetMinWidth(80.0)?;
    reboot_button.SetIsEnabled(true)?;
    Grid::SetRow(&*reboot_button, 1)?;

    let objtext: Object = HString::from("Reboot Later").try_into()?;
    reboot_later_button.SetContent(objtext)?;
    reboot_later_button.SetHorizontalAlignment(HorizontalAlignment::Left)?;
    reboot_later_button.SetVerticalAlignment(VerticalAlignment::Bottom)?;
    reboot_later_button.SetMargin(Thickness {
        Left: 12.0,
        Top: 8.0,
        Right: 12.0,
        Bottom: 8.0,
    })?;
    reboot_later_button.SetMinWidth(80.0)?;
    reboot_later_button.SetIsEnabled(true)?;
    Grid::SetRow(&*reboot_later_button, 1)?;

    Ok(())
}

use bindings::Windows::UI::Xaml::Controls::*;
use bindings::Windows::UI::Xaml::Hosting::*;
use bindings::Windows::UI::Xaml::*;
use std::convert::TryInto;
use windows::IInspectable;
use windows::HSTRING;

pub struct LandingPage {
    root: Page,
    combobox: ComboBox,
    install_button: Button,
}

impl LandingPage {
    pub fn new(ctx: &DesktopWindowXamlSource) -> windows::Result<LandingPage> {
        let page = LandingPage {
            root: Page::new()?,
            combobox: ComboBox::new()?,
            install_button: Button::new()?,
        };

        init(&page)?;
        hook_event_handlers(ctx, &page)?;
        Ok(page)
    }

    pub fn page(&self) -> &Page {
        &self.root
    }
}

fn hook_event_handlers(ctx: &DesktopWindowXamlSource, page: &LandingPage) -> windows::Result<()> {
    let combo = &page.combobox;
    let button = page.install_button.clone();
    combo.SelectionChanged(SelectionChangedEventHandler::new(move |_, _| {
        button.SetIsEnabled(true)
    }))?;

    let ctx = ctx.clone();
    page.install_button
        .Click(RoutedEventHandler::new(move |_, _| {
            let next = super::landing_v2::LandingV2::new(&ctx)?;
            ctx.SetContent(next.page())
        }))?;
    Ok(())
}

fn init(page: &LandingPage) -> windows::Result<()> {
    layout_install_button(&page.install_button)?;
    let stack = create_stack(&page.combobox)?;
    let grid = create_grid(&stack, &page.install_button)?;

    page.root.SetContent(grid)?;

    Ok(())
}

fn create_grid(stack: &StackPanel, button: &Button) -> windows::Result<Grid> {
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
    grid.Children()?.Append(button)?;

    Ok(grid)
}

fn create_stack(combo: &ComboBox) -> windows::Result<StackPanel> {
    let stack = StackPanel::new()?;
    stack.SetVerticalAlignment(VerticalAlignment::Center)?;
    Grid::SetRow(&stack, 0)?;

    let title = TextBlock::new().unwrap();
    title.SetFontSize(24.0)?;
    title.SetText("Old\nSelect Language to Install:")?;
    title.SetHorizontalAlignment(HorizontalAlignment::Center)?;
    title.SetMargin(Thickness {
        Left: 0.0,
        Top: 0.0,
        Right: 0.0,
        Bottom: 64.0,
    })?;

    combo.SetFontSize(17.0)?;
    combo.SetWidth(300.0)?;
    combo.SetHorizontalAlignment(HorizontalAlignment::Center)?;
    for lang in create_langs() {
        let objtext: IInspectable = HSTRING::from(lang).try_into()?;
        combo.Items()?.Append(objtext)?;
    }

    stack.Children()?.Append(&title)?;
    stack.Children()?.Append(&*combo)?;

    Ok(stack)
}

fn layout_install_button(button: &Button) -> windows::Result<()> {
    let objtext: IInspectable = HSTRING::from("Install").try_into()?;
    button.SetContent(objtext)?;
    button.SetHorizontalAlignment(HorizontalAlignment::Right)?;
    button.SetVerticalAlignment(VerticalAlignment::Bottom)?;
    button.SetMargin(Thickness {
        Left: 8.0,
        Top: 8.0,
        Right: 12.0,
        Bottom: 8.0,
    })?;
    button.SetMinWidth(80.0)?;
    button.SetIsEnabled(false)?;
    Grid::SetRow(&*button, 1)?;

    Ok(())
}

fn create_langs() -> Vec<&'static str> {
    let tags = vec!["fo", "se", "sma", "smj", "smn", "sms", "crk", "srs"];
    tags.iter()
        .map(|s| {
            let record = iso639::autonym::get(s).unwrap();
            match record.autonym {
                Some(auto) => auto,
                None => record.name,
            }
        })
        .collect()
}

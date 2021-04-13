use bindings::Windows::UI::Core::*;
use bindings::Windows::UI::Xaml::Controls::*;
use bindings::Windows::UI::Xaml::Hosting::*;
use bindings::Windows::UI::Xaml::*;
use std::convert::TryInto;
use windows::{HString, Object};

pub struct DownloadPage {
    root: Page,
    progress: ProgressBar,
    progress_text: TextBlock,
    cancel_button: Button,
}

impl DownloadPage {
    pub fn new(ctx: &DesktopWindowXamlSource) -> windows::Result<DownloadPage> {
        let page = DownloadPage {
            root: Page::new()?,
            progress: ProgressBar::new()?,
            progress_text: TextBlock::new()?,
            cancel_button: Button::new()?,
        };

        init(&page)?;

        let progress = page.progress.clone();
        let dispatcher = Window::Current().unwrap().Dispatcher().unwrap();
        let ctx = ctx.clone();

        std::thread::spawn(move || {
            for n in 1..=10 {
                std::thread::sleep(std::time::Duration::from_millis(1000));
                let progress = progress.clone();
                dispatcher
                    .RunAsync(
                        CoreDispatcherPriority::Low,
                        DispatchedHandler::new(move || progress.SetValue2(10.0 * (n as f64))),
                    )
                    .unwrap();
            }

            std::thread::sleep(std::time::Duration::from_millis(1000));
            dispatcher
                .RunAsync(
                    CoreDispatcherPriority::Low,
                    DispatchedHandler::new(move || {
                        let completion_page = super::completion::CompletionPage::new()?;
                        ctx.SetContent(completion_page.page())
                    }),
                )
                .unwrap();
        });

        Ok(page)
    }

    pub fn page(&self) -> &Page {
        &self.root
    }
}

fn init(page: &DownloadPage) -> windows::Result<()> {
    let grid = Grid::new()?;
    let row_1 = RowDefinition::new()?;
    row_1.SetHeight(GridLength {
        Value: 1.0,
        GridUnitType: GridUnitType::Auto,
    })?;

    let row_2 = RowDefinition::new()?;
    row_2.SetHeight(GridLength {
        Value: 1.0,
        GridUnitType: GridUnitType::Auto,
    })?;

    let row_3 = RowDefinition::new()?;
    row_3.SetHeight(GridLength {
        Value: 32.0,
        GridUnitType: GridUnitType::Pixel,
    })?;

    let row_4 = RowDefinition::new()?;
    row_4.SetHeight(GridLength {
        Value: 1.0,
        GridUnitType: GridUnitType::Star,
    })?;

    grid.RowDefinitions()?.Append(&row_1)?;
    grid.RowDefinitions()?.Append(&row_2)?;
    grid.RowDefinitions()?.Append(&row_3)?;
    grid.RowDefinitions()?.Append(&row_4)?;

    let title = TextBlock::new()?;
    title.SetFontSize(24.0)?;
    title.SetText("Downloading...")?;
    title.SetHorizontalAlignment(HorizontalAlignment::Center)?;
    title.SetVerticalAlignment(VerticalAlignment::Center)?;
    title.SetMargin(Thickness {
        Left: 0.0,
        Top: 48.0,
        Right: 0.0,
        Bottom: 16.0,
    })?;
    Grid::SetRow(&title, 0)?;

    let subtitle = TextBlock::new()?;
    subtitle.SetText("downloading language resources")?;
    subtitle.SetHorizontalAlignment(HorizontalAlignment::Center)?;
    subtitle.SetVerticalAlignment(VerticalAlignment::Center)?;
    subtitle.SetMargin(Thickness {
        Left: 48.0,
        Top: 0.0,
        Right: 48.0,
        Bottom: 64.0,
    })?;
    Grid::SetRow(&subtitle, 1)?;

    let stack = StackPanel::new()?;
    Grid::SetRow(&stack, 2)?;

    let progress = &page.progress;
    progress.SetMargin(Thickness {
        Left: 48.0,
        Top: 0.0,
        Right: 48.0,
        Bottom: 0.0,
    })?;
    progress.SetMinimum(0.0)?;
    progress.SetMaximum(100.0)?;

    let progress_text = &page.progress_text;
    progress_text.SetText("Here is progress text")?;
    progress_text.SetHorizontalAlignment(HorizontalAlignment::Left)?;
    progress_text.SetVerticalAlignment(VerticalAlignment::Center)?;
    progress_text.SetMargin(Thickness {
        Left: 48.0,
        Top: 0.0,
        Right: 48.0,
        Bottom: 0.0,
    })?;

    stack.Children()?.Append(&*progress)?;
    stack.Children()?.Append(&*progress_text)?;

    initialize_cancel_button(page)?;

    grid.Children()?.Append(&title)?;
    grid.Children()?.Append(&subtitle)?;
    grid.Children()?.Append(&stack)?;
    grid.Children()?.Append(&page.cancel_button)?;

    page.root.SetContent(grid)?;

    Ok(())
}

fn initialize_cancel_button(page: &DownloadPage) -> windows::Result<()> {
    let button = &page.cancel_button;
    let objtext: Object = HString::from("Cancel").try_into()?;
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
    button.SetIsEnabled(true)?;
    Grid::SetRow(&*button, 3)?;

    Ok(())
}

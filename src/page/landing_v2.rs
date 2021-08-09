use bindings::Windows::UI::Xaml::{
    Controls::*, Hosting::DesktopWindowXamlSource, RoutedEventHandler,
};
use std::collections::HashMap;

pub struct LandingV2 {
    page: Page,
}

impl LandingV2 {
    pub fn new(ctx: &DesktopWindowXamlSource) -> windows::Result<Self> {
        let mut page = Page::new()?;
        let mut elements = HashMap::new();
        crate::parse_ui::parse("ui/landing.xml", &mut page, &mut elements);

        let button = elements["install_button"].as_button().unwrap();
        let combo = elements["combo"].as_combo_box().unwrap();

        {
            let button = button.clone();
            combo.SelectionChanged(SelectionChangedEventHandler::new(move |_, _| {
                button.SetIsEnabled(true)
            }))?;
        }
        {
            let ctx = ctx.clone();
            button.Click(RoutedEventHandler::new(move |_, _| {
                let next = super::landing::LandingPage::new(&ctx)?;
                ctx.SetContent(next.page())
            }))?;
        }

        Ok(LandingV2 { page })
    }

    pub fn page(&self) -> &Page {
        &self.page
    }
}

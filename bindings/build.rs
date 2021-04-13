fn main() {
    windows::build!(
            Windows::UI::Xaml::Hosting::{DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory, WindowsXamlManager},
            Windows::UI::Xaml::Controls::{Button, Grid, Page, ComboBox, TextBlock, UIElementCollection, RowDefinitionCollection, ItemCollection, StackPanel, SelectionChangedEventHandler, ProgressBar},
            Windows::UI::Xaml::{GridUnitType, GridLength, VerticalAlignment, HorizontalAlignment, Thickness, RoutedEventHandler},
    );
}
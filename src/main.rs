mod deck;
mod card;
mod view;

use view::View;

fn main()
{
    let app = View ::default();
    let native_options = eframe::NativeOptions {resizable: false,..eframe::NativeOptions::default()};
    eframe::run_native(Box::new(app), native_options)
}
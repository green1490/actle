use eframe::{egui,epi};

struct TemplateApp {
    // Example stuff:
    label: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned()
        }
    }
}

impl epi::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) 
    {
        ctx.set_debug_on_hover(true);
        
        egui::SidePanel::left("side panel")
        .show(ctx,|ui|
        {
        });


        egui::CentralPanel::default().show(ctx, |ui|
        {
            if ui.button("click me").clicked()
            {
                println!("yes");
            }
        });

    }


    fn name(&self) -> &str {
        self.label.as_str()
    }
}

fn main()
{
    let app = TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options)
}
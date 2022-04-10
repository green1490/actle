//refactor
//üres textet ne lehessen megadni
use eframe::{egui::{self},epi};

struct TemplateApp  
{
    title: String,
    button_label:String,
    labels:Vec<String>
}

impl Default  for TemplateApp  {
    fn default() -> Self {
        Self {
            title: "Hello World!".to_owned(),
            labels:Vec::new(),
            button_label:String::new()
        }
    }
}

impl epi::App  for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) 
    {
           
        egui::SidePanel::left("side panel")
        .show(ctx,|ui|
        {
            if ui.button("+").clicked()
            {
                if !self.labels.contains(&self.button_label.trim().to_string())
                {
                    let text = self.button_label.trim().to_string().to_owned();
                    if !text.is_empty()
                    {
                        self.labels.push(text);
                    }
                }
            }

            ui.add(egui::widgets::TextEdit::singleline(&mut self.button_label));
            

            ui.vertical(|ui| 
            {   
                ui.add(egui::widgets::Separator::default());
                
                for item in &self.labels
                {
                    if ui.button(item).clicked()
                    {
                        println!("{}",item);
                    }
                }
                
            });
        });

        egui::CentralPanel::default().show(ctx, |ui|
        {
            
        });

    }
    
    //load the states from a file
    fn setup(&mut self,_ctx: &egui::Context,_frame: &epi::Frame,_storage: Option<&dyn epi::Storage>)
    {
        
    }


    fn name(&self) -> &str {
        self.title.as_str()
    }

}

fn main()
{
    let app = TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options)
}
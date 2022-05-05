

use eframe::{egui,epi};
use serde::{Serialize,Deserialize};
use serde_json;
use super::deck::Deck;
use super::card::Card;

#[derive(Serialize, Deserialize)]
pub struct View  
{
    title: String,
    button_label:String,
    labels:Vec<String>,
    learning_widget:bool,
    question:String,
    answer:String,
    button_number:u8,
    selected_deck:String,
    decks:Vec<Deck>
}

impl Default  for View  {
    fn default() -> Self {
        Self {
            //The name of the app
            title: "Actle".to_owned(),
            //holds the side panel buttons
            labels:Vec::new(),
            //the new name of the button that will appear on the side panel
            button_label:String::new(),
            learning_widget: true,
            question:String::new(),
            answer:String::new(),
            //helps with if you are switching between learning and adding widget
            //you won't immediately add it.
            button_number:0,
            selected_deck:String::new(),
            decks:Vec::new()
        }
    }
}

impl epi::App  for View {
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) 
    {
        
        egui::SidePanel::left("side panel").max_width(300.0)
        .show(ctx,|ui|
        {
            ui.add_space(6.3);
            
            ui.horizontal(|ui|
            {
                if ui.button("+").clicked()
                {
                    if !self.labels.contains(&self.button_label.trim().to_string())
                    {
                        let text = self.button_label.trim().to_string().to_owned();
                        if !text.is_empty()
                        {
                            self.labels.push(text.to_owned());
                            self.button_label = String::from("");
                            self.decks.push(Deck { name: text.to_owned(), cards: Vec::new() });
                        }
                    }
                }
    
                ui.add(egui::widgets::TextEdit::singleline(&mut self.button_label));    
            });
            ui.vertical(|ui| 
            {   
                ui.add(egui::widgets::Separator::default());
                
                for item in &self.labels
                {
                    if ui.button(item).clicked()
                    {
                        self.selected_deck = item.to_owned();
                    }
                }
                
            });
        });

        egui::CentralPanel::default().show(ctx, |ui|
        {
            ui.with_layout(egui::Layout::from_main_dir_and_cross_align(
                egui::Direction::RightToLeft, egui::Align::Min
            ), |ui|
            {
                if ui.button("Add new card").clicked()
                {
                    self.learning_widget = false;
                    
                    if self.button_number >= 1 && 
                    (!self.question.trim().to_string().is_empty() 
                    && !self.answer.trim().to_string().is_empty())
                    {
                        //println!("{}",serde_json::to_string(&self).unwrap());
                        for item in &mut self.decks
                        {
                            if item.name == self.selected_deck
                            {
                                item.cards.push(Card { question: self.question.to_owned(), answer: self.question.to_owned() });
                                println!("{}",serde_json::to_string(&item).unwrap());
                            }
                        }
                    }

                    else
                    {
                        self.button_number += 1;
                    }
                }

                if ui.button("Learn cards").clicked()
                {
                    self.learning_widget = true;
                    self.button_number = 0;
                }
            });
            
            ui.separator();

            if !self.learning_widget
            {
                ui.vertical_centered(|ui|
                {
                    ui.text_edit_singleline(&mut self.question);
                    ui.text_edit_multiline(&mut self.answer);
                });
            }

            else 
            {
                //implement 
            }
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

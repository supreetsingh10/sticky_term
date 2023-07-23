use crate::backend::config::Config; 
use chrono::datetime;
use serde::{Deserialize, Serialize};
use tui::style::Color; 

// Do I need a initializer function before I parse the file. 
// The messages type should be initialized. 
// Messages vec will allow us to index the messages, and later write them on a file if needed. 
// The Mesage struct will hold all the key value pairs.
#[derive(Serialize, Deserialize, Debug, Clone,)]
pub struct Messages {
    pub messages: Vec<Message>
}


// Make a struct for every mesage. 
// Parse through key and value. 
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Message {
    message_id: u32,
    message_text: String,
    color: Color,
    x: u16,
    y: u16,
    date: datetime,
}


impl Message {
    pub fn get_message_id(&self) -> &u32 {
        &self.message_id
    }

    pub fn get_message_text(&self) -> &str {
        self.message_text.as_str()
    }

    pub fn get_note_color(&self) -> &tui::style::Color {
        &self.color 
    }

     pub fn get_x(&self) -> u16 {
         self.x
     }

     pub fn get_y(&self) -> u16 {
         self.y
     }

}

pub fn parse_config(config: Config) -> Result<Messages, String> {
    serde_json::from_str(config.get_config_text().as_str()).
        map_err(|e| e.to_string())
}



use serde::{Serialize,Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Card
{
    pub question:String,
    pub answer:String
}

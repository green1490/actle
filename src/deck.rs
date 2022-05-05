use serde::{Serialize,Deserialize};
use serde_json;
use super::card::Card;

#[derive(Serialize, Deserialize)]
pub struct Deck
{
    pub name: String,
    pub cards: Vec<Card>
}

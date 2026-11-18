use crate::the_crew::{player::Player,Card,Suit,PlayerId};

#[derive(Debug, Clone)]
pub struct Mission{
    pub description:&'static str,
    pub is_fulfilled:fn(&Vec<Player>,PlayerId)->bool,
    pub owned_by:Option<PlayerId>
    // failed_historics:fn(&Vec<Player>)->bool
}

pub const MISSION_G9:Mission = Mission{
		description: "Win green 9",
		is_fulfilled: |players:&Vec<Player>,owned_by: usize| {
			players[owned_by].m_tricks.iter().any(
				|t| {
					dbg!("{:?}",&t.cards);
					t.cards.contains(&Card::Suit(Suit::Green, 9))
				})
			},
		owned_by: None
};

use crate::the_crew::{Mission,Player,Card,Suit};

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

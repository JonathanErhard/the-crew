use rand::rng;
use rand::seq::SliceRandom;
use std::ops::{Deref, DerefMut};

use inline_colorization::*;
use std::fmt;

pub mod mission;
use mission::Mission;

pub mod player;
use player::{Player,PlayerView,PlayerInterface,TerminalInterface};

pub type PlayerId = usize;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Green,
    Blue,
    Pink,
    Yellow,
}

impl Suit {
    fn all() -> [Suit; 4] {
        [Suit::Green, Suit::Blue, Suit::Pink, Suit::Yellow]
    }
    
}


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Card {
    Suit(Suit, u8),
    Trump(u8),
}

impl Card{
    
    fn suit(self) -> Option<Suit> {
        match self {
            Card::Suit(s, _) => Some(s),
            Card::Trump(_) => None,
        }
    }

    fn trumps(&self, other_card:&Card) -> bool{
        match (self, other_card) {
            (Card::Trump(a), Card::Trump(b)) => a > b,
            (Card::Trump(_), Card::Suit(..)) => true,
            (Card::Suit(..), Card::Trump(_)) => false,
            (Card::Suit(s1, r1), Card::Suit(s2, r2)) => {
                if s1 == s2 {
                    return r1 > r2;
                }
                false
            },
        }
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        match self {
            Card::Suit(s, n) => {
               match s {
                Suit::Blue => {write!(f,"{color_blue}{}{color_reset}",n)},
                Suit::Green => {write!(f,"{color_green}{}{color_reset}",n)},
                Suit::Pink => {write!(f,"{color_magenta}{}{color_reset}",n)},
                Suit::Yellow => {write!(f,"{color_yellow}{}{color_reset}",n)}
               }
            },
            Card::Trump(n) => {
                write!(f,"{bg_white}{color_black}{}{color_reset}{bg_reset}",n)
            }
        }
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct CardStack{
    pub cards:Vec<Card>
}

impl fmt::Debug for CardStack{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:?}",self.cards)
    }
}

impl Deref for CardStack {
    type Target = Vec<Card>;
    fn deref(&self) -> &Vec<Card> {
        &self.cards
    }
}

impl DerefMut for CardStack {
    fn deref_mut(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trick{
    pub cards: CardStack,
    pub first_move:(Card,PlayerId),
    pub trick_number: u8
}

/* Struct that holds */
pub struct Game{
    pub m_num_players: usize,
    pub m_missions: Vec<Mission>,
    pub m_players: Vec<Player>,
}

impl Game{
    // function pushes <num_players> out of the woumb with shuffled cards in their hands
    fn deal(&mut self) -> (PlayerId){
        let mut deck: Vec<Card> = Vec::new();
        for s in Suit::all(){
            for i in 1..10{
                deck.push(Card::Suit(s,i));
            }
        }
        for i in 1..5{
            deck.push(Card::Trump(i));
        }
        deck.shuffle(&mut rng());
        let mut captain_id:usize=0;
        for i in 0..(self.m_num_players){
            // 40 is the dec size!
            let first_card:usize = (40+1)*i/self.m_num_players;
            let last_card:usize = (40*(i+1)/self.m_num_players-1)+1;
            let hand = CardStack { cards: deck[first_card..last_card].to_vec()};
            let mut is_captain = false;
            for card in &hand.cards{
                if let Card::Trump(4) = card{
                        is_captain = true;
                        captain_id = i;
                    }
            }
            self.m_players[i].m_is_captain = is_captain;
            self.m_players[i].m_hand = hand;
        }
        (captain_id)
    }

    pub fn add_player(&mut self,player_interface:Box<dyn PlayerInterface>){
            self.m_players.push(
            Player {
                m_id: self.m_num_players,
                m_hand: CardStack { cards: (Vec::new()) },
                m_tricks: Vec::new(),
                m_is_captain: false,
                m_has_communicated: false,
                m_is_communicating: None,
                interface: player_interface
            }
        );
    }

    fn get_valid_cards(hand:&CardStack, first_card:&Card) -> CardStack{
        let mut valid_cards: CardStack  = CardStack { cards: Vec::new() };
        let f_suit=first_card.suit();
        for card in &hand.cards{
            if card.suit() == f_suit{
                valid_cards.cards.push(*card);
            }
        }
        if valid_cards.cards.is_empty(){
            return hand.clone();
        }
        valid_cards
    }

    fn game_over(players: &Vec<Player>) -> bool{
        let mut ret:bool = false;
        for player in players{
            ret |= player.m_hand.cards.is_empty();
        }
        ret
    }

    fn next_trick(&mut self, starting_player_id: usize,trick_number:u8)->usize{
        let n = self.m_num_players;

        // Leader plays first. There is no trick yet, so the view gets None.
        let starting_player = &mut self.m_players[starting_player_id];
        let legal = starting_player.m_hand.cards.clone();
        let first_card = starting_player.choose_card(&starting_player.to_view(&None), &legal);
        assert!(legal.contains(&first_card), "interface returned an illegal card");
        self.m_players[starting_player_id].remove_card(first_card);

        let mut trick = Some(Trick {
            trick_number,
            cards: CardStack { cards: vec![first_card] },
            first_move: (first_card, starting_player_id),
        });
        let mut winner = (first_card, starting_player_id);

        for offset in 1..n {
            let player_id = (starting_player_id + offset) % n;
            let player = &mut self.m_players[player_id];

            let valid_cards = Game::get_valid_cards(&player.m_hand, &first_card);
            let card = &player.choose_card(&player.to_view(&trick), &valid_cards);
            assert!(valid_cards.contains(card), "interface returned an illegal card");
            player.remove_card(*card);

            trick.as_mut().unwrap().cards.cards.push(*card);
            if card.trumps(&winner.0) {
                winner = (*card, player_id);
            }
        }
        winner.1
    }

    fn choose_missions(&mut self, captain_id: usize) {
        let n = self.m_num_players;
        let total = self.m_missions.len();
        let base = total / n;
        let mut ceil_slots = total % n; // players who still must/may take base + 1

        for turn in 0..n {
            let player_id = 0;
            let players_after = n - turn - 1;

            let mut allowed: Vec<usize> = Vec::new();
            if ceil_slots <= players_after {
                allowed.push(base);
            }
            if ceil_slots > 0 {
                allowed.push(base + 1);
            }

                let player = &(self.m_players[player_id]);
                let player_view = player.to_view(&None);
                let picked = player.interface.deref().choose_mission(&player_view, &allowed, &self.m_missions,);

            if picked.len() == base + 1 {
                ceil_slots -= 1;
            }
            for i in picked {
                self.m_missions[i - 1].owned_by = Some(player_id);
            }
        }
}


    /*
    1. deal cards
    2. choose missions
    3. play out tricks
    4. check if missions are satisfied
     */

    pub fn play(&mut self)->bool{
        for i in 0..self.m_num_players{
                self.add_player(Box::new(TerminalInterface));
        }
        let tricknumber:u8 = 1;
        let captain_id = Game::deal(self);

        self.choose_missions(captain_id);

        let mut last_winner_id: usize = captain_id;
        while !Game::game_over(&self.m_players){
            last_winner_id = Game::next_trick(self, last_winner_id,tricknumber);
        }
        self.m_missions.iter()
            .all(|m| (m.is_fulfilled)(&self.m_players,m.owned_by.expect("Card not selected")))
    }

}
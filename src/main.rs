mod the_crew;

use the_crew::mission::MISSION_G9;
use the_crew::player::PlayerInterface;

use crate::the_crew::player::TerminalInterface;

fn main() {
    let missions = vec![MISSION_G9];
    let mut game = the_crew::Game{m_num_players:3,m_missions:missions,m_players:Vec::new()};
    if game.play(){
        print!("WOWZEE YOU DID IT!")
        } else {
            print!("YOU ARE A FAILURE AND YOU WILL DIE SAD AND ALONE!")
        }
}

/*
    let g = |n| Card::Suit(Suit::Green, n);
    let b = |n| Card::Suit(Suit::Blue, n);
    let p = |n| Card::Suit(Suit::Pink, n);
    let y = |n| Card::Suit(Suit::Yellow, n);
    let t = |n| Card::Trump(n);

    // (starting player, cards in play order)
    let tricks: [(usize, [Card; 4]); 10] = [
        (0, [g(5), g(9), g(2), g(7)]), // p1 wins (G9)
        (1, [b(3), b(8), b(6), b(1)]), // p2 wins (B8)
        (2, [p(4), p(1), t(2), p(9)]), // p0 trumps in (T2)
        (0, [y(7), y(3), y(9), y(5)]), // p2 wins (Y9)
        (2, [g(1), g(8), g(3), g(4)]), // p3 wins (G8)
        (3, [p(7), t(4), p(5), p(6)]), // p0 trumps in (T4)
        (0, [b(5), b(2), b(9), b(4)]), // p2 wins (B9)
        (2, [y(8), y(4), t(1), y(6)]), // p0 trumps in (T1)
        (0, [g(6), y(1), t(3), p(2)]), // p2 trumps in (T3)
        (2, [p(3), p(8), b(7), y(2)]), // p3 wins (P8)
    ];

    // 4 players, hands already empty (end of the game)
    let mut players: Vec<Player> = (0..4)
        .map(|id| Player {
            id,
            hand: CardStack { cards: Vec::new() },
            tricks: Vec::new(),
            is_captain: id == 0, // p0 played Trump 4
            has_communicated: false,
            is_communicating: None,
        })
        .collect();

    for (n, (starter, cards)) in tricks.iter().enumerate() {
        // Find the winner using Card::trumps
        let mut winner = (cards[0], *starter);
        for (i, &card) in cards.iter().enumerate().skip(1) {
            if card.trumps(&winner.0) {
                winner = (card, (starter + i) % 4);
            }
        }

        let trick = Trick {
            cards: CardStack { cards: cards.to_vec() },
            first_move: (cards[0], *starter),
            trick_number: (n + 1) as u8,
        };
        println!("trick {:2}: starter {} -> {:?}, winner: player {}", n + 1, starter, trick.cards, winner.1);
        players[winner.1].tricks.push(trick);
    }

    print!("{:?}",(mission1.is_fulfilled)(&players,mission1.owned_by));
 */
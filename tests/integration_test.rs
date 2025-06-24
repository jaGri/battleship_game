use tokio::runtime::Runtime;
use transport::adapters::InMemTransport;
use transport::ReliableTransport;
use core::{GameState, PlayerId};
use core::message::Message;
use player::ProbAi;
use player::Difficulty;

#[test]
fn play_full_game_inmem_ai() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let (t1, t2) = InMemTransport::pair(16);
        let mut p1 = ReliableTransport::new(t1);
        let mut p2 = ReliableTransport::new(t2);
        let mut gs1 = GameState::new(5,5);
        let mut gs2 = GameState::new(5,5);
        let mut ai1 = ProbAi::new(5,5,Difficulty::Hard, PlayerId::One);
        let mut ai2 = ProbAi::new(5,5,Difficulty::Hard, PlayerId::Two);

        // Handshake
        p1.send(Message::Handshake { player: PlayerId::One }).await.unwrap();
        p2.send(Message::Handshake { player: PlayerId::Two }).await.unwrap();

        // Simulate one attack each
        let m1 = ai1.next_move(&gs1).await;
        p1.send(Message::Attack { x: m1.x, y: m1.y }).await.unwrap();
        if let Message::Attack { x,y } = p2.recv().await.unwrap() {
            let (hit, sunk) = gs2.receive_attack(PlayerId::One, x,y);
            p2.send(Message::AttackResult { hit, sunk }).await.unwrap();
        }

        assert!(!gs1.is_game_over());
        assert!(!gs2.is_game_over());
    });
}

use async_trait::async_trait;
use transport::ReliableTransport;
use battleship_core::state::{GameState, PlayerId, Phase, Orientation};
use battleship_core::ship::ShipType;
use battleship_core::message::Message;
use interface_cli::{InputProvider, OutputRenderer, InputEvent};
use std::io;

pub async fn run_placement<T, I, O>(
    transport: &mut ReliableTransport<T>,
    interface_in: &mut I,
    interface_out: &mut O,
    state: &mut GameState,
    local_id: PlayerId,
) -> io::Result<()>
where
    T: transport::RawTransport + Send,
    I: InputProvider + Send + Sync,
    O: OutputRenderer + Send + Sync,
{
    for &ship in &[ShipType::Carrier, ShipType::Battleship, ShipType::Cruiser, ShipType::Submarine, ShipType::Destroyer] {
        loop {
            interface_out.render_state(state).await?;
            println!("Place {:?} as: place x,y,ori", ship);
            let ev = interface_in.next_input().await?;
            if let InputEvent::Place { x, y, orientation } = ev {
                if state.board_p1.place_ship(ship, x, y, orientation).is_ok() {
                    break;
                } else {
                    println!("Invalid placement");
                    continue;
                }
            }
        }
    }
    transport.send(Message::StateSync(state.clone())).await?;
    if let Message::StateSync(peer) = transport.recv().await? {
        state.board_p2 = peer.board_p1.clone();
    }
    state.phase = Phase::Playing;
    Ok(())
}

// Simple in-mem peer for testing:
pub async fn run_peer<T: transport::RawTransport + Send + 'static>(mut transport: ReliableTransport<T>) {
    let mut state = GameState::new(10,10);
    let local = PlayerId::Two;
    transport.recv().await.unwrap(); // handshake
    transport.send(Message::Handshake { player: local }).await.unwrap();
    state.phase = Phase::Playing;
}

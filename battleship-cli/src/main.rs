use battleship_engine::Engine;
use battleship_interface::cli::CLIInterface;
use battleship_player::InterfaceClient;
use battleship_transport::LocalTransport;
use futures::executor::block_on;

fn main() {
    let ui1 = CLIInterface;
    let ui2 = CLIInterface;

    let (player1_transport, engine_t1) = LocalTransport::pair();
    let (player2_transport, engine_t2) = LocalTransport::pair();

    let handle1 = std::thread::spawn(move || {
        let mut client = InterfaceClient::new(ui1, player1_transport);
        block_on(client.run());
    });

    let handle2 = std::thread::spawn(move || {
        let mut client = InterfaceClient::new(ui2, player2_transport);
        block_on(client.run());
    });

    let mut engine = Engine::new(engine_t1, engine_t2);
    block_on(engine.run());

    handle1.join().unwrap();
    handle2.join().unwrap();
}

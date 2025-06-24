mod placement;
use clap::{Arg, Command};
use tracing::{info, debug, Level};
use tracing_subscriber;
use tokio::net::TcpListener;
use tokio::io;
use std::path::Path;

use core::{GameState, PlayerId, Phase};
use core::message::Message;
use transport::{RawTransport, ReliableTransport};
use transport::adapters::{InMemTransport, TcpTransport, BtleTransport};
use persistence::JsonPersistence;
use interface_cli::CliInterface;
use player::{Player, HumanPlayer, AiPlayer, ProbAi, Difficulty, Move};

#[tokio::main]
async fn main() -> io::Result<()> {
    let matches = Command::new("Battleship")
        .version("0.1")
        .about("Multi-transport Battleship game")
        .arg(Arg::new("mode").long("mode").short('m')
            .help("Transport mode: inmem, tcp-server, tcp-client, ble")
            .takes_value(true).default_value("inmem"))
        .arg(Arg::new("addr").long("addr").short('a')
            .help("Address for TCP mode")
            .takes_value(true))
        .arg(Arg::new("buffer").long("buffer").short('b')
            .help("In-memory channel buffer size")
            .takes_value(true).default_value("32"))
        .arg(Arg::new("width").long("width")
            .help("Board width")
            .takes_value(true).default_value("10"))
        .arg(Arg::new("height").long("height")
            .help("Board height")
            .takes_value(true).default_value("10"))
        .arg(Arg::new("layout").long("layout")
            .help("Ship layout config file path")
            .takes_value(true))
        .arg(Arg::new("verbose").long("verbose").short('v')
            .help("Enable verbose logs"))
        .arg(Arg::new("ai").long("ai")
            .help("Enable AI player"))
        .get_matches();

    if matches.is_present("verbose") {
        tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();
    } else {
        tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    }

    let mode = matches.value_of("mode").unwrap();
    let addr = matches.value_of("addr");
    let buffer: usize = matches.value_of_t("buffer").unwrap_or(32);
    let width: u8 = matches.value_of_t("width").unwrap_or(10);
    let height: u8 = matches.value_of_t("height").unwrap_or(10);
    let layout_path = matches.value_of("layout");
    debug!("Config: mode={}, addr={:?}, buffer={}, width={}, height={}, layout={:?}",
        mode, addr, buffer, width, height, layout_path);

    let mut state = if let Some(path) = layout_path {
        core::layout::ShipLayout::apply
    GameState::from_file(path, width, height).unwrap_or_else(|_| GameState::new(width, height))
    } else {
        GameState::new(width, height)
    };

    let local_id: PlayerId;
    let raw: Box<dyn RawTransport<Error = io::Error> + Send> = match mode {
        "inmem" => {
            let (t1, t2) = InMemTransport::pair(buffer);
            tokio::spawn(placement::run_peer(t2));
            local_id = PlayerId::One;
            Box::new(t1)
        }
        "tcp-server" => {
            let a = addr.expect("--addr required");
            let listener = TcpListener::bind(a).await?;
            info!("Listening on {}", a);
            let (stream, _) = listener.accept().await?;
            local_id = PlayerId::One;
            Box::new(TcpTransport::new(stream))
        }
        "tcp-client" => {
            let a = addr.expect("--addr required");
            let stream = tokio::net::TcpStream::connect(a).await?;
            local_id = PlayerId::Two;
            Box::new(TcpTransport::new(stream))
        }
        "ble" => { local_id = PlayerId::One; Box::new(BtleTransport) }
        _ => panic!("Unknown mode"),
    };
    let mut transport = ReliableTransport::new(raw);
    let persistence = JsonPersistence;
    let save_path = Path::new("savegame.json");
    let mut cli = CliInterface::new();
    let mut player: Box<dyn Player + Send> = if matches.is_present("ai") {
        Box::new(ProbAi::new(width, height, Difficulty::Medium, local_id))
    } else {
        Box::new(HumanPlayer::new(cli.clone()))
    };

    transport.send(Message::Handshake { player: local_id }).await?;
    if let Message::Handshake { player: peer } = transport.recv().await? {
        info!("Handshake: {:?} <-> {:?}", local_id, peer);
    }

    placement::run_placement(&mut transport, &mut cli, &mut cli, &mut state, local_id).await?;

    while !state.is_game_over() {
        cli.render_state(&state).await?;
        // omitted main loop for brevity...
        break;
    }

    Ok(())
}

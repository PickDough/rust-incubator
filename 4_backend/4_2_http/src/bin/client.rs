#[allow(dead_code, unused_variables)]
use std::{io, thread};

use actix_web::web::Bytes;
use anyhow::Result;
use awc::ws;
use clap::{command, Parser, Subcommand};
use futures_util::sink::SinkExt;
use serde::{Deserialize, Serialize};
use step_4_2::commands::clap::{RoleCommands, UserCommands};
use tokio::{select, sync::mpsc};
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt as _};

extern crate step_4_2;

#[actix::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    let mut cmd_rx = UnboundedReceiverStream::new(cmd_rx);

    // run blocking terminal input reader on separate thread
    let input_thread = thread::spawn(move || loop {
        let mut cmd = String::with_capacity(32);

        if io::stdin().read_line(&mut cmd).is_err() {
            eprintln!("Failed to read line from stdin");
            return;
        }

        let cmd = cmd.trim();
        let cli = Cli::parse_from(cmd.split_whitespace());
        print!("{cmd:?} ");

        cmd_tx.send(cli.command).unwrap();
    });

    let (_, mut ws) = awc::Client::new()
        .ws("ws://127.0.0.1:8080/ws")
        .connect()
        .await
        .unwrap();

    loop {
        select! {
            Some(msg) = ws.next() => {
                match msg {
                    Ok(ws::Frame::Text(txt)) => {
                        // log echoed messages from server
                        println!("Server: {txt:?}");
                    }

                    Ok(ws::Frame::Ping(_)) => {
                        // respond to ping probes
                        ws.send(ws::Message::Pong(Bytes::new())).await.unwrap();
                    }

                    _ => {}
                }
            }

            Some(cmd) = cmd_rx.next() => {
                ws.send(ws::Message::Text(serde_json::to_string(&cmd)?.into())).await.unwrap();
            }

            else => break
        }
    }

    input_thread.join().unwrap();

    Ok(())
}

/// Cli for defined CRUD operations on `User` and `Role` entities
#[derive(Debug, Parser)]
#[command(version, about, long_about)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum Commands {
    User {
        #[clap(subcommand)]
        command: UserCommands,
    },
    Role {
        #[clap(subcommand)]
        command: RoleCommands,
    },
}

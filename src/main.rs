use asking::error::Processing;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    loop {
        loop {
            let eula_accepted = asking::yn()
                .message("Please agree to the MINECRAFT END USER LICENSE AGREEMENT at https://account.mojang.com/documents/minecraft_eula before continuing. [y/N]")
                .default_value(false)
                .ask();

            match async_std::task::block_on(eula_accepted) {
                Ok(true) => println!("EULA accepted"),
                Ok(false) => {
                    println!("You have to agree to the MINECRAFT END USER LICENSE AGREEMENT at https://account.mojang.com/documents/minecraft_eula before continuing.");
                    continue;
                }
                Err(_processing) => {
                    println!("You have to agree to the MINECRAFT END USER LICENSE AGREEMENT at https://account.mojang.com/documents/minecraft_eula before continuing.");
                    continue;
                }
            }
        }
    }
}

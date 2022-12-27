pub mod commands;
pub mod client;
mod services;

use std::collections::HashMap;
use std::io::{stdout, Write};
use crate::commands::{
    Args,
    CommandTypes
};
use clap::{Parser};
use services::handlers;
use client::client_builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = client_builder();

    match args.commands {
        CommandTypes::Text(cmd) => {
            let res = handlers::generate_text(
                client,
                &cmd
            ).await;
            match res {
                Ok(text) => {
                    stdout().write(text.as_bytes())?;
                },
                Err(err) => println!("{}", err)
            }
        },
        CommandTypes::Image(cmd) => handlers::generate_image(
            client,
            &cmd.description
        ).await,
    }

    Ok(())
}

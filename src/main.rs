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
        CommandTypes::GenerateText(cmd) => {
            let res = handlers::generate_text(
                client,
                &cmd
            ).await;
            match res {
                Ok(text) => {
                    stdout().write(text.as_bytes())?;
                    stdout().write("\n\n".as_bytes())?;
                },
                Err(err) => println!("{}", err)
            }
        },
        CommandTypes::GenerateImage(cmd) => {
            let res = handlers::generate_images(
                client,
                &cmd
            ).await;
            match res {
                Ok(images) => {
                    images.iter().for_each(|image| {
                        stdout().write(image.as_bytes()).unwrap();
                        stdout().write("\n\n".as_bytes()).unwrap();
                    });
                },
                Err(err) => println!("{}", err)
            }
        },
    }

    Ok(())
}

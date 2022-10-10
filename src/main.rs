use controllers::subscribe::subscribe;
use controllers::unsubscribe::unsubscribe;
use scrapers::scrape::scrape;
use teloxide::Bot;
use teloxide::types::Message;
use teloxide::utils::command::BotCommands;
use teloxide::{prelude::*};
use std::result::Result;
use tokio_cron_scheduler::{JobScheduler, Job};
use std::thread;
use std::env;
use dotenv::dotenv;

use crate::scrapers::scrape_iskanjedela::scrape_iskanjedela;

mod helpers;
mod scrapers;
mod models;
mod controllers;
mod config;

extern crate serde_json;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("$TELEGRAM_BOT_TOKEN is not set");
    env::set_var("TELOXIDE_TOKEN", token);
    pretty_env_logger::init();
    let bot = Bot::from_env();
    thread::spawn(|| {
        run_cron();
    });
    println!("Running telegram bot!");
    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Subscribe for jobs")]
    Subscribe,
    #[command(description = "Unsubscribe from jobs")]
    Unsubscribe
}



#[tokio::main]
async fn run_cron() {
    let mut sched = JobScheduler::new();
    match sched.add(Job::new_async("0 10,20,30,40,50,0 * * * *", move |_, _|  Box::pin(async { 
        match scrape() {
            Ok(_) => (),
            Err(e) => println!("Error on scrape: {:?}", e)
        }
    })).unwrap()) {
        Ok(c) => println!("Started cron!: {:?}", c),
        Err(e) => println!("Something went wrong scheduling CRON: {:?}", e)
    };
    match sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
          println!("Shut down done");
        })
    })) {
        Ok(c) => println!("Shutdown handler set for cron!: {:?}", c),
        Err(e) => println!("Something went wrong setting shutdown handler for CRON: {:?}", e)
    };
    if let Err(e) = sched.start().await {
        eprintln!("Error on scheduler {:?}", e);
    }
}

async fn answer(
    bot: Bot,
    message: Message,
    command: Command,
) -> ResponseResult<()> {
    match command {
        Command::Help => { bot.send_message(message.chat.id, Command::descriptions().to_string()).await? },
        Command::Unsubscribe => { bot.send_message(message.chat.id, unsubscribe(&bot, message)).await? },
        Command::Subscribe => { bot.send_message(message.chat.id, subscribe(&bot, message)).await? },
    };
    Ok(())
}



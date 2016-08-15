extern crate tokio;
extern crate rand;

use tokio::reactor;
use tokio::reactor::{Reactor, Task, Tick};
use tokio::tcp::TcpListener;
use rand::{Rng, thread_rng};

use std::io::{self, Write, BufReader, BufRead};
use std::fs::File;

struct Listener {
  socket: TcpListener,
  quotes: Vec<String>
}

impl Task for Listener {
  fn tick(&mut self) -> io::Result<Tick> {
    while let Some(mut conn) = try!(self.socket.accept()) {
      let quote = thread_rng().choose(&self.quotes).unwrap();
      println!("{}", &quote);       
      try!(conn.write_all(quote.as_bytes()));
    }

    Ok(Tick::WouldBlock)
  }
}

fn read_quotes(file: &str) -> Vec<String> {
   let mut quotes = Vec::new();
   let quote_file = File::open(file).unwrap();
   let mut quote = String::new();
   for line in BufReader::new(quote_file).lines() {
     if let Ok(line) = line {
       if line == "%" {
         quotes.push(quote.clone());
         quote.clear();
       } else {
         quote.push_str(&line);
         quote.push('\n');
       }
     }
   }
   quotes.push(quote);
   return quotes;
}

fn main() {
   println!("Reading quotes"); 
   let quotes = read_quotes("quotes.txt");

   println!("Starting the *EVIL* asynchronous QoTD server...");

   let reactor = Reactor::default().unwrap();

   reactor.handle().oneshot(||{
     let addr = "0.0.0.0:1234".parse().unwrap();
     let listener = match TcpListener::bind(&addr) {
       Ok(l) => l,
       Err(e) => {
         println!("Error creating listener: {}", e);
         std::process::exit(1);
       }
     };

     reactor::schedule(Listener {socket: listener, quotes: quotes});

     Ok(())
   });

   reactor.run();
}
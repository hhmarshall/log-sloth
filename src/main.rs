use std::env;
use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::{ BufReader, BufRead, Error, ErrorKind };
use std::process::exit;

// for simple testing, from another term window
// $ echo "a=b c=d e=f BLERG" | nc localhost 1516

fn main() {
  let args : Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("must set action to 'server' or 'client'");
    exit(1)
  }

  match &args[1][..] {
    "server" => server(),
    "client" => unimplemented!(),
    other => {
      println!("we don't handle {:?}, use 'server' or 'client'", other);
      Ok(())
    }
  }.unwrap();

  ()
}

fn server() -> Result<(),io::Error> {
  let listener = TcpListener::bind("0.0.0.0:1516")?;

  for stream in listener.incoming() {
    let _res = handle_client(&mut stream?);
  };

  Ok(())
}

fn handle_client(stream: &mut TcpStream) -> Result<(),io::Error> {
  let bufr = BufReader::with_capacity(4*1024, stream);
  for line in bufr.lines() {
    handle_line(line?)?
  }
  Ok(())
}

fn handle_line(line: String) -> Result<(),io::Error> {
  let f = Fortigate{};
  // let _res = f.process(&line[..]);
  let _res = f.process2(&line[..]);


  Ok(())
}

pub trait LogProcessor {
  fn process (&self, string: &str) -> Result<String,io::Error>;
  fn process2(&self, string: &str) -> Result<String,io::Error>;
}

struct Fortigate {}

impl LogProcessor for Fortigate {
  fn process(&self, string: &str) -> Result<String,io::Error> {
    // println!("sup {}", string);
    let tokens : Vec<Vec<String>> = string.split_whitespace() .map(|x| x.split(' ') .map(|x| x.to_owned()) .collect() ) .collect(); 
    // Err(Error::new(ErrorKind::InvalidData, "bad line".to_string()))

    println!("token value: {:?}", tokens);

    Ok("yes".to_owned())
  }

  fn process2(&self, string: &str) -> Result<String,io::Error> {
    // attempt to reuse previous code to include split on "="
    //  - it works, but the output is verbose internals
    // println!("sup {:?}", string
    //   .split_whitespace()
    //   .map(|x| x.split('=').map(|x| x.to_owned()))
    //   .collect::<Vec<_>>());

    // simpler 2 phase approach
    let tokens : Vec<&str> = string
      .split_whitespace()
      .collect();

    println!("token value: {:?}", tokens);
    for token in tokens.iter() {
      println!("{:?}", token.split('=').collect::<Vec<_>>() );
    }

    // confused map experiment that ...umm...doesn't crash
    println!("{:?}",
      tokens
        .iter()
        .map(|x| x.split('='))
        .collect::<Vec<_>>() );

    Ok("yes".to_owned())
  }
}

pub struct NLog {
  pub application: String
}

#[test]
fn fortigate_parses() {
  let f = Fortigate{};
  let res = f.process("a=b c=d e=f g=h");
  assert_eq!(res.unwrap(),"123")
}

#[test]
fn fortigate_parses2() {
  let f = Fortigate{};
  // not complete - func call still just prints most data
  let res = f.process2("a=b c=d e=f BLERG");
}
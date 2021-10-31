use structopt::StructOpt;
use structopt::clap::Error;
use structopt::clap::ErrorKind;

#[derive(Debug, StructOpt)]
#[structopt(
help = 
"usage: ./njvm [option] [option] ...
  --prog1          select program 1 to execute
  --prog2          select program 2 to execute
  --prog3          select program 3 to execute
  --version        show version and exit
  --help           show this help and exit",
name = "Ninja Virtual Machine",
version = "version 1 (compiled Sep 23 2015, 10:36:52)",
)]
pub struct Njvm {
    #[structopt(long = "prog1", about = "Select program 1 to execute")]
    prog1: bool,
    #[structopt(long = "prog2")]
    prog2: bool,
    #[structopt(long = "prog3")]
    prog3: bool,
}

impl Njvm {
    pub fn start(command: Njvm) {
        println!("Ninja Virtual Machine started");
        match command {
            Njvm { prog1: true, ..}=> Njvm::prog1(),
            Njvm { prog2: true, ..}=> Njvm::prog2(),
            Njvm { prog3: true, ..}=> Njvm::prog3(),
            _ => {}

        }
        println!("Ninja Virtual Machine stopped");
    }

    fn prog1() {
        println!("Executing prog1");
    }
 
    fn prog2() {
        println!("Executing prog2");
    }
 
    fn prog3() {
        println!("Executing prog3");
    }
}

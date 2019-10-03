extern crate rustyline;
use std::convert::From;

use rlua::{Lua};

use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Debug)]
enum Error {
    Lua(rlua::Error),
    Readline(ReadlineError),
}
impl From<rlua::Error> for Error {
    fn from(err: rlua::Error) -> Error { Error::Lua(err) }
}
impl From<ReadlineError> for Error {
    fn from(err: ReadlineError) -> Error { Error::Readline(err) }
}

fn main() -> Result<(), Error> {
    let lua = Lua::new();

    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    lua.context(|lua_ctx| {
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    match lua_ctx.load(line.as_str()).exec() {
                        Ok(_) => {},
                        Err(err @ rlua::Error::SyntaxError { .. }) => {
                            println!("{}", err)
                        },
                        Err(err) => println!("Error: {:?}", err),
                    }
                },
                Err(ReadlineError::Interrupted) => { break },
                Err(ReadlineError::Eof) => { break },
                Err(err) => { println!("Error: {:?}", err) }
            }
        }
        rl.save_history("history.txt")?;
        Ok(())
    })
}

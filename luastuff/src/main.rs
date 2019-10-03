extern crate rustyline;
use std::sync::{Arc, Mutex};
use std::convert::From;

use rlua::{Lua, Function, Integer};

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

fn setup(lua: Lua) -> Result<(), Error> {
    lua.context(|lua_ctx| {
        let globals_arc = Arc::new(Mutex::new(lua_ctx.globals()));

        let sum = lua_ctx.create_function(|_, list: (Vec<i32>)| {
            let res: i32 = list.iter().sum();
            Ok(res)
        })?;
        {
            let globals = globals_arc.lock().unwrap();
            globals.set("sum", sum)?;
        }

        let print_sum = lua_ctx.create_function(|_, (list, label): (Vec<i32>, String)| {
            let globals = globals_arc.lock().unwrap();
            // let print: Function = globals.get("print")?;
            // let res = sum.call::<_, Integer>(list)?;
            let res = 42i32;
            // print.call::<_, ()>(format!("{}: {}", label, res))?;
            Ok(())
        })?;
        {
            let globals = globals_arc.lock().unwrap();
            globals.set("print_sum", print_sum)?;
        }
        Ok(())
    })
}

fn main() -> Result<(), Error> {
    let lua = Lua::new();

    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    setup(lua)?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                lua.context(|lua_ctx| {
                    match lua_ctx.load(line.as_str()).exec() {
                        Ok(_) => {},
                        Err(err @ rlua::Error::SyntaxError { .. }) => {
                            println!("{}", err)
                        },
                        Err(err) => println!("Error: {:?}", err),
                    }
                })
            },
            Err(ReadlineError::Interrupted) => { break },
            Err(ReadlineError::Eof) => { break },
            Err(err) => { println!("Error: {:?}", err) }
        }
    }
    rl.save_history("history.txt")?;
    Ok(())
}

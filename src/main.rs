/*
 * file: main.rs
 * author: kota kato 2022
 * description:
 *   main source code of rusty_awk
 */

// TODO
//
// awk '{print $2 $1}'

mod ast;
mod core;

use crate::core::AWKCore;

fn main() {
    mainloop();
}

fn mainloop() {
    // check arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        // show help message
        println!("usage: awk [-F fs] [-v var=value] [-f progfile | 'prog'] [file ...]");
        return;
    }

    // parse awk program
    let parsed_program = ast::program::parse_program(&args[1]);
    // check parse error
    if parsed_program.is_err() {
        println!("Parse Err!!!");
        dbg!(&parsed_program);
        return;
    }
    // executable program
    let core = AWKCore::new_core(parsed_program.unwrap().1);
    dbg!(&core);

    core.exec_program();
}

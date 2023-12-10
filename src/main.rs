use std::env::{self, args};

use nocturnes::{crash, diag, fond};

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = &args[1];
    if cmd == "diag" {
        diag::diag();
    } else if cmd == "fond" {
        let mut path = None;
        if args.len() > 1 {
            path = Some(args[2].to_string());
        }

        fond::fond(path);
    } else if cmd == "crash" {
        crash::crash();
    }
}

// main.rs
//
// Copyright (c) 2022 Jeremiah LaRocco <jeremiah_larocco@fastmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

extern crate argparse;
extern crate snake_game;

use argparse::{ArgumentParser, Store};

fn main() {
    let mut file_name = "".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("The path to the thing.");
        ap.refer(&mut file_name)
            .add_argument("configfile", Store,
                          "Config file.");
        ap.parse_args_or_exit();
    }
    println!("Reading the thing: {}", file_name);

    let the_thing = snake_game::snake::go();
    println!("{:?}", the_thing);
}

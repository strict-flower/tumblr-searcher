use crate::searcher::frontend;
use std::io;

mod searcher;

fn main() -> io::Result<()> {
    frontend::main()
}

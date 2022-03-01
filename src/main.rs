
mod game;

use std::io;


fn read_line() -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim_end().to_string())
}



fn main() -> io::Result<()> {
    loop {
        let x = read_line()?;

        println!("{}", x.len());
    }
    Ok(())
}

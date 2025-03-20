use std::str;
use std::io::{self, BufRead, Write};

struct Forth(());

impl Forth {
    fn parse(source_code: &str) -> Result<Forth, ()> {
        println!("Forth source code: {}", source_code);
        Ok(Forth(()))
    }

    fn run<R: BufRead, W: Write, E: Write>(self: Self, mut stdin: R, mut stdout: W, mut stderr: E) -> io::Result<()> {
        writeln!(stderr, "Testing stderr..")?;
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        write!(stdout, "{}", line)?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let source_code = "1";
    let forth = Forth::parse(source_code).expect("Parsing Forth source code failed due to Syntax Error");
    forth.run(io::stdin().lock(), io::stdout(), io::stderr())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forth_runs() -> io::Result<()> {
        let stdin = io::Cursor::new("1");
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let source_code = "1";
        let forth = Forth::parse(source_code).expect("Parsing Forth source code failed due to Syntax Error");
        forth.run(stdin, &mut stdout, &mut stderr)?;
        let stdout = str::from_utf8(&stdout).expect("UTF8 Error while converting stdout to string in test");
        let stderr = str::from_utf8(&stderr).expect("UTF8 Error while converting stderr to string in test");
        assert_eq!(stdout, "1");
        Ok(())
    }
}

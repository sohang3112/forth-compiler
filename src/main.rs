use std::str::{self, FromStr};
use std::io::{self, BufRead, Write};

mod parser;
use parser::AST;

impl AST {
    fn run<R: BufRead, W: Write, E: Write>(self: Self, mut stdin: R, mut stdout: W, mut _stderr: E) -> io::Result<()> {
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        write!(stdout, "{}", line)?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let code = "1";
    let forth: AST = source_code.parse().expect("Parsing Forth source code failed due to Syntax Error");
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
        let code = "1";
        let forth: AST = source_code.parse().expect("Parsing Forth source code failed due to Syntax Error");
        forth.run(stdin, &mut stdout, &mut stderr)?;
        let stdout = str::from_utf8(&stdout).expect("UTF8 Error while converting stdout to string in test");
        let stderr = str::from_utf8(&stderr).expect("UTF8 Error while converting stderr to string in test");
        assert_eq!(stdout, "1");
        assert_eq!(stderr, "");
        Ok(())
    }
}

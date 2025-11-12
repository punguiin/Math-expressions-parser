# Math expression parser

Main idea of parses that it will take lines of text from .txt file as input, extract mathematical expressions from it, calculate them, and output the results in "res.txt" file. Parser will be able to work with basic operations, trigonometrical functions, exponential functions (may be more in future)

## Code example

```rust
use clap::{Arg, Command, builder::PathBufValueParser};
use math_expression_parser::parse_and_eval;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    //here should be your code that gets path to input file. Example (using clap):
    let matches = Command::new("Math Expression parser")
        .version("1.0")
        .author("Nazar Sydorchuk <n.sydorchuk@ukma.edu.ua>")
        .about("Parses and evaluates mathematical expressions from a file")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("Path to the input file containing the mathematical expression")
                .value_parser(PathBufValueParser::new()),
        )
        .get_matches();

    let file_path: &PathBuf = matches.get_one("file").expect("No file path provided");
    let input = std::fs::read_to_string(file_path)?;
    let lines: Vec<&str> = input.lines().collect();
    for input in lines {
        parse_and_eval(input)?;
    }
    Ok(())
}
```

## Output example

```txt

```

## Grammar

```pest
WHITESPACE = _{ " " | "\t" | "\n" | "\r" }
num = { ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT+ |  ASCII_DIGIT+ }
plus = { "(" ~ expression ~ "+" ~ expression ~ ")" }
minus = { "(" ~ expression ~ "-" ~ expression ~ ")" }
multiply = { "(" ~ expression ~ "*" ~ expression ~ ")" }
divide = { "(" ~ expression ~ "/" ~ expression ~ ")" }
sin = { "sin" ~ "(" ~ expression ~ ")" }
cos = { "cos" ~ "(" ~ expression ~ ")" }
tan = { "tan" ~ "(" ~ expression ~ ")" }
exp = { "exp" ~ "(" ~ expression ~ ")" }
pow = { "pow" ~ "(" ~ expression ~ "," ~ expression ~ ")" }
root = { "root" ~ "(" ~ expression ~ "," ~ expression ~ ")" }
log = { "log" ~ "(" ~ expression ~ "," ~ expression ~ ")" }
ln = { "ln" ~ "(" ~ expression ~ ")" }
expression = { num | plus | minus | multiply | divide | sin | cos | tan | exp | root | pow | log | ln }
input = { SOI ~ expression ~ EOI }
```

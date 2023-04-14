# RUSTLexer
## Description
- a Lexer in Rust. the main goal is to take source file and tokenize it line by line.

## Tokenization Details.
- This programs tokenizes every valid token and assign to it a value, type, and a Location(Row, col).
- Token types:
```Rust
enum TokenT {
    // Special tokens.
    COMMENT__,
    DQUOTE__,
    SQUOTE__,
    OPAR__,
    CPAR__,
    OCURLY__,
    CCURLY__,
    PLUS__,
    MINUS__,
    COMA__,
    SEMICOLON__,
    EQUAL__,
    GT__,
    LT__,
    QM__,

    // Other
    NONE__,
    NUMBER__,
    STRING__,
    VARNAME__,
    
    // Built-in functions.
    PRINT__,
}
```
- How To use:
```Rust
fn main() -> Result<(), io::Error> {
	// Create a new lexer.
    let mut lex: Lexer = Lexer::new("file.txt");

	// display The state of the lexer.
    lex.display();
    
	// Read the file into the lexer.
	lex.read()?;

	// make token to store the next token.
    let mut token;
    
    while lex.is_not_empty() {
        token = lex.next(); // return Result so we need to match.
		match token {
			Ok(mut token_) => {
				// Token found.
				token_.display_token();
			},
			Err(e) => {
				// Error.
				println!("{}", e);
				Ok(())
			}
		}
	}
    

    return Ok(());
}
```
## Quick Start
``` console
	$ cargo run ./source
```

## Structures
```Rust
struct Location {
    row: usize,
    col: usize,
}

struct Lexer<'a> {
    file_path: &'a str,
    source:    Vec<u8>,
    cur:       usize,
    row:       usize,
    col:       usize,
    size:      usize,
    token_table: HashMap<char, TokenT>,
}
struct Token {
    value:       String,
    token_type:  TokenT,
    size:        usize,
    loc:         Location,
}```
- for more click [Here](https://github.com/Moody0101-X/RUSTLexer/blob/main/src/main.rs)


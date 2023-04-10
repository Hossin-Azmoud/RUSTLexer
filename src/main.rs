use std::fs::File;
use std::io;
use std::io::Read;

// TODO: Read file.
// TODO: Tokenize file content

fn main() -> io::Result<()>{

    const SPACE:     char  = ' ';
    const NL:        char  = '\n';
    
    const OPAR:      char  = '(';
    const CPAR:      char  = ')';
    const OCURLY:    char  = '{';
    const CCURLY:    char  = '}';
    const PLUS:      char  = '+';
    const COMA:      char  = ',';
    const SEMICOLON: char  = ';';

    let mut source = File::open("source")?;
    let mut contents = vec![];
    
    source.read_to_end(&mut contents)?;

    let mut index:        usize  = 0;
    let mut token_index:  usize  = 0;
    let mut token_buffer: String = String::new();

    while index < contents.len() {

        let mut c: char = char::from(contents[index]);     
        
        while c != SPACE && c != NL && index < contents.len()
        {
            match c {
                OPAR =>  {
                    token_buffer = String::from(OPAR);
                    index += 1;
                    break;
                },
                CPAR =>  {
                    token_buffer = String::from(CPAR);
                    index += 1;
                    break;
                },

                OCURLY => {
                    token_buffer = String::from(OCURLY);
                    index += 1;
                    break;
                },
                
                CCURLY => {
                    token_buffer = String::from(CCURLY);
                    index += 1;
                    break;
                },
                COMA => {
                    token_buffer = String::from(COMA);
                    index += 1;
                    break;
                },
                SEMICOLON => {
                    token_buffer = String::from(SEMICOLON);
                    index += 1;
                    break;
                },
                
                PLUS => {
                    token_buffer = String::from(PLUS);
                    index += 1;
                    break;
                },

                _ => {
                    while c.is_alphanumeric() {
                        if c == SPACE {
                            index += 1;
                            break;
                        }
                        token_buffer += &String::from(c);
                              
                        if index < contents.len() { index += 1; } else {
                            break;
                        }
                        
                        c = char::from(contents[index]);                    
                    }

                    break;
                }
            }
        }
        
        /*        
        
        if token_buffer.len() > 0 {
            print!("{} => { }\n", token_index, token_buffer);
            token_buffer = String::new();
        }
        
        */
        if token_buffer.len() > 0 {
            print!("{} => { }\n", token_index, token_buffer);
            token_buffer = String::new();
            token_index += 1;
        }
        index += 1; 
   }
    
 
    Ok(())

}





/*
struct Token {
    Type:  String,
    Value: String,
};

struct Lexer {
    File_path: &str,
    Source: String,
    Cursor: i32,
    Row: i32,
    Col: i32,
};

impl Lexer {

    fn new(file_path: &str) -> self {
        Lexer {
            File_path: file_path,
            Source: String::new(),
            Cursor: 0,
            Row: 0,
            Col: 0,
        };   
    }
    
    fn next(&mut self) -> Token {
        return 
    }
    
    fn start(&mut self) {
        let mut f = File::open(self.File_path)?;
        let mut buffer = ;
        f.read_to_string(&mut self.Source)?;
    }

    fn is_empty(&mut self) {
        
    }
}
*/


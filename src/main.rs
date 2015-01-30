use std::old_io as io;
use std::os;

#[derive(PartialEq, Eq)]
struct CodeLine {
    indent: i16,
    code: String,
}

fn main() {
    let args = os::args();
    let mut f1 = io::BufferedReader::new(io::File::open(&Path::new(&args[1])));
    let mut f2 = io::BufferedReader::new(io::File::open(&Path::new(&args[2])));
    let mut diff = false;
    loop {
        let l1 = f1.lines().next();
        let l2 = f2.lines().next();
        if l1 == None {
            if l2 != None {
                diff = true;
            }
            break;
        } else if l2 == None {
            diff = true;
            break;
        } else {
            let cl1 = CodeLine::from_string(l1.unwrap().ok().unwrap());
            let cl2 = CodeLine::from_string(l2.unwrap().ok().unwrap());
            print!("{} {}", cl1.indent, cl1.code);
            print!("{} {}", cl2.indent, cl2.code);
            if cl1.code != cl2.code {
                diff = true;
            }
        }
    }
    if diff {
        println!("Files differ.");
    } else {
        println!("Files are the same!");
    }
}

impl CodeLine {
    fn from_string(line: String) -> CodeLine {
        let mut indent = 0;
        for (i,c) in line.chars().enumerate() {
            match c {
                '\t' => indent += 8,
                ' ' => indent += 1,
                _ => return CodeLine { indent: indent, code: line[i..].to_string() }
            }
        }
        return CodeLine { indent: indent, code: "".to_string() };
    }
}

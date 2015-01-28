use std::old_io::BufferedReader;

use std::old_io::File;
use std::os;

fn main() {
    let args = os::args();
    let mut f1 = BufferedReader::new(File::open(&Path::new(&args[1])));
    let mut f2 = BufferedReader::new(File::open(&Path::new(&args[2])));
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
            break;
        } else if l1.unwrap() != l2.unwrap() {
            diff = true;
        }
    }
    if diff {
        println!("Files differ.");
    } else {
        println!("Files are the same!");
    }
}

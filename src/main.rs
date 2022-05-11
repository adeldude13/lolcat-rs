use std::io;

fn rgb(i: usize) -> (u8, u8, u8) {
    let sin = f64::sin; 
    let f = 0.1;
    let pi = std::f64::consts::PI;
    ((sin(f*(i as f64))*127.0 + 128.0) as u8, (sin(f*(i as f64)+2.0*pi/3.0)*127.0 + 128.0) as u8, (sin(f*(i as f64)+4.0*pi/3.0)*127.0 + 128.0) as u8)
}

fn printrgb((r, g, b):(u8, u8, u8), c:char) {
    print!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, c);
}


fn make(inp: String) { 
    let mut i: usize = 0; 
    while i < inp.len() {
        let (r, g, b):(u8, u8, u8) = rgb(i);
        let c: char = inp.chars().nth(i).unwrap();
        printrgb((r, g, b), c);
        i+=1;
    }
}


fn main() {

    loop {
        let mut inp = String::new();
        let err = io::stdin().read_line(&mut inp);
        match err {
            Ok(len) => {
                if len == 0 {
                    return;
                } else {
                    make(inp); 
                }
            }
            Err(error) => {
                eprintln!("error: {}", error);
                return; 
            }
        }
    }
}


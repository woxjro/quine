fn main() {
    let s = include_str!("main.rs");
    let v: Vec<_> = s
        .split('\u{20}')
        .map(|e| e.trim())
        .filter(|e| e.len() > 0)
        .collect();
    let limit = 22;
    let mut now = 0;
    for e in v {
        //println!("{e:>limit$}");
        if e.len() + now < limit {
            print!("{} ", e);
            now += e.len();
        } else {
            println!("");
            print!("{} ", e);
            now = e.len();
        }
    }
}

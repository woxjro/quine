use include_str as is;
use println as pln;
fn main() {
    let spc = "\u{20}";
    let s = is!("main.rs");
    let ns = s.replace("\n", spc);
    let mut v: Vec<_> = ns
        .split(spc)
        .map(|e| e.trim())
        .filter(|e| e.len() > 0)
        .collect();
    let lmt = 32;
    let mut w;
    let mut res = vec![];
    while v.len() > 0 {
        (w, v) = a(v, lmt);
        res.push((w, lmt));
    }
    for (i, (row, lmt)) in res.iter().enumerate() {
        let s = p(&row, *lmt);
        let n = (5 - (i as i32 % 10)).abs() as usize;
        pln!("{}{s}", "  ".repeat(n * 2));
    }
}
fn a(v: Vec<&str>, lmt: usize) -> (Vec<&str>, Vec<&str>) {
    let mut now = 0;
    let mut r = vec![];
    let mut idx = 0;
    let mut l0 = false;
    for (i, &e) in v.iter().enumerate() {
        if lmt < now {
            break;
        }
        if lmt < now + e.len() + if i == 0 { 0 } else { 1 } {
            if r.len() <= 1 {
                l0 = true;
            }
            break;
        }
        r.push(e);
        now += e.len() + if i == 0 { 0 } else { 1 };
        idx = i;
    }
    if l0 {
        (vec![], v)
    } else {
        (r, v[(idx + 1)..].to_vec())
    }
}
fn p(v: &Vec<&str>, lmt: usize) -> String {
    let sum: usize = v.iter().map(|e| e.len()).sum();
    let n = (lmt - sum) % v.len();
    let mut res = "".to_string();
    for (i, e) in v.iter().enumerate() {
        res = format!(
            "{}{}{}",
            res,
            e,
            "\u{20}".repeat(((lmt - sum) / v.len()) + if i <= n { 1 } else { 0 })
        );
    }
    res
}

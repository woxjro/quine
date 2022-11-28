use image::{self, imageops::*};
fn main() {
    let size = 100;
    let img = image::open("woxjro_100x100.jpg").unwrap();
    let mut img = img.grayscale();
    let mut img = img.as_mut_luma8().unwrap();
    let buf = img.as_raw();

    let data: Vec<_> = buf
        .chunks(size)
        .map(|row| row.iter().map(|e| e < &254).collect::<Vec<_>>())
        .collect();
    for row in &data {
        for e in row {
            print!("{}", if *e { "  " } else { "OO" });
        }
        println!("");
    }

    let res = compression(&data, size);
    let res2 = compression2(&res, size);
    for row in res {
        for e in &row {
            //print!("{} ", e.1);
        }
        //println!("{}", row.iter().map(|e| e.1).sum::<usize>());
        //println!("");
    }

    dbg!(&res2);

    dither(&mut img, &BiLevel);
    img.save("woxjro.png").unwrap(); // this step is optional but convenient for testing
}

fn compression(data: &Vec<Vec<bool>>, size: usize) -> Vec<Vec<(bool, usize)>> {
    let mut res = vec![];
    for row in data {
        let mut left = 0;
        let mut right = 0;
        let mut row_res = vec![];
        while left < size {
            while row[left] == row[right] {
                right += 1;
                if right >= size {
                    break;
                }
            }
            row_res.push((row[left], right - left));
            left = right;
        }
        res.push(row_res);
    }
    res
}

fn compression2(data: &Vec<Vec<(bool, usize)>>, size: usize) -> Vec<(Vec<(bool, usize)>, usize)> {
    let mut left = 0;
    let mut right = 0;
    let mut res = vec![];
    while left < size {
        while data[left] == data[right] {
            right += 1;
            if right >= size {
                break;
            }
        }
        res.push((data[left].clone(), right - left));
        left = right;
    }
    res
}

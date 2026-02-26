#![allow(unused)]

#[derive(Debug)]

fn main() {
    let mut pal: i32 = -132;
    let palu: u32 = 132;
    let rt: i64 = palu as i64;
    let palt: isize = 132;
    let we: bool = true;
    let ch: char = 'g';
    let dwg: i32 = i32::MAX;
    let arr: [u32; 23] = [1; 23];

    let tup: (i32, bool, f32) = (12, true, 33.3);
    let lang = "game";
    let sli = &arr[1..3];

    let new: String = String::from("hello wor;d");
    let neww:String = "hello woeed".to_string();
    let newss: &str = "dgdsd";
    let eeed: &str = &new;

     
    format!("{eeed}");
     println!("{eeed}");
    println!("hello world");
    println!("hello world");
    println!("{:?}", lang);
    println!("{:#?}", lang);
    println!("{:#?}", tup);
    println!("{:#?}", arr);
    println!("{:#?}", arr);
    println!("{:?}", sli)
}

#![allow(unused)]

// #[derive(Debug)]

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

    let new: String = String::from("hello world");

    let neww: String = "hello woeed".to_string();

    let newss: &str = "dgdsd";

    let eeed: &str = &new;


    match newwss{
        
    }
    match newwssddd{
        
    }
    match newwssddd{
        
    }
    match newwssddd{
        
    }
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }
    impl Point {
        fn strange(&mut self, x: f32, y: f32) {
            self.x = x;
            self.y = y;
        }

        // Associated function
        fn new(x: f32, y: f32) -> Self {
            Self { x, y }
        }

        // Method
        fn move_to(&mut self, x: f32, y: f32) {
            self.x = x;
            self.y = y;
        }
    }
    let mut p = Point::new(1.0, 2.0);

    p.move_to(5.0, 8.0);

    println!("{:?}", p);

    //Point {x:2, y:2 }::strange(4, 5)
    let mut p = Point { x: 2.0, y: 2.0 };
    p.strange(4.0, 5.0);
    println!("this is the new x: {:?}", p);

    format!("{eeed}");

    println!("{eeed}");

    println!("hello world");

    println!("{:?}", lang);

    println!("{:#?}", lang);

    println!("{:#?}", tup);

    println!("{:#?}", arr);

    println!("{:?}", sli)
}

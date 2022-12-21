use std::io;

fn print_dots(num_of_dots: u8) {
    for _ in 0..num_of_dots  {
        print!(".");
    }
}

fn print_asterisks(num_of_asterisks: u8) {
    for _ in 0..num_of_asterisks  {
        print!("*");
    }
}

fn print_crown(width: u8, height: u8) {
    let mut h = (width+1)/2;//calculate height of crown according to the picture width
    let w = (height*2)-1;//calculate width of crown according to the picture height
    let mut additional_dots = 0;

    if height > h {
        //the height of the picture is larger than the tree, so we need some extra lines on the top of the picture
        for _ in 0..height-h {
            print_dots(width);
            println!();
        }
    }
    else if width > w {
        h = height;//use expected tree height
        //the width of the picture is larger than the tree, so we need some extra dots on the left and right side of the picture
        additional_dots = (width-w)/2;
    }

    for x in 0..h {
        let num_of_dots = h-x-1+additional_dots;
        let num_of_asterisks = 2*x+1;
        print_dots(num_of_dots);
        print_asterisks(num_of_asterisks);
        print_dots(num_of_dots);
        println!();
    }
}

fn print_tree_trunk(width: u8) {
    let num_of_dots = (width-3)/2;
    let trunk_width = 3;

    for _ in 0..3 {
        print_dots(num_of_dots);
        print_asterisks(trunk_width);
        print_dots(num_of_dots);
        println!();
    }
}

fn read_number()-> u8 {
    loop {
        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        let num: u8 = match num.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        return num;
    }
}

fn main() {
    println!("Enter width (odd number >= 5):");
    let width = read_number();
    assert!(width >= 5);
    println!("Enter height without tree trunk (number >= 3):");
    let height = read_number();
    assert!(width >= 3);
    print_crown(width,height);
    print_tree_trunk(width);
}

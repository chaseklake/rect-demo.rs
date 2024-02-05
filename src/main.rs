// Let’s make a new binary project that will take the width and height of a rectangle 
// specified in pixels and calculate the area of the rectangle.

fn main() {
    // Part 1: Bare Bones
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_bare_bones(width1, height1)
    );

    // Part 2: Custom Tuples
    let rect1 = (30u32, 50u32);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_tuples(rect1)
    );

    // Part 3: Perfectly balanced, as all structs should be
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_struct(&rect1)
    );

    println!("rect1 is {:?}", rect1);
}

// A bare-bones function. Width and Height should be combined into something!
fn area_bare_bones(width: u32, height: u32) -> u32 {
    width * height
}

// Using tuples. Still a bit too abstracted
fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Goldilocks:
#[derive(Debug)] // This is an attribute, used for printing the struct
struct Rectangle {
    width: u32,
    height: u32
}

// Perfect.
fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
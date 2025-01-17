use std::fmt;

#[derive(Debug)]
struct MinMax(i64,i64);
// earlier we got an error when we used the {} delimiter for struct
// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for MinMax {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}



impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

//Activity
//
#[derive(Debug)]
struct Complex{
    real:f64,
    imag:f64,
}

impl fmt::Display for Complex{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
         write!(f, "{} + {}i",self.real,self.imag)
    }
}
fn main()
{
    let minmax = MinMax(0,14);

    println!("Compare structures:");
    println!("Display: {}",minmax);
    println!("Debug: {:?}",minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3,3);

    println!("The big range is {big} and the small is {small}",small = small_range, big = big_range);

    let point = Point2D { x : 3.3, y: 7.2};

    println!("Compare points:");
    println!("Display : {}", point);
    println!("Debug: {:?}", point);

    let pnt = Complex {real : 3.3, imag : 7.2};
    //println!("Compare points:");
    println!("Display : {}", pnt);
    println!("Debug: {:?}", pnt);

}

struct UnPrintable(i32);

#[derive(Debug)]

struct DebugPrintable(i32);

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);


fn main()
{
    println!("{:?} months in a year.",12);
    //Note the difference between the next two statements
    //{:?} prints the argumetns as such
    //{} prints the value of the argument. Try it out!
    println!("{1:?} {0:?} is the {actor:?} name.","Slater","Christian",actor = "actor's");
    println!("{1} {0} is the {actor} name.","Slater","Christian",actor = "actor's");

    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));

}


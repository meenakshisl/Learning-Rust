fn main()
{
    println!("{} days",31);
    println!("{0}, this is  {1}. {1}. this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",subject="How", verb = "are", object="you?");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    //right-align
    println!("{number:>width$}",number=1,width=6);
    println!("My name is {0}, {1} {0}","Bond","James"); //FIX-ERR


    struct Structure(i32);

    //println!("This struct `{}` won't print...",Structure(3)); FIX ERR

    let pi = 3.141592;
    println!(" Pi is {:.3}",pi); //Activity 2
}

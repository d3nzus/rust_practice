fn main() {
    //  This variable is immutable.
    let x = 5;

    //  This variable is mutable.
    let mut y = 6;

    //  If we try to change the value of x, it would error.

    //  Constants cannot be mutable whatsover and the type of the constant must be annotated.
    const HOURS_IN_A_DAY: u32 = 24;

    //  Shadowing is the name of redeclaring a variable. 
    //  There is a difference between shadowing and simply reassigning the variable.
    //  It mainly lets you shadow in a curly bracket scope and return to the original value once it ends.

    let m = 5;
     
    {
        let m = m + 1;
        //  m is now 6
    }
    //  m is back to 5

    //  Think of it like temporary reassignment.

    
}

fn main() {
    println!("# CHAPTER 3, SECTION 5: Control Flow\n");


    println!("\n## `if` Statements");
    
    let number=17;

    if number<5 {
        println!("Condition was TRUE!");
    } else {
        println!("FALSE was the condition!");
    }

    if number!=0 {
        println!("The number was nonzero");
    }

    if number % 4==0 {
        println!("Number is divisible by 4");
    } else if number % 3==0 {
        println!("Number is divisible by 3");
    } else if number % 2==0 {
        println!("Number is divisible by 2");
    } else {
        println!("The number isn't divisible by 2, 3, or 4");
    }

    println!("\n###if in a let statement");
    let condition=true; //this was in the example. I started to play a bit. 
    let newnumber=if (number%2!=0) {"Odd"} else {"Even"};
    println!("The value is {newnumber}");

}
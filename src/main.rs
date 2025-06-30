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

    println!("\n## Loops");
    println!("\n### Basic Loops");
    //As per the book...mostly. It's an infinite loop, so I'm commenting it out after I run it once.
    //loop {
    //    println!("GOTO 10!");
    //}

    //Let's try it with a break statement
    let mut counter=0;

    let result=loop {
        counter+=1;
    //    println!("Counter: {counter}");   //I wanted to see what was going on. Noising when I run it. 

        if counter==10 {
            break counter*2;
        }
    };

    println!("The result is {result}");

    println!("\nLoop with a Label");
    counter=0;
    'counting_up: loop {
        println!("Counter={counter}");
        let mut remaining=10;
        loop {
            println!("\tRemaining={remaining}");
            if remaining==9 {
                break;
            }
            if counter==2 {
                break 'counting_up;
            }
            remaining -=1;
        }
        counter+=1;
    }

}
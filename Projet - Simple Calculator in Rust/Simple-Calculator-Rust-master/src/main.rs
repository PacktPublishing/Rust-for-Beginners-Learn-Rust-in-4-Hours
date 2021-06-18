fn main() {
    let mut choice = String::new();

    loop{
    calculator();
    println!("Do you want to continue (Y to continue and N to exit)");

    std::io::stdin()
        .read_line(&mut choice)
        .expect("Y to continue and N to exit");
    
    if choice.trim()=="N" {
        break;
    }
    }

}

fn calculator (){

    let mut number1 = String::new();
    let mut number2 = String::new();
    let mut operator = String::new();

    println!("Please enter the first number");
    std::io::stdin()
        .read_line(&mut number1)
        .expect("Failed to read first number.");

    println!("Please enter the second number");
    std::io::stdin()
        .read_line(&mut number2)
        .expect("Failed to read second number.");

    println!("Please enter the Operator (+ for addition, - for subtraction, * for multiplication and / for division)");

    std::io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read operator.");

    let num1: f64 = number1
        .trim()
        .parse()
        .expect("Failed to convert the Number 1, make sure it is a valid Number");
    let num2: f64 = number2
        .trim()
        .parse()
        .expect("Failed to convert the Number 1, make sure it is a valid Number");


    if operator.trim()=="+" {
        println!("The sum of {} and {} is {}",num1,num2,num1+num2)
    }
    else if operator.trim()=="-" {
        println!("The differnce of {} and {} is {}",num1,num2,num1-num2)
    }
    else if operator.trim()=="*" {
        println!("The product of {} and {} is {}",num1,num2,num1*num2)
    }
    else if operator.trim()=="/" {
        println!("The remainder of {} and {} is {}",num1,num2,num1/num2)
    }
    else{
        println!("Invalid Operator,Please enter the Operator (+ for addition, - for subtraction, * for multiplication and / for division) ")
    }

}

//We need to read two numbers from the user.
//We need to read the operation - addition, subtraction, multiplication or division, exponents.
//We need to perform that operation depending upon the choice of operation by the user.
//Display the result.

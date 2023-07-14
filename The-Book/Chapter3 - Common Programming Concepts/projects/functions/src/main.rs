fn function_no_return_type() {
    println!("Not returning");
}

fn function_no_return_statement()->i32 {
    if true {
        5
    } 
    else {
        println!("");
        2
    }
}

fn function_no_return_statement_variable() -> i32 {
    let a = 2;
    a
}

fn function_no_return_statement_invalid() -> i32 {
    if true {
        return 7; // wont work if return is removed
    }
    2
}

fn main() {
    function_no_return_type();
    println!("{}",function_no_return_statement());
    println!("{}",function_no_return_statement_variable());
    println!("{}",function_no_return_statement_invalid());

    let y = {
        let x = 3;
        x * x * x
    };

    println!("Value of y is {}",y)
    
}

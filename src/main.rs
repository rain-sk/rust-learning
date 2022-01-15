fn log_exercise(i: i32) {
    println!("\nExercise {}", i);
}

fn main() {
    log_exercise(1);
    {
        // variables
        let answer = 42;
        println!("Hello, world! {}", answer);
    }

    log_exercise(2);
    {
        // for loops
        for i in 1..5 {
            println!("Hello {}", i);
        }
    }

    log_exercise(3);
    {
        // if / else
        for i in 1..5 {
            if i % 2 == 0 {
                println!("even {}", i);
            } else {
                println!("odd {}", i);
            }
        }
    }

    log_exercise(3);
    {
        // if / else can be evaluated as an expression, use instead of ternary: x ? y : z
        for i in 1..5 {
            let even_odd = if i % 2 == 0 { "even" } else { "odd" };
            println!("{} {}", even_odd, i);
        }
    }

    log_exercise(5);
    {
        // declare variables as mutable so they can be modified after initialization
        let mut sum = 0;
        for i in 0..5 {
            sum += i;
        }
        println!("sum is {}", sum);
    }

    log_exercise(6);
    {
        // labmdas and functions have similar syntax for declaring return types
        // let sqr = |x: f64| -> f64 { return x * x; };
        // if the last expression in a function leaves off the semicolon,
        // the value of the expression will be returned
        fn sqr(x: f64) -> f64 {
            x * x
        }

        let res = sqr(2.0);
        println!("square is {}", res);
    }

    log_exercise(7);
    {
        // elegantly express recursive functions
        fn factorial(n: u64) -> u64 {
            if n == 0 {
                1
            } else {
                n * factorial(n - 1)
            }
        }

        let res = factorial(12);
        println!("factorial is {}", res);
    }

    log_exercise(8);
    {
        // values can be passed by reference (hybrid syntax of references and pointers in c++)
        fn by_ref(x: &i32) -> i32 {
            *x + 1
        }

        let i = 10;
        let res1 = by_ref(&i);
        let res2 = by_ref(&41);
        println!("{} {}", res1, res2);
    }

    log_exercise(9);
    {
        // values can be passed by reference (hybrid syntax of references and pointers in c++)
        fn modifies(x: &mut f64) {
            *x = 1.0;
        }

        let mut res = 0.0;
        modifies(&mut res);
        println!("res is {}", res);
    }

    log_exercise(10);
    {
        // type-after-variable style applies to let
        let bigint: i64 = 0;
        println!("bigint is {}", bigint);
    }
}

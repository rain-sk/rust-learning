// Exercises from: https://stevedonovan.github.io/rust-gentle-intro/

static mut EXERCISE_COUNTER: i32 = 0;

fn log_exercise<T>(exercise: T)
where
    T: Fn() -> (),
{
    unsafe {
        EXERCISE_COUNTER += 1;
        println!("\nExercise {}", EXERCISE_COUNTER);
    }
    exercise();
}

fn main() {
    log_exercise(|| {
        // variables and format strings
        let answer = 42;
        println!("Hello, world! {}", answer);
    });

    log_exercise(|| {
        // for loops and ranges
        for i in 1..5 {
            println!("Hello {}", i);
        }
    });

    log_exercise(|| {
        // if / else
        for i in 1..5 {
            if i % 2 == 0 {
                println!("even {}", i);
            } else {
                println!("odd {}", i);
            }
        }
    });

    log_exercise(|| {
        // if / else can be evaluated as an expression, use instead of ternary: x ? y : z
        for i in 1..5 {
            let even_odd = if i % 2 == 0 { "even" } else { "odd" };
            println!("{} {}", even_odd, i);
        }
    });

    log_exercise(|| {
        // declare variables as mutable so they can be modified after initialization
        let mut sum = 0;
        for i in 0..5 {
            sum += i;
        }
        println!("sum is {}", sum);
    });

    log_exercise(|| {
        // labmdas and functions have similar syntax for declaring return types
        // let sqr = |x: f64| -> f64 { return x * x; };
        // if the last expression in a function leaves off the semicolon,
        // the value of the expression is returned
        fn sqr(x: f64) -> f64 {
            x * x
        }

        let res = sqr(2.0);
        println!("square is {}", res);
    });

    log_exercise(|| {
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
    });

    log_exercise(|| {
        // values can be passed by reference (hybrid syntax of references and pointers in c++)
        fn by_ref(x: &i32) -> i32 {
            *x + 1
        }

        let i = 10;
        let res1 = by_ref(&i);
        let res2 = by_ref(&41);
        println!("{} {}", res1, res2);
    });

    log_exercise(|| {
        // values can be passed by reference (hybrid syntax of references and pointers in c++)
        fn modifies(x: &mut f64) {
            *x = 1.0;
        }

        let mut res = 0.0;
        modifies(&mut res);
        println!("res is {}", res);
    });

    log_exercise(|| {
        // type-after-variable style applies to let
        let bigint: i64 = 0;
        println!("bigint is {}", bigint);
    });
}

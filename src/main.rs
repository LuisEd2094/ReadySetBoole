use ready_set_boole::adder::adder;

fn run_adder() {
    let a: u32 = 2;
    let b: u32 = 3;
    let sum: u32 = adder(a, b);

    assert_eq!(sum, 5, "Adder function did not return the expected result!");

    println!("{} + {} = {}", a, b, sum);
}

fn main() {
    run_adder();
}

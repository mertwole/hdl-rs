use derive_macros::logic_generator;

fn adder() {
    #[logic_generator(for I in 0..10)]
    {
        println!("I: {}", I);
    }
}

#[test]
fn test_adder() {
    //
}

extern crate movable;

use movable::Movable;

// something  that doesn't implements Copy
#[derive(PartialEq, Debug)]
enum NoCopy{A, B}

fn main() {

    let lambda = |a: &NoCopy| match a
    {
	NoCopy::A => 0u8, _ => 1
    };

    let lol = Movable::new(NoCopy::B);
    println!("{:?}", lol);
    assert!(lol.use_to(lambda)==1);
    assert!(!lol.has_moved());
    assert!(lol.use_to(lambda)==1);
    assert!(lol.consume() == NoCopy::B);
    assert!(lol.has_moved());
    println!("{:?}", lol);

    let lol = Movable::new(NoCopy::A);
    println!("{:?}", lol);
    assert!(lol.use_to(lambda)==0);
    assert!(!lol.has_moved());
    assert!(lol.use_to(lambda)==0);
    assert!(lol.consume() == NoCopy::A);
    assert!(lol.has_moved());
    println!("{:?}", lol);
    

        
        
}

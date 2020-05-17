extern crate movable;

use movable::Movable;

#[derive(Debug)]
struct Something(u32);

impl Something
{
    fn update(self) -> Self
    {
	let mut moved = self;
	moved.0 += 1;
	moved
    }
}


struct OtherSomething;

impl OtherSomething
{
    fn on_event<F>(&self, mut f: F)
	where F: FnMut() -> ()
    {
	f()
    }
}

fn main()
{

    let othersomething = OtherSomething;

    let v = Something(0);
    let movable_v = Movable::new(v);
    println!("{:?}", movable_v);    
    othersomething.on_event(
	||
	{
	    let mut v = movable_v.consume();
	    v = v.update();
	    movable_v.insert(v);
	});
    println!("{:?}", movable_v);    

    othersomething.on_event(
	||
	{
	    movable_v.update_move(Something::update)
	}
    );
    println!("{:?}", movable_v);    
    
}


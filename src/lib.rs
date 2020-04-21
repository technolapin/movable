use std::cell::RefCell;

/**
A structure containing a single value that can be moved out imutably.
This can be used if you want a structure to be able to own something that will eventaly be moved, but you want your structure to keep existing.
*/
pub struct Movable<T>(RefCell<Vec<T>>);

impl<T> Movable<T>
{
    /// Creates a new Movable, initialized with the given value.
    pub fn new(content: T) -> Self
    {
	Self(RefCell::new(vec![content]))
    }
    /**
    Moves the internal value, emptying the Movable
    Panics if the Movable has been moved.
     */
    pub fn consume(&self) -> T
    {
	if self.has_moved()
	{
	    panic!("Movable already consumed!")
	}
	else
	{
	    self.0.borrow_mut().pop().unwrap()
	}
    }

    /**
    Returns true if the Movable's internal value has been moved out.
     */
    pub fn has_moved(&self) -> bool
    {
	self.0.borrow().len() == 0	
    }

    /**
    Applies the given closure to the internal content of the Movable.
    Panics if the Movable has been moved.
    The type U shouldn't contain any reference to the internal value unless you know what you do.
     */
    pub fn use_to<U, F>(&self, f: F) -> U
    where
	F: Fn(&T) -> U
    {
	if self.has_moved()
	{
	    panic!("Movable already consumed!")
	}
	else
	{
	    f(self.0.borrow()
	      .get(0).unwrap())
	}
    }
}

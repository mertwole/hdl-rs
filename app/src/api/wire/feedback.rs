use std::{cell::RefCell, rc::Rc};

use autoimpl_operators::WireBitwiseOps;

use super::{Wire, WireState};

#[derive(Clone, Copy, WireBitwiseOps)]
pub struct FeedbackWireOutput {
    // TODO: Store it in static var.
    //eval: Rc<RefCell<Option<Box<dyn Fn() -> WireState>>>>,
}

impl FeedbackWireOutput {
    pub fn new() -> Self {
        Self {
           // eval: Rc::default(),
        }
    }

    pub fn set_value<W: Wire + 'static>(&mut self, wire: W) {
        //let eval = move || wire.eval();
        //self.eval.borrow_mut().replace(Box::from(eval));
    }
}

impl Wire for FeedbackWireOutput {
    fn eval(&self) -> super::WireState {
        //self.eval.borrow().as_ref().unwrap()()
        todo!()
    }
}

pub trait Model<D> {
    fn new(data: D) -> Self;

    fn data(&self) -> &RefCell<D>;
}

use std::cell::RefCell;

pub struct CellModel<D> {
    data: RefCell<D>,
}

impl<D> Model<D> for CellModel<D> {
    fn new(data: D) -> Self {
        Self {
            data: RefCell::new(data),
        }
    }

    fn data(&self) -> &RefCell<D> {
        &self.data
    }
}

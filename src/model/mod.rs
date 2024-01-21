use crate::prelude::*;

pub trait Model<D: Clone> {
    fn new(data: D) -> Self;

    fn data(&self) -> &RefCell<D>;

    fn state(&self) -> Result<D>;
}

use std::{cell::RefCell, ops::Deref};

pub struct CellModel<D> {
    data: RefCell<D>,
}

impl<D: Clone> Model<D> for CellModel<D> {
    fn new(data: D) -> Self {
        Self {
            data: RefCell::new(data),
        }
    }

    fn data(&self) -> &RefCell<D> {
        &self.data
    }

    fn state(&self) -> Result<D> {
        let borrowed = self
            .data
            .try_borrow()
            .map_err(|e| Error::Generic("whoopsie".to_string()))?;

        Ok((*borrowed).clone())
    }
}

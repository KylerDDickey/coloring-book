use std::ops::Deref;
use std::rc::Rc;

use crate::{
    model::{CellModel, Model},
    prelude::*,
    view::{TestView, View},
};

pub trait HasModel<D: Clone, M: Model<D>> {
    fn model(&self) -> &impl Model<D>;
}

pub trait Controller<D: Clone, M: Model<D>>: HasModel<D, M> + Sized {
    fn render_view<I: From<D>, O, V: View<I, O>>(&self, view: &V) -> Result<O> {
        let state = self.model().state()?;

        Ok(view.render(state.into()))
    }
}

pub struct TestController {
    model: CellModel<String>,
}

impl TestController {
    pub fn new(model: CellModel<String>) -> Self {
        Self { model }
    }
}

impl HasModel<String, CellModel<String>> for TestController {
    fn model(&self) -> &impl Model<String> {
        &self.model
    }
}

impl Controller<String, CellModel<String>> for TestController {}

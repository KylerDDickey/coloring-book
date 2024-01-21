use crate::{
    model::{CellModel, Model},
    view::{TestView, View},
};

pub trait HasModel<D, M: Model<D>> {
    fn model(&self) -> &impl Model<D>;
}

pub trait HasView<I, O, D, M: Model<D>> {
    fn view(&self) -> &impl View<I, O, D, M>;
}

pub trait Controller<I, O, D, M: Model<D>, V: View<I, O, D, M>>:
    HasView<I, O, D, M> + HasModel<D, M> + Sized
{
    fn new(model: M, view: V) -> Self;
}

pub struct TestController {
    model: CellModel<String>,
    view: TestView,
}

impl<'a> HasView<&'a str, String, String, CellModel<String>> for TestController {
    fn view(&self) -> &impl View<&'a str, String, String, CellModel<String>> {
        &self.view
    }
}

impl HasModel<String, CellModel<String>> for TestController {
    fn model(&self) -> &impl Model<String> {
        &self.model
    }
}

impl Controller<&str, String, String, CellModel<String>, TestView> for TestController {
    fn new(model: CellModel<String>, view: TestView) -> Self {
        Self { model, view }
    }
}

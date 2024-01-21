use crate::{
    controller::{Controller, TestController},
    model::{CellModel, Model},
};

pub trait View<I, O, D, M: Model<D>>: Sized {
    fn bind_into_controller(input_config: I) -> impl Controller<I, O, D, M, Self>;

    fn render(&self) -> O;
}

pub struct TestView;

impl<'a> View<&'a str, String, String, CellModel<String>> for TestView {
    fn bind_into_controller(
        input_config: &'a str,
    ) -> impl Controller<&'a str, String, String, CellModel<String>, Self> {
        let model = CellModel::new(String::from(input_config));
        let mut view = Self {};
        TestController::new(model, view)
    }

    fn render(&self) -> String {
        String::from("test")
    }
}

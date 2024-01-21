use crate::{
    controller::{Controller, TestController},
    model::{CellModel, Model},
};

pub trait View<I, O>: Sized {
    fn render(&self, input: I) -> O;
}

pub struct TestView;

impl View<String, String> for TestView {
    fn render(&self, input: String) -> String {
        input
    }
}

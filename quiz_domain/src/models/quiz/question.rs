use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub trait ModelIDInterface<'a>:
    Eq + PartialEq + Deserialize<'a> + Serialize + Clone + Debug
{
    fn value(&self) -> String;
}

pub trait QuestionSetInterface<'a>: Debug + Deserialize<'a> + Serialize + Clone {
    fn id(&self) -> &ModelIDImpl;

    fn name(&self) -> &String;

    fn new<ID: Into<ModelIDImpl>>(id: ID, name: String) -> Self;
}

#[derive(Eq, PartialEq, Deserialize, Serialize, Clone, Debug)]
pub struct ModelIDImpl {
    id: u64,
}

impl ModelIDInterface<'_> for ModelIDImpl {
    fn value(&self) -> String {
        format!("{}", self.id)
    }
}

impl From<u64> for ModelIDImpl {
    fn from(n: u64) -> Self {
        ModelIDImpl::new(n)
    }
}

impl ModelIDImpl {
    fn new(value: u64) -> Self {
        ModelIDImpl { id: value }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuestionSetImpl {
    id: ModelIDImpl,
    name: String,
}

impl QuestionSetInterface<'_> for QuestionSetImpl {
    fn id(&self) -> &ModelIDImpl {
        &self.id
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn new<ID: Into<ModelIDImpl>>(id: ID, name: String) -> Self {
        QuestionSetImpl {
            id: id.into(),
            name,
        }
    }
}

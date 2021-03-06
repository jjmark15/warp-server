use crate::{ModelIDWithUUID, ModelId, QuestionSet, QuestionSetImpl};

pub trait ExampleQuizObjectsService {
    type QuestionSetID: ModelId + Default;
    type QuestionSetType: QuestionSet<QuestionSetId = Self::QuestionSetID>;

    fn get_example_question_set(&self) -> Self::QuestionSetType {
        Self::QuestionSetType::with_id(
            Self::QuestionSetID::default(),
            "Example question set title".to_string(),
        )
    }
}

#[derive(Debug, Default)]
pub struct ExampleQuizObjectsServiceImpl;

impl ExampleQuizObjectsService for ExampleQuizObjectsServiceImpl {
    type QuestionSetID = ModelIDWithUUID;
    type QuestionSetType = QuestionSetImpl;
}

impl ExampleQuizObjectsServiceImpl {
    pub fn new() -> Self {
        ExampleQuizObjectsServiceImpl
    }
}

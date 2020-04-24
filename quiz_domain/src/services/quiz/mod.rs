use crate::models::quiz::question::{QuestionSetImpl, QuestionSetInterface};

pub trait QuizServiceInterface<'a, QuestionSet: QuestionSetInterface<'a>> {
    fn get_example_question_set() -> QuestionSet {
        QuestionSet::with_name("Example question set title".to_string())
    }
}

pub struct QuizServiceImpl;

impl QuizServiceInterface<'_, QuestionSetImpl> for QuizServiceImpl {}

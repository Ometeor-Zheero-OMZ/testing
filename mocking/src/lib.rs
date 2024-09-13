use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait BigComputer {
    fn compute_answer(&self, question: i32) -> i32;
}

pub fn get_answer(computer: &impl BigComputer, question: i32) -> String {
    format!("The answer is {}", computer.compute_answer(question))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn answer_should_be_42() {
        let mut mock_computer = MockBigComputer::new();
        mock_computer
            .expect_compute_answer()
            .times(1)
            .return_const(42);

        let answer = get_answer(
            &mock_computer,
        1
        );

        assert_eq!(answer, "The answer is 42");
    }
}
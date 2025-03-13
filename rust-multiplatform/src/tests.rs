//! Tests for the rust-multiplatform framework

#[cfg(test)]
mod tests {
    use crate::{AppBuilder, RmpAppModel, RmpViewModel, register_app};
    use crossbeam::channel::{Sender, unbounded};
    use std::sync::Arc;

    // Define a test model update type
    #[derive(Debug, PartialEq)]
    enum TestModelUpdate {
        TestUpdate { value: i32 },
    }

    // Define a test action type
    #[derive(Debug, PartialEq)]
    enum TestAction {
        TestAction,
    }

    // Define a test model
    #[derive(Debug)]
    struct TestModel {
        pub count: i32,
        pub data_dir: String,
    }

    // Implement RmpAppModel for the test model
    impl RmpAppModel for TestModel {
        type ActionType = TestAction;
        type UpdateType = TestModelUpdate;

        fn create(data_dir: String) -> Self {
            TestModel {
                count: 0,
                data_dir,
            }
        }

        fn action(&mut self, action: Self::ActionType) {
            match action {
                TestAction::TestAction => self.count += 1,
            }
        }
    }

    // Define a test view model
    #[derive(Clone)]
    struct TestViewModel(Sender<TestModelUpdate>);

    // Use the register_app macro to generate the FFI code
    register_app!(TestModel, TestViewModel, TestAction, TestModelUpdate);

    #[test]
    fn test_register_app() {
        // The test is mainly to make sure the macro expands correctly
        // We can add more specific tests if needed
        assert!(true);
    }
}
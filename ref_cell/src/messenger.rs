pub mod messenger {
    pub trait Logger {
        fn warning(&self, msg: &str);
        fn info(&self, msg: &str);
        fn error(&self, msg: &str);
    }

    pub struct Tracker<'a, L: Logger> {
        logger: &'a L,
        value: usize,
        max: usize,
    }

    impl<'a, L: Logger> Tracker<'a, L> {
        pub fn new(logger: &'a L, max: usize) -> Self {
            Tracker {
                logger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;
            let percentage = self.value * 100 / self.max;
            if percentage >= 100 {
                self.logger.error("Error: you are over your quota!");
            } else if percentage >= 70 {
                self.logger.warning(&format!(
                    "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                    percentage
                ));
            }
        }

        pub fn peek(&self) {
            let percentage = self.value * 100 / self.max;
            self.logger.info(&format!(
                "Info: you are using up to {}% of your quota",
                percentage
            ));
        }
    }
}

pub struct Robot {
    robot_name: String,
}

fn random_name() -> String {
    "".to_string()
}

impl Robot {
    pub fn new() -> Self {
        Self {
            robot_name: random_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.robot_name
    }

    pub fn reset_name(&mut self) {
        self.robot_name = random_name();
    }
}

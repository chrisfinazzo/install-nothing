pub struct LogCorpus {
    success_logs: Vec<String>,
    error_logs: Vec<String>,
}

impl LogCorpus {
    pub fn new(success_content: &str, error_content: &str) -> Self {
        Self {
            success_logs: Self::parse(success_content),
            error_logs: Self::parse(error_content),
        }
    }

    fn parse(content: &str) -> Vec<String> {
        content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(String::from)
            .collect()
    }

    pub fn success_logs(&self) -> &[String] {
        &self.success_logs
    }

    pub fn error_logs(&self) -> &[String] {
        &self.error_logs
    }
}

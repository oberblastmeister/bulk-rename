pub enum ExitCode {
    Success,
    GeneralError,
}

impl From<ExitCode> for i32 {
    fn from(item: ExitCode) -> Self {
        match item {
            ExitCode::Success => 0,
            ExitCode::GeneralError => 1
        }
    }
}

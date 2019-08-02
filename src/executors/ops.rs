pub trait Ops {
    fn copy<S: AsRef<str>>(&self, file_name: S) -> Result<(), Box<dyn std::error::Error>>;
    fn copy_glob<S: AsRef<str>>(&self, pattern: S) -> Result<(), Box<dyn std::error::Error>>;
    fn execute<S: AsRef<str>>(&self, command: S) -> Result<(), Box<dyn std::error::Error>>;
}

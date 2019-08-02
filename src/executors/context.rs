use std::path::PathBuf;

pub trait Context: Sized {
    fn home(&self) -> Option<Self>;
    fn config(&self) -> Option<Self>;
    fn sub<S: AsRef<str>>(&self, sub: S) -> Self;
    fn search(&self, pattern: &str) -> Result<Vec<Option<Self>>, Box<dyn std::error::Error>>;
}

impl Context for PathBuf {
    fn home(&self) -> Option<Self> {
        dirs::home_dir()
    }

    fn config(&self) -> Option<Self> {
        dirs::config_dir()
    }

    fn sub<S: AsRef<str>>(&self, sub: S) -> Self {
        let mut s = self.clone();
        s.push(sub.as_ref());
        s
    }

    fn search(&self, pattern: &str) -> Result<Vec<Option<Self>>, Box<dyn std::error::Error>> {
        let full_pattern = format!(
            "{}{}{}",
            self.to_str().unwrap(),
            std::path::MAIN_SEPARATOR,
            pattern
        );

        Ok(glob::glob(full_pattern.as_ref())?
            .map(Result::ok)
            .collect::<Vec<_>>())
    }
}

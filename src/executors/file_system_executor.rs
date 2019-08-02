use super::context::Context;
use super::ops::Ops;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct FileSystemContext<C: Context> {
    source: C,
    target: C,
}

trait Executor {}

impl<C> FileSystemContext<C>
where
    C: Context + Default,
{
    pub fn new(target_dir: C) -> Self {
        FileSystemContext {
            source: C::default(),
            target: target_dir,
        }
    }
}

struct Executor<C>
where
    C: Context + Default,
{
    context: C,
}

impl Ops for Executor<PathBuf> {
    fn copy<S: AsRef<str>>(&self, file_name: S) -> Result<(), Box<dyn std::error::Error>> {
        let mut source = self.source.clone();
        source.push(file_name.as_ref());
        if !source.exists() {
            Ok(()); // TODO think of a better return type
        }

        let mut destination = self.target.clone();
        fs::create_dir_all(destination.as_path())?;
        destination.push(file_name.as_ref());
        fs::copy(source.as_path(), destination.as_path())?;
        Ok(())
    }

    fn copy_glob<S: AsRef<str>>(&self, pattern: S) -> Result<(), Box<dyn std::error::Error>> {
        let full_pattern = format!(
            "{}{}{}",
            self.source.to_str().unwrap(),
            std::path::MAIN_SEPARATOR,
            pattern.as_ref()
        );
        for entry in glob::glob(full_pattern.as_ref())? {
            if let Ok(path) = entry {
                let current = path.strip_prefix(self.source.as_path())?;
                let mut destination = self.target.clone();
                destination.push(current);

                println!(
                    "copying files matching {} from {} to {}",
                    pattern.as_ref(),
                    path.display(),
                    destination.display()
                );
            }
        }
        Ok(())
    }

    fn execute<S: AsRef<str>>(&self, command: S) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "executing '{}' in {}",
            command.as_ref(),
            self.source.display()
        );
        Ok(())
    }
}

impl Context for FileSystemContext<PathBuf> {
    fn home(&self) -> Option<Self> {
        let mut target = self.target.clone();
        target.push("home");
        self.source
            .home()
            .map(|source| FileSystemContext { source, target })
    }

    fn config(&self) -> Option<Self> {
        let mut target = self.target.clone();
        target.push("config");
        self.source
            .config()
            .map(|source| FileSystemContext { source, target })
    }

    fn sub<S: AsRef<str>>(&self, sub: S) -> Self {
        let FileSystemContext {
            mut source,
            mut target,
        } = self.clone();
        source.push(sub.as_ref());
        target.push(sub.as_ref());

        FileSystemContext { source, target }
    }

    fn search(&self, pattern: &str) -> Result<Vec<Option<Self>>, Box<dyn std::error::Error>> {
        let mut ret = vec![];
        let sources = self.source.search(pattern)?;
        for source in sources {
            if source.is_none() {
                continue;
            }
            let source = source.unwrap();
            let remaining = source.strip_prefix(self.source.as_path())?;
            let mut target = self.target.clone();
            target.push(remaining);
            ret.push(Some(FileSystemContext { source, target }))
        }
        Ok(ret)
    }
}

impl Default for FileSystemContext<PathBuf> {
    fn default() -> Self {
        FileSystemContext {
            source: PathBuf::default(),
            target: PathBuf::default(),
        }
    }
}

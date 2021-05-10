use chrono::{DateTime, TimeZone, Utc};
use shorthand::ShortHand;
use std::{
    collections::VecDeque,
    error::Error,
    fmt,
    fs::{read_dir, DirEntry},
    path::PathBuf,
    time::SystemTime,
};

#[derive(Debug)]
pub enum Status {
    Open,
    Closed,
    Unknown,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "Open"),
            Self::Closed => write!(f, "Closed"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(ShortHand, Debug)]
#[shorthand(disable(set))]
/// Represents a workspace.
pub struct Workspace {
    story: String,
    description: String,
    status: Status,
    modified: DateTime<Utc>,
    path: PathBuf,
}

impl Workspace {
    fn new(file: DirEntry) -> Result<Self, Box<dyn Error>> {
        let name = file.file_name().into_string().unwrap();
        let mut parts = name.split(".").collect::<VecDeque<_>>();
        let modified_time = file
            .metadata()?
            .modified()?
            .duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(Workspace {
            story: parts.pop_front().expect("story").to_string(),
            description: parts.pop_front().unwrap_or_default().to_string(),
            status: Status::Unknown,
            modified: Utc.timestamp(modified_time.as_secs() as i64, modified_time.subsec_nanos()),
            path: file.path(),
        })
    }
}

pub struct WorkspaceFactory {
    parent_dir: String,
}

impl WorkspaceFactory {
    pub fn new(parent_dir: String) -> Self {
        Self { parent_dir }
    }

    pub fn get_workspaces(&self) -> Result<Vec<Workspace>, Box<dyn Error>> {
        Ok(read_dir(&self.parent_dir)?
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .filter(|file| file.file_type().expect("file type").is_dir())
            .filter(|file| {
                !file
                    .file_name()
                    .into_string()
                    .expect("file name")
                    .starts_with(".")
            })
            .map(Workspace::new)
            .map(Result::unwrap)
            .collect())
    }
}

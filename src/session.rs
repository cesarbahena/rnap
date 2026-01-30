use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::app::{Dnap, GenomeId, InsulatorId, TfId};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Session {
    pub schema_version: u32,
    pub profile: String,
    pub actor: SessionActor,
    pub scope: SessionScope,
    pub issued_by: SessionIssuer,
    pub issued_at: SystemTime,
    pub expires_at: Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SessionActor {
    pub tf_id: TfId,
    pub subject: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SessionScope {
    pub insulator_id: InsulatorId,
    pub genome_id: GenomeId,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionIssuer {
    EpigeneticsLocal,
}

pub trait SessionProvider {
    fn current_session(&self) -> Result<Session, SessionError>;
}

#[derive(Serialize, Deserialize, Default)]
pub struct LocalState {
    pub dnap: Dnap,
    pub session: Option<Session>,
}

pub struct LocalStateStore {
    path: PathBuf,
}

impl Default for LocalStateStore {
    fn default() -> Self {
        Self {
            path: default_state_path(),
        }
    }
}

impl LocalStateStore {
    pub fn load(&self) -> Result<LocalState, SessionError> {
        if !self.path.exists() {
            return Ok(LocalState::default());
        }

        let content = fs::read_to_string(&self.path).map_err(SessionError::Io)?;
        serde_json::from_str(&content).map_err(SessionError::Serde)
    }

    pub fn save(&self, state: &LocalState) -> Result<(), SessionError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(SessionError::Io)?;
        }
        let content = serde_json::to_string_pretty(state).map_err(SessionError::Serde)?;
        fs::write(&self.path, content).map_err(SessionError::Io)
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl SessionProvider for LocalStateStore {
    fn current_session(&self) -> Result<Session, SessionError> {
        self.load()?.session.ok_or(SessionError::MissingSession)
    }
}

#[derive(Debug)]
pub enum SessionError {
    MissingHome,
    MissingSession,
    Io(io::Error),
    Serde(serde_json::Error),
}

impl fmt::Display for SessionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionError::MissingHome => write!(formatter, "home directory is not available"),
            SessionError::MissingSession => write!(formatter, "no active DNAp session"),
            SessionError::Io(error) => write!(formatter, "{error}"),
            SessionError::Serde(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for SessionError {}

fn default_state_path() -> PathBuf {
    if let Ok(xdg_state_home) = env::var("XDG_STATE_HOME") {
        return PathBuf::from(xdg_state_home)
            .join("dnap")
            .join("state.json");
    }

    env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".local")
        .join("state")
        .join("dnap")
        .join("state.json")
}

//! Manifest for Tydi projects.
//!
//! `tydi.toml` is a project's manifest file that contains all metadata and
//! configuration information about the project.
//!
//! The manifest file is inspired by Rust's [`Cargo.toml`].
//!
//! # Example
//!
//! This example `tydi.toml` shows all valid fields of a project manifest.
//!
//! ```toml
//! [project]                                      # Project metadata
//! name = "std"                                   # The project name
//! version = "0.1.0"                              # The project version (Semantic Versioning)
//! authors = ["Delft University of Technology"]   # List of project authors
//! description = "The Tydi standard library"      # Optional description of the project
//!
//! [dependencies]                                 # Dependencies configuration
//! axi = { path = "/axi" }                        # Example of a path dependency
//! wishbone = { git = "git@github.com:...", ... } # Example of a git dependency
//! ```
//!
//! [`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html

use semver::Version;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

/// The manifest file name (`tydi.toml`).
pub const FILE_NAME: &str = "tydi.toml";

/// A project's manifest with project metadata and dependency data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    /// The project metadata.
    project: ProjectMetadata,
    /// Optional list of dependencies.
    dependencies: Option<HashMap<String, Dependency>>,
}

impl Manifest {
    /// Returns the project metadata.
    pub fn project(&self) -> &ProjectMetadata {
        &self.project
    }

    /// Returns a reference to the map with dependencies, if there are any.
    pub fn dependencies(&self) -> Option<&HashMap<String, Dependency>> {
        self.dependencies.as_ref()
    }
}

/// A project with a name, authors and an optional description.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectMetadata {
    /// Name of the project.
    name: String,
    /// Version of the project.
    version: Version,
    /// List of authors.
    authors: Vec<String>,
    /// Optional description of the project.
    description: Option<String>,
}

impl ProjectMetadata {
    /// Returns the name of the project.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    /// Returns the version of the project.
    pub fn version(&self) -> String {
        self.version.to_string()
    }
    /// Returns the authors of the project.
    pub fn authors(&self) -> &[String] {
        self.authors.as_slice()
    }
    /// Returns the description of the project, if there is one.
    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(AsRef::as_ref)
    }
}

/// A dependency specification pointing to another project.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    /// Path specifier for dependency.
    /// Specifies a directory path to a project root.
    Path {
        /// Path to local project.
        path: PathBuf,
    },
    /// Git specifier for dependency.
    /// Specifies a git repository url and optionally a git reference.
    Git {
        /// Git repository url.
        git: String,
        /// Git reference used.
        #[serde(flatten)]
        reference: Option<GitReference>,
        // todo: add recursive flag?
    },
}

/// Git reference enumeration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum GitReference {
    /// Reference to a branch.
    Branch(String),
    /// Reference to a tag.
    Tag(String),
    /// Reference to revision (SHA-1 hash).
    Rev(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{de::DeserializeOwned, Deserialize};

    fn de_toml<T: DeserializeOwned>(input: &str) -> Result<T, toml::de::Error> {
        toml::from_str(input)
    }

    fn wrap_test<T: DeserializeOwned>(input: &str) -> Result<T, toml::de::Error> {
        #[derive(Deserialize)]
        struct Test<T> {
            test: T,
        }
        let input = format!("test = {}", &input);
        Ok(de_toml::<Test<T>>(&input)?.test)
    }

    #[test]
    fn git_reference() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            wrap_test::<GitReference>(r#"{branch = "main"}"#)?,
            GitReference::Branch("main".to_string())
        );
        assert_eq!(
            wrap_test::<GitReference>(r#"{tag = "v1.0.0"}"#)?,
            GitReference::Tag("v1.0.0".to_string())
        );
        assert_eq!(
            wrap_test::<GitReference>(r#"{rev = "c0ffee"}"#)?,
            GitReference::Rev("c0ffee".to_string())
        );
        Ok(())
    }

    #[test]
    fn dependency() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            wrap_test::<Dependency>(r#"{ git = "git@github.com:tydi-lang/tydi.git" }"#)?,
            Dependency::Git {
                git: "git@github.com:tydi-lang/tydi.git".to_string(),
                reference: None,
            }
        );
        assert_eq!(
            wrap_test::<Dependency>(
                r#"{ git = "git@github.com:tydi-lang/tydi.git", branch = "release-1" }"#
            )?,
            Dependency::Git {
                git: "git@github.com:tydi-lang/tydi.git".to_string(),
                reference: Some(GitReference::Branch("release-1".to_string())),
            }
        );
        assert_eq!(
            wrap_test::<Dependency>(
                r#"{ branch = "release-1", git = "git@github.com:tydi-lang/tydi.git" }"#
            )?,
            Dependency::Git {
                git: "git@github.com:tydi-lang/tydi.git".to_string(),
                reference: Some(GitReference::Branch("release-1".to_string())),
            }
        );
        assert_eq!(
            // Tag is ignored here.
            wrap_test::<Dependency>(
                r#"{ git = "git@github.com:tydi-lang/tydi.git", branch = "release-1", tag = "v1" }"#
            )?,
            Dependency::Git {
                git: "git@github.com:tydi-lang/tydi.git".to_string(),
                reference: Some(GitReference::Branch("release-1".to_string())),
            }
        );
        assert_eq!(
            wrap_test::<Dependency>(r#"{ path = "/tydi" }"#)?,
            Dependency::Path {
                path: PathBuf::from("/tydi"),
            }
        );
        assert_eq!(
            // Path is matched first (order in enum). Branch is ignored.
            wrap_test::<Dependency>(r#"{ git = "asdf", path = "/tydi", branch = "asdf" }"#)?,
            Dependency::Path {
                path: PathBuf::from("/tydi"),
            }
        );
        Ok(())
    }

    #[test]
    fn project_metadata() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            de_toml::<ProjectMetadata>(
                r#"
name = "test"
version = "0.1.0"
authors = ["a"]
"#
            )?,
            ProjectMetadata {
                name: "test".to_string(),
                version: Version::parse("0.1.0")?,
                authors: vec!["a".to_string()],
                description: None,
            }
        );
        assert_eq!(
            de_toml::<ProjectMetadata>(
                r#"
name = "test"
version = "0.1.0"
authors = []
description = "test"
"#
            )?,
            ProjectMetadata {
                name: "test".to_string(),
                version: Version::parse("0.1.0")?,
                authors: vec![],
                description: Some("test".to_string()),
            }
        );
        Ok(())
    }

    #[test]
    fn manifest() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            de_toml::<Manifest>(
                r#"
[project]
name = "test"
version = "0.1.0"
authors = ["a"]
"#
            )?,
            Manifest {
                project: ProjectMetadata {
                    name: "test".to_string(),
                    version: Version::parse("0.1.0")?,
                    authors: vec!["a".to_string()],
                    description: None,
                },
                dependencies: None,
            }
        );
        assert_eq!(
            de_toml::<Manifest>(
                r#"
[project]
name = "test"
version = "0.1.0"
authors = ["a"]

[dependencies]
"#
            )?,
            Manifest {
                project: ProjectMetadata {
                    name: "test".to_string(),
                    version: Version::parse("0.1.0")?,
                    authors: vec!["a".to_string()],
                    description: None,
                },
                dependencies: Some(HashMap::new()),
            }
        );
        let mut map = HashMap::new();
        map.insert(
            "a".to_string(),
            Dependency::Path {
                path: PathBuf::from("/tydi"),
            },
        );
        assert_eq!(
            de_toml::<Manifest>(
                r#"
[project]
name = "test"
version = "0.1.0"
authors = ["a"]

[dependencies]
a = { path = "/tydi" }
"#
            )?,
            Manifest {
                project: ProjectMetadata {
                    name: "test".to_string(),
                    version: Version::parse("0.1.0")?,
                    authors: vec!["a".to_string()],
                    description: None,
                },
                dependencies: Some(map),
            }
        );
        Ok(())
    }
}

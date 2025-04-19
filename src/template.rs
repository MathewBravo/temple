use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TemplateMetadata {
    id: String,
    name: String,
    vesion: semver::Version,
    description: String,
    tags: Vec<String>,
    variables: HashMap<String, TemplateVar>,
    post: PostRunPolicy,
    compat: Vec<semver::VersionReq>,
    docs: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateVar {
    default: Option<String>,
    prompt: String,
    ttype: String,
    pattern: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum PostRunPolicy {
    Always,
    Ask,
    Never,
}

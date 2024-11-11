use crate::semver::SemVer;
use anyhow::Result;
use octocrab::{params, Octocrab};

enum PullRequestKind {
    Feature,
    BugFix,
    HotFix,
    Refactor,
    Dependency,
}

enum PullRequestLabel {
    Major,
    Minor,
    Patch,
    CLI,
    Codegen,
    Expander,
    LanguageServer,
    Lower,
    Parser,
    ProjectManager,
    Tokenizer,
    Workflows,
    ReleaseAutomation,
}

struct PullRequest {
    number: u64,
    title: String,
    kind: PullRequestKind,
    labels: Vec<PullRequestLabel>,
}

pub struct ReleaseNote {
    version: SemVer,
    pull_requests: Vec<PullRequest>,
}

impl ReleaseNote {
    pub async fn new() -> Result<Self> {
        let octocrab = Octocrab::builder().build()?;
        let pulls = octocrab
            .pulls("brack-lang", "brack")
            .list()
            .base("develop")
            .state(params::State::Closed)
            .per_page(100)
            .send()
            .await?;
        for pull in &pulls.items {
            println!("{:?}", pull);
        }
        Ok(Self {
            version: SemVer::new(0, 0, 0),
            pull_requests: vec![],
        })
    }
}

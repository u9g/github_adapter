use octocrab::models::{issues::Issue, reactions::Reaction, Label};

#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    Account(()),
    Comment(()),
    Issue {
        issue: Issue,
        repo_owner: String,
        repo_name: String,
    },
    Label(Label),
    Organization(()),
    Reactions(Reaction),
    Repository {
        owner: String,
        name: String,
    },
    User(()),
}

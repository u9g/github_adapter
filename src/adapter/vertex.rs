use std::rc::Rc;

use octocrab::models::{issues::Issue, Label};

use super::reactions::Reactions;

#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    Account(()),
    Comment(()),
    Issue {
        issue: Box<Issue>,
        repo_owner: Rc<str>,
        repo_name: Rc<str>,
    },
    Label(Label),
    Organization(()),
    Reactions(Reactions),
    Repository {
        owner: Rc<str>,
        name: Rc<str>,
    },
    User(()),
}

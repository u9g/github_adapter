use std::{cell::RefCell, rc::Rc};

use octorust::types::{FullRepository, Issue, IssueComment, IssueSimple, Label, ReactionRollup};

#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    Account(AccountVertex),
    Comment(Box<IssueComment>),
    Issue(IssueVertex),
    Label(Label),
    Reactions(Option<ReactionRollup>),
    Repository(RepositoryVertex),
}

#[derive(Debug, Clone)]
pub struct AccountVertex {
    pub name: Rc<str>,
}

#[derive(Debug, Clone)]
pub struct RepositoryVertex {
    pub owner: Rc<str>,
    pub name: Rc<str>,
    pub repo_data: Box<RefCell<Option<FullRepository>>>,
}

impl RepositoryVertex {
    pub(crate) fn new(owner: Rc<str>, name: Rc<str>) -> Self {
        Self {
            owner,
            name,
            repo_data: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IssueVertex {
    pub simple_issue: Box<IssueSimple>,
    /// only gotten/used for reactions
    pub full_issue: Box<RefCell<Option<Issue>>>,
    pub owner: Rc<str>,
    pub name: Rc<str>,
}

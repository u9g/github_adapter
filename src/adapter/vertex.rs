use std::{cell::RefCell, rc::Rc};

use octorust::types::{Issue, IssueSimple, Label, ReactionRollup};

#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    Account(()),
    Comment(()),
    Issue(IssueVertex),
    Label(Label),
    Organization(()),
    Reactions(Option<ReactionRollup>),
    Repository { owner: Rc<str>, name: Rc<str> },
    User(()),
}

#[derive(Debug, Clone)]
pub struct IssueVertex {
    pub simple_issue: Box<IssueSimple>,
    /// only gotten/used for reactions
    pub full_issue: Box<RefCell<Option<Issue>>>,
    pub owner: Rc<str>,
    pub name: Rc<str>,
}

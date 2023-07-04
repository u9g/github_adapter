use trustfall::provider::{
    ContextIterator, ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo, VertexIterator,
};

use super::vertex::Vertex;

pub(super) fn resolve_comment_edge<'a>(
    contexts: ContextIterator<'a, Vertex>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, Vertex, VertexIterator<'a, Vertex>> {
    match edge_name {
        "by" => comment::by(contexts, resolve_info),
        "reactions" => comment::reactions(contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Comment'")
        }
    }
}

mod comment {
    use trustfall::provider::{
        resolve_neighbors_with, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn by<'a>(
        contexts: ContextIterator<'a, Vertex>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, Vertex, VertexIterator<'a, Vertex>> {
        todo!("implement edge 'by' for type 'Comment'")
    }

    pub(super) fn reactions<'a>(
        contexts: ContextIterator<'a, Vertex>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, Vertex, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, |v| {
            Box::new(std::iter::once(Vertex::Reactions(
                v.as_comment().expect("to have a comment").reactions.clone(),
            )))
        })
    }
}

pub(super) fn resolve_issue_edge<'a>(
    contexts: ContextIterator<'a, Vertex>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, Vertex, VertexIterator<'a, Vertex>> {
    match edge_name {
        "comment" => issue::comment(contexts, resolve_info),
        "label" => issue::label(contexts, resolve_info),
        "opened_by" => issue::opened_by(contexts, resolve_info),
        "reactions" => issue::reactions(contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Issue'")
        }
    }
}

mod issue {
    use trustfall::provider::{
        resolve_neighbors_with, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use crate::adapter::{generic_iterator::GenericIterator, util::client, vertex::IssueVertex};
    use async_std::task;

    use super::super::vertex::Vertex;

    pub(super) fn comment<'a>(
        contexts: ContextIterator<'a, Vertex>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, Vertex, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, |v| {
            let Vertex::Issue(IssueVertex {
                owner,
                name,
                simple_issue,
                ..
            }) = v
            else {
                unreachable!("expected to have a issue vertex")
            };
            let issue_number = simple_issue.number;

            let owner = owner.clone();
            let name = name.clone();
            let iter = GenericIterator::new(Box::new(move |page| {
                task::block_on(client().issues().list_comments(
                    &owner,
                    &name,
                    issue_number,
                    None,
                    100,
                    page,
                ))
                .expect("to get page of issues")
            }));
            Box::new(
                iter.into_iter()
                    .map(move |comment| Vertex::Comment(Box::new(comment))),
            )
        })
    }

    pub(super) fn label<'a>(
        contexts: ContextIterator<'a, Vertex>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, Vertex, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, |v| {
            Box::new(
                v.as_issue()
                    .expect("to have an issue")
                    .simple_issue
                    .labels
                    .clone()
                    .into_iter()
                    .map(Vertex::Label),
            )
        })
    }

    pub(super) fn opened_by<'a>(
        contexts: ContextIterator<'a, Vertex>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, Vertex, VertexIterator<'a, Vertex>> {
        todo!("implement edge 'opened_by' for type 'Issue'")
    }

    pub(super) fn reactions<'a>(
        contexts: ContextIterator<'a, Vertex>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, Vertex, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, |v| {
            let issue = v.as_issue().expect("to have an issue");

            if issue.full_issue.borrow().is_none() {
                let issue_data = task::block_on(client().issues().get(
                    &issue.owner,
                    &issue.name,
                    issue.simple_issue.number,
                ))
                .expect("to be able to get issue info from repo");
                issue.full_issue.replace(Some(issue_data));
            }

            let maybe_reactions = issue
                .full_issue
                .borrow()
                .as_ref()
                .unwrap()
                .reactions
                .clone();

            Box::new(std::iter::once(Vertex::Reactions(maybe_reactions)))
        })
    }
}

pub(super) fn resolve_repository_edge<'a>(
    contexts: ContextIterator<'a, Vertex>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, Vertex, VertexIterator<'a, Vertex>> {
    match edge_name {
        "issue" => repository::issue(contexts, resolve_info),
        "owner" => repository::owner(contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Repository'")
        }
    }
}

mod repository {
    use async_std::task;
    use std::cell::RefCell;

    use trustfall::provider::{
        resolve_neighbors_with, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use crate::adapter::{
        generic_iterator::GenericIterator, util::client, vertex::RepositoryVertex,
    };

    use super::super::vertex::Vertex;

    pub(super) fn issue<'a>(
        contexts: ContextIterator<'a, Vertex>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, Vertex, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, |v| {
            let Vertex::Repository(RepositoryVertex { owner, name, .. }) = v else {
                unreachable!("Need a repository in Repository.issue")
            };
            let owner = owner.clone();
            let name = name.clone();
            let owner2 = owner.clone();
            let name2 = name.clone();
            let iter = GenericIterator::new(Box::new(move |page| {
                task::block_on(client().issues().list_for_repo(
                    &owner,
                    &name,
                    "",
                    octorust::types::IssuesListState::All,
                    "",
                    "",
                    "",
                    "",
                    octorust::types::IssuesListSort::Updated,
                    octorust::types::Order::Desc,
                    None,
                    100,
                    page,
                ))
                .expect("to get page of issues")
            }));
            Box::new(
                iter.into_iter()
                    .filter(|issue| issue.pull_request.is_none())
                    .map(move |issue| {
                        Vertex::Issue(crate::adapter::vertex::IssueVertex {
                            simple_issue: Box::new(issue),
                            full_issue: RefCell::new(None).into(),
                            owner: owner2.clone(),
                            name: name2.clone(),
                        })
                    }),
            )
        })
    }

    pub(super) fn owner<'a>(
        contexts: ContextIterator<'a, Vertex>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, Vertex, VertexIterator<'a, Vertex>> {
        todo!("implement edge 'owner' for type 'Repository'")
    }
}

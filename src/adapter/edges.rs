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
        ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo, VertexIterator,
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
        todo!("implement edge 'reactions' for type 'Comment'")
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
    use std::rc::Rc;

    use futures::executor;
    use trustfall::provider::{
        resolve_neighbors_with, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn comment<'a>(
        contexts: ContextIterator<'a, Vertex>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, Vertex, VertexIterator<'a, Vertex>> {
        todo!("implement edge 'comment' for type 'Issue'")
    }

    pub(super) fn label<'a>(
        contexts: ContextIterator<'a, Vertex>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, Vertex, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, |v| {
            Box::new(
                v.as_issue()
                    .unwrap()
                    .0
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
            let issue_vertex = v.as_issue().expect("to have a issue vertex");
            let issue = octocrab::instance().issues(issue_vertex.1, issue_vertex.2);
            Box::new(
                executor::block_on(
                    issue
                        .list_reactions(issue_vertex.0.id.0)
                        .per_page(100)
                        .send(),
                )
                .expect("expect to be able to get reactions")
                .into_iter()
                .map(Vertex::Reactions),
            )
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
    use futures::executor;
    use trustfall::provider::{
        resolve_neighbors_with, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn issue<'a>(
        contexts: ContextIterator<'a, Vertex>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, Vertex, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, |v| {
            if let Vertex::Repository { owner, name } = v {
                Box::new(
                    executor::block_on(
                        octocrab::instance()
                            .issues(owner.to_string(), name.to_string())
                            .list()
                            .per_page(100)
                            .send(),
                    )
                    .expect("to find issues")
                    .into_iter()
                    .map(|v| Vertex::Issue {
                        issue: v,
                        repo_owner: owner.to_string(),
                        repo_name: name.to_string(),
                    }),
                )
            } else {
                unreachable!("attempted to resolve edge 'issue' on non-vertex 'Repository'")
            }
        })
    }

    pub(super) fn owner<'a>(
        contexts: ContextIterator<'a, Vertex>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, Vertex, VertexIterator<'a, Vertex>> {
        todo!("implement edge 'owner' for type 'Repository'")
    }
}

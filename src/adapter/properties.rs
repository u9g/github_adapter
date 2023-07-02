use octocrab::models::issues::IssueStateReason;
use trustfall::{
    provider::{resolve_property_with, ContextIterator, ContextOutcomeIterator, ResolveInfo},
    FieldValue,
};

use super::vertex::Vertex;

pub(super) fn resolve_account_property<'a>(
    contexts: ContextIterator<'a, Vertex>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, Vertex, FieldValue> {
    match property_name {
        "name" => todo!("implement property 'name' in fn `resolve_account_property()`"),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Account'"
            )
        }
    }
}

pub(super) fn resolve_comment_property<'a>(
    contexts: ContextIterator<'a, Vertex>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, Vertex, FieldValue> {
    match property_name {
        "message" => {
            todo!("implement property 'message' in fn `resolve_comment_property()`")
        }
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Comment'"
            )
        }
    }
}

pub(super) fn resolve_issue_property<'a>(
    contexts: ContextIterator<'a, Vertex>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, Vertex, FieldValue> {
    match property_name {
        "name" => resolve_property_with(contexts, |v| {
            v.as_issue()
                .expect("to have an issue")
                .0
                .title
                .to_string()
                .into()
        }),
        "description" => resolve_property_with(contexts, |v| {
            v.as_issue()
                .expect("to have an issue")
                .0
                .body
                .as_ref()
                .map_or_else(|| FieldValue::Null, Into::into)
        }),
        "state" => resolve_property_with(contexts, |v| {
            let issue_vertex = v.as_issue().unwrap();

            match &issue_vertex.0.state_reason {
                Some(reason) => match reason {
                    IssueStateReason::Completed => "closed:completed",
                    IssueStateReason::NotPlanned => "closed:not_planned",
                    IssueStateReason::Reopened => "opened:reopened",
                    _ => unreachable!(),
                },
                None => match issue_vertex.0.state {
                    octocrab::models::IssueState::Closed => "closed",
                    octocrab::models::IssueState::Open => "open",
                    _ => unreachable!(),
                },
            }
            .into()
        }),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Issue'")
        }
    }
}

pub(super) fn resolve_label_property<'a>(
    contexts: ContextIterator<'a, Vertex>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, Vertex, FieldValue> {
    match property_name {
        "name" => resolve_property_with(contexts, |v| v.as_label().unwrap().name.clone().into()),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Label'")
        }
    }
}

pub(super) fn resolve_organization_property<'a>(
    contexts: ContextIterator<'a, Vertex>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, Vertex, FieldValue> {
    match property_name {
        "name" => {
            todo!("implement property 'name' in fn `resolve_organization_property()`")
        }
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Organization'"
            )
        }
    }
}

pub(super) fn resolve_reactions_property<'a>(
    contexts: ContextIterator<'a, Vertex>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, Vertex, FieldValue> {
    match property_name {
        "confused" => {
            todo!("implement property 'confused' in fn `resolve_reactions_property()`")
        }
        "eyes" => todo!("implement property 'eyes' in fn `resolve_reactions_property()`"),
        "heart" => {
            todo!("implement property 'heart' in fn `resolve_reactions_property()`")
        }
        "hooray" => {
            todo!("implement property 'hooray' in fn `resolve_reactions_property()`")
        }
        "laugh" => {
            todo!("implement property 'laugh' in fn `resolve_reactions_property()`")
        }
        "minus_one" => {
            todo!("implement property 'minus_one' in fn `resolve_reactions_property()`")
        }
        "plus_one" => {
            todo!("implement property 'plus_one' in fn `resolve_reactions_property()`")
        }
        "rocket" => {
            todo!("implement property 'rocket' in fn `resolve_reactions_property()`")
        }
        "total" => resolve_property_with(contexts, |v| {
            v.as_reactions().expect("to have a reaction").total.into()
        }),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Reactions'"
            )
        }
    }
}

pub(super) fn resolve_repository_property<'a>(
    contexts: ContextIterator<'a, Vertex>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, Vertex, FieldValue> {
    match property_name {
        "name" => {
            todo!("implement property 'name' in fn `resolve_repository_property()`")
        }
        "stars" => {
            todo!("implement property 'stars' in fn `resolve_repository_property()`")
        }
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Repository'"
            )
        }
    }
}

pub(super) fn resolve_user_property<'a>(
    contexts: ContextIterator<'a, Vertex>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, Vertex, FieldValue> {
    match property_name {
        "name" => todo!("implement property 'name' in fn `resolve_user_property()`"),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'User'")
        }
    }
}

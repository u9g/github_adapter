use async_std::task;
use trustfall::{
    provider::{resolve_property_with, ContextIterator, ContextOutcomeIterator, ResolveInfo},
    FieldValue,
};

use super::{util::client, vertex::Vertex};

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
        "message" => resolve_property_with(contexts, |v| {
            v.as_comment()
                .expect("to have a comment")
                .body
                .clone()
                .into()
        }),
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
        "description" => resolve_property_with(contexts, |v| {
            v.as_issue()
                .expect("to have an issue")
                .simple_issue
                .body
                .to_string()
                .into()
        }),
        "name" => resolve_property_with(contexts, |v| {
            v.as_issue()
                .expect("to have an issue")
                .simple_issue
                .title
                .to_string()
                .into()
        }),
        "state" => resolve_property_with(contexts, |v| {
            v.as_issue()
                .expect("to have an issue")
                .simple_issue
                .state
                .to_string()
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
        "name" => {
            resolve_property_with(contexts, |v| v.as_label().unwrap().name.to_string().into())
        }
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
        "confused" => resolve_property_with(contexts, |v| {
            v.as_reactions()
                .unwrap()
                .as_ref()
                .map_or_else(|| 0, |reactions| reactions.confused)
                .into()
        }),
        "eyes" => resolve_property_with(contexts, |v| {
            v.as_reactions()
                .unwrap()
                .as_ref()
                .map_or_else(|| 0, |reactions| reactions.eyes)
                .into()
        }),
        "heart" => resolve_property_with(contexts, |v| {
            v.as_reactions()
                .unwrap()
                .as_ref()
                .map_or_else(|| 0, |reactions| reactions.heart)
                .into()
        }),
        "hooray" => resolve_property_with(contexts, |v| {
            v.as_reactions()
                .unwrap()
                .as_ref()
                .map_or_else(|| 0, |reactions| reactions.hooray)
                .into()
        }),
        "laugh" => resolve_property_with(contexts, |v| {
            v.as_reactions()
                .unwrap()
                .as_ref()
                .map_or_else(|| 0, |reactions| reactions.laugh)
                .into()
        }),
        "minus_one" => resolve_property_with(contexts, |v| {
            v.as_reactions()
                .unwrap()
                .as_ref()
                .map_or_else(|| 0, |reactions| reactions.minus_one)
                .into()
        }),
        "plus_one" => resolve_property_with(contexts, |v| {
            v.as_reactions()
                .unwrap()
                .as_ref()
                .map_or_else(|| 0, |reactions| reactions.plus_one)
                .into()
        }),
        "rocket" => resolve_property_with(contexts, |v| {
            v.as_reactions()
                .unwrap()
                .as_ref()
                .map_or_else(|| 0, |reactions| reactions.rocket)
                .into()
        }),
        "total" => resolve_property_with(contexts, |v| {
            v.as_reactions()
                .unwrap()
                .as_ref()
                .map_or_else(|| 0, |reactions| reactions.total_count)
                .into()
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
        "name" => resolve_property_with(contexts, |v| {
            v.as_repository()
                .expect("to have a repo")
                .name
                .as_ref()
                .into()
        }),
        "stars" => resolve_property_with(contexts, |v| {
            let repo = v.as_repository().expect("to have a repo");
            let repos = client().repos();
            let future = repos.get(&repo.owner, &repo.name);
            let full_repo_data = task::block_on(future).expect("to be able to fetch repo");
            repo.repo_data
                .replace(Some(full_repo_data))
                .as_ref()
                .expect("to have repo_data after we refreshed it")
                .stargazers_count
                .into()
        }),
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

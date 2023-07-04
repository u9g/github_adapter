use std::rc::Rc;

use async_std::task;
use trustfall::provider::{ResolveInfo, VertexIterator};

use super::{
    generic_iterator::GenericIterator,
    util::client,
    vertex::{RepositoryVertex, Vertex},
};

pub(super) fn repository<'a>(
    owner: &str,
    name: Option<&str>,
    _resolve_info: &ResolveInfo,
) -> VertexIterator<'a, Vertex> {
    if let Some(repo_name) = name {
        Box::new(std::iter::once(Vertex::Repository(RepositoryVertex::new(
            owner.into(),
            repo_name.into(),
        ))))
    } else {
        let owner_for_repo_iter: Rc<str> = owner.clone().into();
        let owner_for_returned_iter: Rc<str> = owner_for_repo_iter.clone();

        let iter = GenericIterator::new(Box::new(move |page| {
            task::block_on(client().repos().list_for_user(
                &owner_for_repo_iter,
                octorust::types::ReposListUserType::Owner,
                octorust::types::ReposListOrgSort::Updated,
                octorust::types::Order::Desc,
                100,
                page,
            ))
            .expect("to be able to get page of repos for user/org")
        }));

        Box::new(iter.map(move |v| {
            Vertex::Repository(RepositoryVertex::new(
                owner_for_returned_iter.clone(),
                v.name.into(),
            ))
        }))
    }
}

use trustfall::provider::{ResolveInfo, VertexIterator};

use super::vertex::{RepositoryVertex, Vertex};

pub(super) fn repository<'a>(
    owner: &str,
    name: Option<&str>,
    _resolve_info: &ResolveInfo,
) -> VertexIterator<'a, Vertex> {
    if let Some(repo_name) = name {
        Box::new(std::iter::once(Vertex::Repository(RepositoryVertex {
            owner: owner.into(),
            name: repo_name.into(),
            repo_data: Default::default(),
        })))
    } else {
        Box::new(std::iter::empty())
        // task::block_on(client().repos().list_all_for_user(
        //     &owner,
        //     &name,
        //     "",
        //     octorust::types::IssuesListState::All,
        //     "",
        //     "",
        //     "",
        //     "",
        //     octorust::types::IssuesListSort::Updated,
        //     octorust::types::Order::Desc,
        //     None,
        //     100,
        //     page,
        // ))
        // .expect("to get page of issues");
        // Box::new(std::iter::once(Vertex::Repository {
        //     owner: owner.into(),
        //     name: repo_name.into(),
        // }))
    }
}

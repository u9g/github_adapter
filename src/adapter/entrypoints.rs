use trustfall::provider::{ResolveInfo, VertexIterator};

use super::vertex::Vertex;

pub(super) fn repository<'a>(
    owner: &str,
    name: Option<&str>,
    _resolve_info: &ResolveInfo,
) -> VertexIterator<'a, Vertex> {
    Box::new(std::iter::once(Vertex::Repository {
        owner: owner.to_string(),
        name: name
            .map(|s| s.to_string())
            .expect("can't handle nullable names yet"),
    }))
}

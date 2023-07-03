use std::sync::{Arc, OnceLock};

use trustfall::{
    provider::{
        resolve_coercion_using_schema, ContextIterator, ContextOutcomeIterator, EdgeParameters,
        ResolveEdgeInfo, ResolveInfo, VertexIterator,
    },
    FieldValue, Schema,
};

use super::vertex::Vertex;

static SCHEMA: OnceLock<Schema> = OnceLock::new();

#[derive(Debug)]
pub struct Adapter {}

impl Adapter {
    pub const SCHEMA_TEXT: &'static str = include_str!("./schema.graphql");

    pub fn schema() -> &'static Schema {
        SCHEMA.get_or_init(|| Schema::parse(Self::SCHEMA_TEXT).expect("not a valid schema"))
    }
}

impl<'a> trustfall::provider::Adapter<'a> for Adapter {
    type Vertex = Vertex;

    fn resolve_starting_vertices(
        &self,
        edge_name: &Arc<str>,
        parameters: &EdgeParameters,
        resolve_info: &ResolveInfo,
    ) -> VertexIterator<'a, Self::Vertex> {
        match edge_name.as_ref() {
            "Repository" => {
                let owner: &str = parameters
                    .get("owner")
                    .expect(
                        "failed to find parameter 'owner' when resolving 'Repository' starting vertices",
                    )
                    .as_str()
                    .expect(
                        "unexpected null or other incorrect datatype for Trustfall type 'String!'",
                    );
                let name: Option<&str> = parameters
                    .get("name")
                    .expect(
                        "failed to find parameter 'name' when resolving 'Repository' starting vertices",
                    )
                    .as_str();
                super::entrypoints::repository(owner, name, resolve_info)
            }
            _ => {
                unreachable!(
                    "attempted to resolve starting vertices for unexpected edge name: {edge_name}"
                )
            }
        }
    }

    fn resolve_property(
        &self,
        contexts: ContextIterator<'a, Self::Vertex>,
        type_name: &Arc<str>,
        property_name: &Arc<str>,
        resolve_info: &ResolveInfo,
    ) -> ContextOutcomeIterator<'a, Self::Vertex, FieldValue> {
        match type_name.as_ref() {
            "Account" => super::properties::resolve_account_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Comment" => super::properties::resolve_comment_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Issue" => super::properties::resolve_issue_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Label" => super::properties::resolve_label_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Organization" => super::properties::resolve_organization_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Reactions" => super::properties::resolve_reactions_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Repository" => super::properties::resolve_repository_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "User" => super::properties::resolve_user_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            _ => {
                unreachable!(
                    "attempted to read property '{property_name}' on unexpected type: {type_name}"
                )
            }
        }
    }

    fn resolve_neighbors(
        &self,
        contexts: ContextIterator<'a, Self::Vertex>,
        type_name: &Arc<str>,
        edge_name: &Arc<str>,
        parameters: &EdgeParameters,
        resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, Self::Vertex, VertexIterator<'a, Self::Vertex>> {
        match type_name.as_ref() {
            "Comment" => super::edges::resolve_comment_edge(
                contexts,
                edge_name.as_ref(),
                parameters,
                resolve_info,
            ),
            "Issue" => super::edges::resolve_issue_edge(
                contexts,
                edge_name.as_ref(),
                parameters,
                resolve_info,
            ),
            "Repository" => super::edges::resolve_repository_edge(
                contexts,
                edge_name.as_ref(),
                parameters,
                resolve_info,
            ),
            _ => {
                unreachable!(
                    "attempted to resolve edge '{edge_name}' on unexpected type: {type_name}"
                )
            }
        }
    }

    fn resolve_coercion(
        &self,
        contexts: ContextIterator<'a, Self::Vertex>,
        _type_name: &Arc<str>,
        coerce_to_type: &Arc<str>,
        _resolve_info: &ResolveInfo,
    ) -> ContextOutcomeIterator<'a, Self::Vertex, bool> {
        resolve_coercion_using_schema(contexts, Self::schema(), coerce_to_type.as_ref())
    }
}

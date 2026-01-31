use serde::Serialize;

#[derive(Serialize)]
pub struct GraphQLRequest<V: Serialize> {
    pub query: &'static str,
    pub variables: V,
}

pub mod config;

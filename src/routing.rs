use std::collections::HashMap;

use crate::{
    layers::{Layer, LayerGroup},
    systems::DynSystem,
    Request, Response,
};

pub struct Router {
    root: Scope,
    request_layer: Box<dyn Layer<Request> + Send + Sync>,
    response_layer: Box<dyn Layer<Response> + Send + Sync>,
}

impl Router {
    pub fn new(root: impl Into<Scope>) -> Self {
        Router {
            root: root.into(),
            request_layer: Box::new(LayerGroup::new()),
            response_layer: Box::new(LayerGroup::new()),
        }
    }

    pub fn with_layers(
        root: impl Into<Scope>,
        request_layer: impl 'static + Layer<Request> + Send + Sync,
        response_layer: impl 'static + Layer<Response> + Send + Sync,
    ) -> Self {
        Self {
            root: root.into(),
            request_layer: Box::new(request_layer),
            response_layer: Box::new(response_layer),
        }
    }

    pub fn scope(&self) -> &Scope {
        &self.root
    }

    pub fn request_layer(&self) -> &dyn Layer<Request> {
        self.request_layer.as_ref()
    }

    pub fn response_layer(&self) -> &dyn Layer<Response> {
        self.response_layer.as_ref()
    }
}

/// A Node in the Router tree.
pub struct Scope {
    children: HashMap<String, Scope>,
    systems: Vec<DynSystem>,
}

impl Scope {
    /// Construct a new `Scope`
    pub fn new(systems: Vec<DynSystem>) -> Self {
        Self {
            children: HashMap::new(),
            systems,
        }
    }

    /// Construct an empty `Scope`
    pub fn empty() -> Self {
        Scope::new(vec![])
    }

    /// Add a `Scope` as a child of this node
    pub fn route(mut self, path: impl Into<String>, route: impl Into<Scope>) -> Self {
        self.children.insert(path.into(), route.into());

        self
    }

    /// Access the list of systems associated with this node
    pub fn systems(&self) -> &[DynSystem] {
        &self.systems
    }

    /// Scope to a child of this node by path
    pub fn get_child<'a>(&'a self, path: &str) -> Option<&'a Scope> {
        self.children.get(path)
    }
}

impl From<Vec<DynSystem>> for Scope {
    fn from(value: Vec<DynSystem>) -> Self {
        Scope::new(value)
    }
}

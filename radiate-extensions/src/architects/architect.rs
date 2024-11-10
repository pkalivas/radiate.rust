
use crate::architects::nodes::node::Node;
use crate::architects::node_factory::NodeFactory;
use crate::architects::node_collections::node_collection::NodeCollection;
use crate::architects::node_collection_builder::NodeCollectionBuilder;
use crate::architects::schema::node_types::NodeType;


pub struct Architect<C, T>
where
    C: NodeCollection<C, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    pub node_factory: NodeFactory<T>,
    _phantom: std::marker::PhantomData<C>,
}

impl<C, T> Architect<C, T>
where
    C: NodeCollection<C, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    pub fn new(node_factory: NodeFactory<T>) -> Architect<C, T> {
        Architect {
            node_factory,
            _phantom: std::marker::PhantomData
        }
    }

    pub fn build<F>(&self, build_fn: F) -> C
    where
        F: FnOnce(&Architect<C, T>, NodeCollectionBuilder<C, T>) -> C
    {
        build_fn(self, NodeCollectionBuilder::new(&self.node_factory))
    }

    pub fn input(&self, siez: usize) -> C {
        self.new_collection(NodeType::Input, siez)
    }

    pub fn output(&self, siez: usize) -> C {
        self.new_collection(NodeType::Output, siez)
    }

    pub fn gate(&self, siez: usize) -> C {
        self.new_collection(NodeType::Gate, siez)
    }

    pub fn aggregate(&self, siez: usize) -> C {
        self.new_collection(NodeType::Aggregate, siez)
    }

    pub fn weight(&self, siez: usize) -> C {
        self.new_collection(NodeType::Weight, siez)
    }

    fn new_collection(&self, node_type: NodeType, size: usize) -> C {
        let nodes = self.new_nodes(node_type, size);
        C::from_nodes(nodes)
    }

    fn new_nodes(&self, node_type: NodeType, size: usize) -> Vec<Node<T>> {
        (0..size)
            .map(|i| self.node_factory.new_node(i, node_type))
            .collect::<Vec<Node<T>>>()
    }
}

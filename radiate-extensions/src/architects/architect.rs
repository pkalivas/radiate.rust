
use crate::architects::nodes::node::Node;
use crate::architects::node_factory::NodeFactory;
use crate::architects::node_collections::node_collection::NodeCollection;


pub struct Architect<C, N, T>
where
    C: NodeCollection<C, N, T> + Clone + Default,
    N: Node<N, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    pub node_factory: NodeFactory<T>,
    _phantom_c: std::marker::PhantomData<C>,
    _phantom_n: std::marker::PhantomData<N>,
}

impl<C, N, T> Architect<C, N, T>
where
    C: NodeCollection<C, N, T> + Clone + Default,
    N: Node<N, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    pub fn new(node_factory: NodeFactory<T>) -> Architect<C, N, T> {
        Architect {
            node_factory,
            _phantom_c: std::marker::PhantomData,
            _phantom_n: std::marker::PhantomData,
        }
    }

    pub fn create_node_collection(&self) -> C {
        C::new()
    }
}
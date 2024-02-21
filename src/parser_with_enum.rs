pub enum TestEnum {
    A,
    B(String),
}

pub type PoolRef = usize;
#[derive(Default)]
pub struct Pool {
    pool: Vec<Node>
}

impl Pool {
    pub fn add(&mut self, node: Node) -> PoolRef {
        self.pool.push(node);
        self.pool.len() - 1
    }

    pub fn get(&self, pool_ref: PoolRef) -> &Node {
        &self.pool[pool_ref]
    }

    pub fn get_mut(&mut self, pool_ref: PoolRef) -> &mut Node {
        self.pool.get_mut(pool_ref).unwrap()
    }
}

pub struct Node {
    parent: Option<PoolRef>,
    children: Vec<PoolRef>,
    node_type: TestEnum
}

impl Node {
    pub fn new(node_type: TestEnum) -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            node_type,
        }
    }

    pub fn set_parent(&mut self, parent: PoolRef) {
        self.parent = Some(parent);
    }
}

pub struct ParserTree {
    pool: Pool,
    root_ref: PoolRef,
}

impl ParserTree {
    pub fn new(root: TestEnum) -> Self {
        let root_node = Node::new(root);
        let mut pool = Pool::default();
        let root_ref = pool.add(root_node);
        Self {
            pool,
            root_ref,
        }
    }

    pub fn add(&mut self, child: TestEnum, parent: PoolRef) -> PoolRef {
        let mut child_node = Node::new(child);
        child_node.set_parent(parent);
        self.pool.add(child_node)
    }
}



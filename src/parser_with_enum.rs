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
        let child_ref = self.pool.add(child_node);
        let parent_node = self.pool.get_mut(parent);
        parent_node.children.push(child_ref);
        child_ref
    }

    pub fn change_b(&mut self, node_ref: PoolRef, new_value: String) -> Result<(), &'static str> {

        let node_type = &mut self.pool.get_mut(node_ref).node_type;
        match node_type {
            TestEnum::A => {
                return Err("Expected TestEnum B, but found TestEnum A")
            },
            TestEnum::B(ref mut string) => {
                *string = new_value;
            }
        }

        Ok(())
    }
}


#[test]
pub fn test_change_b() {
    let mut tree = ParserTree::new(TestEnum::B("A".to_string()));
    
    match &tree.pool.get(0).node_type {
        TestEnum::A => assert!(false),
        TestEnum::B(str) => assert_eq!(*str, "A".to_string())
    }

    match tree.change_b(0, "B".to_string()) {
        Ok(()) => {},
        Err(_str) => assert!(false),
    }

    match &tree.pool.get(0).node_type {
        TestEnum::A => assert!(false),
        TestEnum::B(str) => assert_eq!(*str, "B".to_string())
    }

}
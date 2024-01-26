

// struct NodeData<T> {
//     data: T
// }

pub struct QuadTree<T> {
    //node_data: NodeData<T>
    pub nodes_list: Vec<T>
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZNodeIndex {
    None, NorthWest, NorthEast, SouthWest, SouthEast
}

impl From<ZNodeIndex> for usize {
    fn from(value: ZNodeIndex) -> Self {
        match value {
            ZNodeIndex::None => 0,
            ZNodeIndex::NorthWest => 1,
            ZNodeIndex::NorthEast => 2,
            ZNodeIndex::SouthWest => 3,
            ZNodeIndex::SouthEast => 4,
        }
    }
}

impl Into<ZNodeIndex> for usize {
    fn into(self) -> ZNodeIndex {
        match self {
            0 => ZNodeIndex::None,
            1 => ZNodeIndex::NorthWest,
            2 => ZNodeIndex::NorthEast,
            3 => ZNodeIndex::SouthWest,
            4 => ZNodeIndex::SouthEast,
            _ => panic!("not possible :D")
        }
    }
}

impl<T> QuadTree<T> {
    pub fn new() -> Self {
        Self { nodes_list: vec!() }
    }

    pub fn index(root: usize, number: ZNodeIndex) -> usize {
        let local_index: usize  = number.into();
        return (root * 4) + local_index;
    }

    pub fn inverse_index(global_index: usize) -> (usize, ZNodeIndex) {
        let mut local_index = (global_index as i64  )  %  4;//if global_index  % 5 == 0  { None } else { Some(global_index  % 5 - 1) };
        local_index -= 1;
        if local_index < 0 {
            local_index = 3;
        }
       
        
        if global_index == 0 {
            local_index = 0;
        } else {
            local_index += 1;
        }

        let root_index = (global_index - local_index as usize) / 4;

        let local_index : ZNodeIndex = (local_index as usize).into();
        (root_index, local_index )
    }

    pub fn children_indices_byroot(root: usize) -> std::ops::Range<usize> {
        let start = (root * 4) + 1;
        let end  = (root * 4) + (3 + 1);
        std::ops::Range { start, end }
    }


    pub fn children_indices(globa_index: usize) -> std::ops::Range<usize> {
        let start = globa_index + 1;
        let end  = start + 4;
        //println!("cc {} {}", start, end);
        std::ops::Range { start, end }
    }
    

    pub fn index_by_node(root: usize, inode: ZNodeIndex) -> usize {
        let local_number: usize = inode.into();
        return (root * 4) + local_number ;
    }

    pub fn parent_index(index: usize) -> Option<usize>
    /* Returns the parent index of a node in an array-based binary tree.
  
    Args:
      index: The index of the node.
      n: The total number of nodes in the tree.
  
    Returns:
      The parent index of the node, or -1 if the node is the root node.
    */
    {


        if index == 0 {
            None
        }
        else {
            Some((index - 1) / 4)
        }
    }
    pub fn set(&mut self, root: usize,  inode: ZNodeIndex, value: T) where T : Default 
    {
        //println!("SET {}, {:?}", root, inode);
        //let i = QuadTree::<T>::index_by_node(root, inode);
        let i = QuadTree::<T>::index(root, inode);
        self.ensure_capacity(i+1);
        self.nodes_list[i] = value;
    }

    pub fn set_root(&mut self, value: T) where T : Default 
    {
        let i = 0;//QuadTree::<T>::index_by_node(root, inode);
        self.ensure_capacity(i+1);
        self.nodes_list[i] = value;
    }

    // pub fn get_by_root(&self, root: usize, local_index: usize) -> T where T : Clone
    // {
    //     let i = QuadTree::<T>::index(root, local_index);
    //     self.nodes_list[i].clone()
    // }

    pub fn get_by_root2(&self, root: usize, local_index: Option<usize>) -> T where T : Clone
    {
        let i = (root * 4) + if local_index.is_none() {0} else { local_index.unwrap() };
        self.nodes_list[i].clone()
    }

    


    pub fn ensure_capacity(&mut self, minlength: usize) where T : Default
    {
        if minlength > self.nodes_list.len() {
            self.nodes_list.resize_with(minlength, Default::default);
        }
    }

    pub fn print_tree(&self) where T : std::fmt::Debug
    {
        for (i, node) in self.nodes_list.iter().enumerate() {
            println!("{}: {:?}", i, node);
        }
    }

    pub fn len(&self) -> usize {  self.nodes_list.len()  }
}

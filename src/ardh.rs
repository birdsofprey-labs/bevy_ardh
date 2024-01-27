use std::collections::{HashSet, VecDeque};
use bevy::{prelude::*};
use crate::quadtree::{ZNodeIndex, QuadTree};

#[derive(Component, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TileId {
    pub address: usize,
    pub face: usize,
}


#[derive(Component)]
pub struct ArdhFlat {
    pub face: usize,
    pub local_tx: Transform,
    pub size: f32,
    pub stree: SearchTree,
    pub lod_tx: Transform
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub parent_copy: Option<Box<Node>>,
    pub id: usize,
    pub tx: Transform,
    pub index: ZNodeIndex,
    pub size: f32,
    pub uv_offset: Vec2,
    pub uv_scale: f32,
    pub depth: usize,
    pub face: usize
    //address: TileId
}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl std::cmp::Eq for Node {

}

pub type QT = QuadTree<Option<Node>>;


pub struct SearchTree {
    pub tree: QT,
    pub running: bool,
    pub stack: VecDeque<Node>,
    pub discovered: HashSet<Node>,
    pub leafs: HashSet<Node>,
    pub leafs_prev: HashSet<Node>,
    //pub root: Option<Node>,
}
impl SearchTree {
pub fn dfs(&mut self, subdiv_fn: impl Fn(&Node) -> bool) -> bool {
    // create a queue for doing BFS
    //let mut q = VecDeque::<usize>::new();
    // let mut v = self.root.clone().unwrap();
    // // mark the source vertex as discovered
    // self.discovered.insert(v.clone());
    // self.stack.push_back(v);

    // if tree.nodes_list[v].is_some() {
    //     println!("v: {}, {:?}", v, QT::inverse_index(v));

    //     if  QT::inverse_index(v).1 == ZNodeIndex::None || v < 50 && QT::inverse_index(v).1 == ZNodeIndex::SouthEast {
    //         let x = v;
    //         tree.set(x, gaia::ZNodeIndex::NorthEast, Some(123));
    //         tree.set(x, gaia::ZNodeIndex::SouthEast, Some(123));
    //         tree.set(x, gaia::ZNodeIndex::NorthWest, Some(123));
    //         tree.set(x, gaia::ZNodeIndex::SouthWest, Some(123));

    //     }
    // }

    // loop till queue is empty
    if self.stack.is_empty()
    {
        self.running = false;

        // for leaf in &self.leafs {
        //     if let Some(leaf) = leaf.parent_copy.clone() {
        //         self.stack.push_back( *leaf.clone() );
        //     }
        // }
        self.stack.push_back(self.tree.nodes_list[0].clone().unwrap());
        self.discovered.clear();
        return true;
    }
    
        // dequeue front node and print it

        //if tree.nodes_list[v].is_none() { continue }

        let v = self.stack.pop_front().unwrap();
        //println!("test");
        //println!("xv: {:?} = {:?}", QT::inverse_index(v), tree.nodes_list[v]);

        // do for every edge (v, u)
        let dosubdiv =  subdiv_fn(&v);
        let sub_nodes = (1..=4usize).map(|li| QT::index(v.id, li.into()));
        for u in sub_nodes {

        
            //println!("v: {}, {:?}", v, QT::inverse_index(u));
            if dosubdiv || u < self.tree.nodes_list.len() + 4 {
                // && tree.nodes_list[u].is_some() {
                // QT::inverse_index(v).1 == ZNodeIndex::None &&
                
                let parent_copy = Some(Box::new(v.clone())); 
                
                let nscale = Vec3::ONE * 0.5;

                // NW = xy
                // NE = xy + np.array([wh[0] // 2, 0])
                // SW = xy + np.array([0, wh[1] // 2])
                // SE = xy + wh / 2
                
                let bor = Transform::IDENTITY;// Transform::from_rotation(Quat::from_axis_angle(Vec3::X, 180f32.to_radians()) );
                
                let uv_scale = v.uv_scale / 2.0;
                let tdim = uv_scale;
                if dosubdiv {
                    let size = v.size / 1.0;
                    let depth = v.depth + 1;
                    let index = ZNodeIndex::NorthWest;
                    let tx = bor * v.tx * Transform::IDENTITY.with_scale(nscale).with_translation(Vec3::new(-size, 0.0, size));
                    let node = Node { uv_scale,  uv_offset: v.uv_offset, id: QT::index(v.id, index), tx, index, size, depth, parent_copy: parent_copy.clone(), face: v.face };
                    self.tree.set(v.id, index, Some(node));

                    let index = ZNodeIndex::NorthEast;
                    let tx = bor *  v.tx * Transform::IDENTITY.with_scale(nscale).with_translation(Vec3::new(size, 0.0, size));
                    let node = Node {  uv_scale,  uv_offset: v.uv_offset + Vec2::new(tdim / 2.0, 0.0), id: QT::index(v.id, index), tx, index, size, depth, parent_copy: parent_copy.clone(), face: v.face };
                    self.tree.set(v.id, index, Some(node));

                    let index = ZNodeIndex::SouthEast;
                    let tx = bor * v.tx * Transform::IDENTITY.with_scale(nscale).with_translation(Vec3::new(size, 0.0, -size));
                    let node = Node { uv_scale,  uv_offset: v.uv_offset + Vec2::new(0.0, tdim / 2.0), id: QT::index(v.id, index), tx, index, size, depth, parent_copy: parent_copy.clone(), face: v.face };
                    self.tree.set(v.id, index, Some(node));

                    let index = ZNodeIndex::SouthWest;
                    let tx = bor * v.tx * Transform::IDENTITY.with_scale(nscale).with_translation(Vec3::new(-size, 0.0, -size));
                    let node = Node { uv_scale,  uv_offset: v.uv_offset + Vec2::new(tdim / 2.0, tdim / 2.0), id: QT::index(v.id, index), tx, index, size, depth, parent_copy: parent_copy.clone(), face: v.face };
                    self.tree.set(v.id, index, Some(node));
                let unode = self.tree.nodes_list[u].clone().unwrap();
                if !self.discovered.contains(&unode) {
                    //println!("  v: {:?} {}", v, u);

                    // mark it as discovered and enqueue it
                    self.discovered.insert(unode.clone());
                    // self.leafs.insert(unode.clone());
                    self.stack.push_back(unode.clone());

            //        self.leafs.remove(&v);
                }

                self.leafs.insert(unode.clone());
        
                self.leafs.remove(&v);
        
            }
             


            }
        }
        
        if v.id < self.tree.nodes_list.len() {
            //println!("xv2: @{} of @{:?} {:?} = {:?}", v.id, QT::parent_index(v.id), QT::inverse_index(v.id), self.tree.nodes_list[v.id]);
        }
        return false;
}
}
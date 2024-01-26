use std::collections::{HashSet, VecDeque};

use ardh::quadtree::{QuadTree, ZNodeIndex};



type Node = Option<i32>;
type QT = QuadTree<Node>;
type NodeId = usize; //(usize, Option<usize>);

fn DFS(tree: &mut QT, v: NodeId, discovered: &mut HashSet<NodeId>) {
    // create a queue for doing BFS
    let mut q = VecDeque::<NodeId>::new();
    let mut v = v;
    // mark the source vertex as discovered
    discovered.insert(v);

    // enqueue source vertex

    //if tree.get_by_root2(v.0, v.1).is_none() { return; }
    q.push_back(v);

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
    while !q.is_empty() {
        // dequeue front node and print it

        //if tree.nodes_list[v].is_none() { continue }

        v = q.pop_front().unwrap();
        //println!("test");
        //println!("xv: {:?} = {:?}", QT::inverse_index(v), tree.nodes_list[v]);

        // do for every edge (v, u)

        let sub_nodes = (1..=4usize).map(|li| QT::index(v, li.into()));
        for u in sub_nodes {

        
            //println!("v: {}, {:?}", v, QT::inverse_index(u));
            if u < tree.nodes_list.len() + 4 {
                // && tree.nodes_list[u].is_some() {
                // QT::inverse_index(v).1 == ZNodeIndex::None &&
                if v < 2 && QT::inverse_index(u).1 == ZNodeIndex::SouthEast {
                    //println!("Z: {}", u);
                    let x = v;
                    //tree.set(x, gaia::ZNodeIndex::NorthEast, Some(123));
                    tree.set(x, ZNodeIndex::SouthEast, Some(123));
                    //tree.set(x, gaia::ZNodeIndex::NorthWest, Some(123));
                    //tree.set(x, gaia::ZNodeIndex::SouthWest, Some(123));
                }
                if !discovered.contains(&u) {
                    //println!("  v: {:?} {}", v, u);

                    // mark it as discovered and enqueue it
                    discovered.insert(u);
                    q.push_back(u);

                    

                }

            }
        }
        
        if v < tree.nodes_list.len() {
            println!("xv2: @{} of @{:?} {:?} = {:?}", v, QT::parent_index(v), QT::inverse_index(v), tree.nodes_list[v]);
        }
    }
}

fn main() {
    let mut tree = QT::new();
    tree.set_root(Some(123));

    let x = 0;
    // tree.set(x, gaia::ZNodeIndex::NorthEast, Some(123));
    // tree.set(x, gaia::ZNodeIndex::SouthEast, Some(123));
    // tree.set(x, gaia::ZNodeIndex::NorthWest, Some(123));
    // tree.set(x, gaia::ZNodeIndex::SouthWest, Some(123));

    tree.print_tree();

    // to keep track of whether a vertex is discovered or not
    let mut discovered = HashSet::<NodeId>::new();

    // Perform BFS traversal from all undiscovered nodes to
    // cover all connected components of a graph
    println!("length {}", tree.len());

    // start BFS traversal from vertex `i`
    DFS(&mut tree, 0, &mut discovered);
}
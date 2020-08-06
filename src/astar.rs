use std::{cell::RefCell, error::Error, rc::Rc};

use crate::{components::EntitySize, game_map::GameMap};

const ADJACENT_SQUARES: [(isize, isize); 8] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

#[derive(Debug)]
struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    position: (isize, isize),
    g: isize,
    h: isize,
    f: isize,
}

impl Node {
    fn new(
        parent: Option<Rc<RefCell<Node>>>,
        position: (isize, isize),
    ) -> Self {
        Self {
            parent,
            position,
            g: 0,
            h: 0,
            f: 0,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

fn make_path(start_node: Rc<RefCell<Node>>) -> Vec<(isize, isize)> {
    let mut path = vec![];

    let mut current = Some(start_node);

    while let Some(in_curr) = current {
        path.push(in_curr.borrow().position);

        if let Some(parent) = in_curr.borrow().parent.as_ref() {
            current = Some(parent.clone());
        } else {
            current = None;
        }
    }

    return path;
}

// this is bad lol
pub fn astar(
    game_map: &GameMap,
    start: (isize, isize),
    end: (isize, isize),
) -> Result<Option<Vec<(isize, isize)>>, Box<dyn Error>> {
    let max_iter = game_map.map_info.total_count / 2;

    let map_len = game_map.render_map.len() as isize;

    let start_node = Node::new(None, start);
    let start_node = Rc::new(RefCell::new(start_node));

    let end_node = Node::new(None, end);
    let end_node = Rc::new(RefCell::new(end_node));

    let mut open_list: Vec<Rc<RefCell<Node>>> = vec![];

    open_list.push(start_node);

    let mut closed_list: Vec<Rc<RefCell<Node>>> = vec![];

    let mut outer_i = 0;
    // loop till we find the end
    while open_list.len() > 0 {
        outer_i += 1;

        if outer_i > max_iter {
            return Ok(None);
        }

        // get the item
        let item = open_list.first().unwrap();
        let current_pos = item.borrow().position;
        let mut current_f = item.borrow().f;
        let mut index = 0;
        // if we fond the end
        if current_pos == end_node.borrow().position {
            return Ok(Some(make_path(item.clone())));
        }

        for (i, node) in open_list.iter().enumerate() {
            if current_f > node.borrow().f {
                index = i;

                current_f = node.borrow().f;
            }
        }

        let item = open_list.remove(index);
        let current_pos = item.borrow().position;

        // if we fond the end
        if current_pos.0 == end_node.borrow().position.0
            && current_pos.1 == end_node.borrow().position.1
        {
            return Ok(Some(make_path(item)));
        }

        // add current_node to closed_list
        closed_list.push(item.clone());

        // generate children
        let mut children: Vec<Rc<RefCell<Node>>> = vec![];

        // make potential children
        for new_pos in ADJACENT_SQUARES.iter() {
            let node_position: (isize, isize) =
                (current_pos.0 + new_pos.0, current_pos.1 + new_pos.1);

            if node_position.0 < 0
                || node_position.0 > map_len
                || node_position.1 < 0
                || node_position.1 > map_len
            {
                continue;
            }

            let node_index = node_position.0
                + (game_map.map_info.column_count as isize * node_position.1);

            // check if can move
            if game_map.render_map[node_index as usize].ent_size
                < EntitySize::Medium
                || node_position == end_node.borrow().position
            {
                let new_node = Rc::new(RefCell::new(Node::new(
                    Some(item.clone()),
                    node_position,
                )));

                children.push(new_node);
            }
        }

        // add children with filter to open list
        'children: for child in children {
            let mut child_inner = child.borrow_mut();

            for cc in &closed_list {
                if child_inner.position == cc.borrow().position {
                    continue 'children;
                }
            }

            child_inner.g = item.borrow().g + 1;

            child_inner.h =
                (child_inner.position.0 - end_node.borrow().position.0).pow(2)
                    + (child_inner.position.1 - end_node.borrow().position.1)
                        .pow(2);

            child_inner.f = child_inner.g + child_inner.h;

            for o_node in &open_list {
                if child_inner.position == o_node.borrow().position
                    && child_inner.g > o_node.borrow().g
                {
                    continue 'children;
                }
            }

            open_list.push(child.clone());
        }
    }

    // i guess we get her if we scan everything and cant find the target?
    return Err(Box::from("we should not get here"));
}

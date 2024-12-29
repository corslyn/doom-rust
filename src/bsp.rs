use crate::Node;

pub fn is_on_left_side(x: i16, y: i16, nodes: &Vec<Node>, node_id: usize) -> bool {
    let node = &nodes[node_id];

    let vec_to_x = x - node.x_start;
    let vec_to_y = y - node.y_start;

    let determinant =
        (node.dx_start as i32 * vec_to_y as i32) - (node.dy_start as i32 * vec_to_x as i32);

    determinant > 0
}

pub fn find_player_node(x: i16, y: i16, nodes: &Vec<Node>, current_node_id: i16) -> &Node {
    // Get the current node
    let current_node = &nodes[current_node_id as usize];

    // Check if the point is on the left or right side of the partition line
    if is_on_left_side(x, y, nodes, current_node_id as usize) {
        // Traverse the left child
        if current_node.l_child >= 0 {
            // Recur for the left child
            find_player_node(x, y, nodes, current_node.l_child)
        } else {
            // If left child is a leaf or invalid, return the current node
            current_node
        }
    } else {
        // Traverse the right child
        if current_node.r_child >= 0 {
            // Recur for the right child
            find_player_node(x, y, nodes, current_node.r_child)
        } else {
            // If right child is a leaf or invalid, return the current node
            current_node
        }
    }
}

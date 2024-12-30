use crate::{
    config::SUBSECTOR_IDENTIFIER, render::draw_segment, Node, Player, Segment, Subsector, Vertex,
};

pub fn is_on_left_side(player: &Player, node: &Node) -> bool {
    let dx = player.pos.0 as i32 - node.x_start as i32;
    let dy = player.pos.1 as i32 - node.y_start as i32;
    let dy_start = node.dy_start as i32;
    let dx_start = node.dx_start as i32;

    dx * dy_start - dy * dx_start < 0
}
/* pub fn render_bsp_node(
    subsectors: &Vec<Subsector>,
    nodes: &Vec<Node>,
    node_id: i32,
    player: &Player,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    remap_x: &Box<dyn Fn(i16) -> i32>,
    remap_y: &Box<dyn Fn(i16) -> i32>,
) {
    if node_id >= 0x8000 {
        let subsector_id = node_id - 0x8000;
        // render_subsector(subsectors, subsector_id, canvas, remap_x, remap_y);
        return;
    }

    let node = &nodes[node_id as usize];
    let (first, second) = if is_on_left_side(player, node) {
        (node.l_child, node.r_child)
    } else {
        (node.r_child, node.l_child)
    };

    for &child_id in &[first, second] {
        render_bsp_node(
            subsectors, nodes, child_id, player, canvas, remap_x, remap_y,
        );
    }
} */

fn render_bsp_node(
    subsectors: &Vec<Subsector>,
    nodes: &Vec<Node>,
    node_id: u16,
    segments: &Vec<Segment>,
    vertexes: &Vec<Vertex>,
    player: &Player,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    remap_x: &Box<dyn Fn(i16) -> i32>,
    remap_y: &Box<dyn Fn(i16) -> i32>,
) {
    if (node_id & SUBSECTOR_IDENTIFIER) != 0 {
        render_subsector(
            subsectors,
            node_id & (!SUBSECTOR_IDENTIFIER),
            segments,
            vertexes,
            canvas,
            remap_x,
            remap_y,
        );
        return;
    }

    let is_on_left = is_on_left_side(player, &nodes[node_id as usize]);

    if is_on_left {
        render_bsp_node(
            subsectors,
            nodes,
            nodes[node_id as usize].l_child.try_into().unwrap(),
            segments,
            vertexes,
            player,
            canvas,
            remap_x,
            remap_y,
        );
        render_bsp_node(
            subsectors,
            nodes,
            nodes[node_id as usize].r_child.try_into().unwrap(),
            segments,
            vertexes,
            player,
            canvas,
            remap_x,
            remap_y,
        );
    } else {
        render_bsp_node(
            subsectors,
            nodes,
            nodes[node_id as usize].r_child.try_into().unwrap(),
            segments,
            vertexes,
            player,
            canvas,
            remap_x,
            remap_y,
        );
        render_bsp_node(
            subsectors,
            nodes,
            nodes[node_id as usize].l_child.try_into().unwrap(),
            segments,
            vertexes,
            player,
            canvas,
            remap_x,
            remap_y,
        );
    }
}

pub fn render(
    subsectors: &Vec<Subsector>,
    nodes: &Vec<Node>,
    segments: &Vec<Segment>,
    vertexes: &Vec<Vertex>,

    player: &Player,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    remap_x: &Box<dyn Fn(i16) -> i32>,
    remap_y: &Box<dyn Fn(i16) -> i32>,
) {
    render_bsp_node(
        subsectors,
        nodes,
        (nodes.len() - 1).try_into().unwrap(),
        segments,
        vertexes,
        player,
        canvas,
        remap_x,
        remap_y,
    );
}

pub fn render_subsector(
    subsectors: &Vec<Subsector>,
    subsector_id: u16,
    segments: &Vec<Segment>,
    vertexes: &Vec<Vertex>,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    remap_x: &Box<dyn Fn(i16) -> i32>,
    remap_y: &Box<dyn Fn(i16) -> i32>,
) {
    let subsector = &subsectors[subsector_id as usize];
    for i in 0..subsector.seg_count {
        let seg = &segments[(subsector.first_seg as usize) + i as usize];
        draw_segment(vertexes, seg, subsector_id as i16, canvas, remap_x, remap_y);
    }
}

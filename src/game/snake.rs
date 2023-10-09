use super::position::Point;

pub type SnakeNode = Point<f32>;
pub type SnakeBody = Vec<SnakeNode>;

pub struct Snake {
    body: SnakeBody
}

impl Snake {
    pub fn new(area_points: &[SnakeNode]) -> Self {
        let mut body = Vec::new();
        body.extend_from_slice(area_points);
        if Self::is_valid_body(&body) {
            Self {
                body,
            }
        } else {
            panic!("Invalid area points for initializing snake body");
        }
    }
    
    pub fn body(&self) -> &SnakeBody {
        &self.body
    }
    
    fn is_unique_node(node: &SnakeNode, body: &SnakeBody) -> bool {
        for body_node in body.iter() {
            if body_node.is_equal(node) {
                return false;
            } 
        }
        true
    }
    
    fn is_spaced_node(node: &SnakeNode, last_node: Option<&SnakeNode>, next_node: Option<&SnakeNode>) -> bool {
        let is_spaced_to_last_node = if let Some(unwrapped_last_node) = last_node {
            node.distance(unwrapped_last_node) == 0.1
        } else {true};
        let is_spaced_to_next_node = if let Some(unwrapped_next_node) = next_node {
            node.distance(unwrapped_next_node) == 0.1
        } else {true};
        is_spaced_to_last_node & is_spaced_to_next_node
    }
    
    fn is_valid_body(body: &SnakeBody) -> bool {
        // 1. Should not share the same point
        // 2. Should be one unit away from the next and last point
        let mut is_all_unique = true;
        let mut is_all_spaced = true;
        for (index, body_node) in body.iter().enumerate() {
            let last_node = body.get(index - 1);
            let next_node = body.get(index + 1);
            if !Self::is_unique_node(body_node, body) {
                is_all_unique = false;
            };
            if !Self::is_spaced_node(body_node, last_node, next_node) {
                is_all_spaced = false;
            };
        }
        is_all_unique & is_all_spaced
    }
}
/// Simple 3x3 matrix for 2D affine transformations.
/// Stored as:
/// [ m11  m12  m13 ]
/// [ m21  m22  m23 ]
/// [  0    0    1  ]  (implicit)
#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    pub m11: f32,
    pub m12: f32,
    pub m13: f32, // tx
    pub m21: f32,
    pub m22: f32,
    pub m23: f32, // ty
}

impl Mat3 {
    pub fn identity() -> Self {
        Mat3 {
            m11: 1.0, m12: 0.0, m13: 0.0,
            m21: 0.0, m22: 1.0, m23: 0.0,
        }
    }

    pub fn from_translation(x: f32, y: f32) -> Self {
        Mat3 {
            m11: 1.0, m12: 0.0, m13: x,
            m21: 0.0, m22: 1.0, m23: y,
        }
    }

    pub fn from_rotation(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Mat3 {
            m11: cos, m12: -sin, m13: 0.0,
            m21: sin, m22:  cos, m23: 0.0,
        }
    }

    pub fn from_scale(sx: f32, sy: f32) -> Self {
        Mat3 {
            m11: sx,  m12: 0.0, m13: 0.0,
            m21: 0.0, m22: sy,  m23: 0.0,
        }
    }

    /// Multiply self * other (self applied first, then other).
    pub fn mul(&self, other: &Mat3) -> Mat3 {
        Mat3 {
            m11: self.m11 * other.m11 + self.m12 * other.m21,
            m12: self.m11 * other.m12 + self.m12 * other.m22,
            m13: self.m11 * other.m13 + self.m12 * other.m23 + self.m13,
            m21: self.m21 * other.m11 + self.m22 * other.m21,
            m22: self.m21 * other.m12 + self.m22 * other.m22,
            m23: self.m21 * other.m13 + self.m22 * other.m23 + self.m23,
        }
    }

    /// Transform a point (x, y).
    pub fn transform_point(&self, x: f32, y: f32) -> (f32, f32) {
        (
            self.m11 * x + self.m12 * y + self.m13,
            self.m21 * x + self.m22 * y + self.m23,
        )
    }
}

/// Local transform of a scene node.
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,   // radians
    pub scale_x: f32,
    pub scale_y: f32,
}

impl Transform {
    pub fn new(x: f32, y: f32, rotation: f32, scale_x: f32, scale_y: f32) -> Self {
        Transform { x, y, rotation, scale_x, scale_y }
    }

    /// Build the local transformation matrix: T * R * S
    pub fn to_matrix(&self) -> Mat3 {
        Mat3::from_translation(self.x, self.y)
            .mul(&Mat3::from_rotation(self.rotation))
            .mul(&Mat3::from_scale(self.scale_x, self.scale_y))
    }
}

/// A node in the scene graph tree.
#[derive(Debug, Clone)]
pub struct SceneNode {
    pub name: String,
    pub local_transform: Transform,
    pub children: Vec<SceneNode>,
}

impl SceneNode {
    pub fn new(name: &str, transform: Transform) -> Self {
        SceneNode {
            name: name.to_owned(),
            local_transform: transform,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: SceneNode) {
        self.children.push(child);
    }

    /// Compute the world matrix for this node given the parent's world matrix.
    /// If `parent_world` is `None`, the identity matrix is used (root).
    pub fn world_matrix(&self, parent_world: Option<&Mat3>) -> Mat3 {
        let local = self.local_transform.to_matrix();
        match parent_world {
            Some(pw) => pw.mul(&local),
            None => local,
        }
    }
}

/// A simple scene graph containing one or more root nodes.
#[derive(Debug, Clone)]
pub struct SceneGraph {
    pub roots: Vec<SceneNode>,
}

impl SceneGraph {
    pub fn new() -> Self {
        SceneGraph { roots: Vec::new() }
    }

    pub fn add_root(&mut self, node: SceneNode) {
        self.roots.push(node);
    }

    /// Traverse the entire graph in deterministic depth‑first pre‑order,
    /// returning a vector of (node_name, world_matrix) for every node.
    pub fn world_matrices(&self) -> Vec<(String, Mat3)> {
        let mut result = Vec::new();
        for root in &self.roots {
            traverse(root, &Mat3::identity(), &mut result);
        }
        result
    }
}

fn traverse(node: &SceneNode, parent_world: &Mat3, out: &mut Vec<(String, Mat3)>) {
    let world = parent_world.mul(&node.local_transform.to_matrix());
    out.push((node.name.clone(), world));
    for child in &node.children {
        traverse(child, &world, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn identity_chain() {
        let t = Transform::new(0.0, 0.0, 0.0, 1.0, 1.0);
        let node = SceneNode::new("root", t);
        let world = node.world_matrix(None);
        assert!((world.m11 - 1.0).abs() < 1e-6);
        assert!((world.m22 - 1.0).abs() < 1e-6);
        assert!((world.m13).abs() < 1e-6);
        assert!((world.m23).abs() < 1e-6);
    }

    #[test]
    fn translation_child() {
        let mut root = SceneNode::new("root", Transform::new(10.0, 0.0, 0.0, 1.0, 1.0));
        root.add_child(SceneNode::new("child", Transform::new(5.0, 0.0, 0.0, 1.0, 1.0)));

        let root_world = root.world_matrix(None);
        assert!((root_world.m13 - 10.0).abs() < 1e-6);

        let child = &root.children[0];
        let child_world = child.world_matrix(Some(&root_world));
        assert!((child_world.m13 - 15.0).abs() < 1e-6);
    }

    #[test]
    fn scale_and_rotation() {
        let t = Transform::new(0.0, 0.0, PI / 2.0, 2.0, 1.0);
        let node = SceneNode::new("s", t);
        let world = node.world_matrix(None);
        let (px, py) = world.transform_point(1.0, 0.0);
        // After scaling x by 2 then rotating 90°: (2,0) -> (0,2) approx.
        assert!((px).abs() < 1e-6);
        assert!((py - 2.0).abs() < 1e-6);
    }

    #[test]
    fn graph_traversal_order() {
        let mut graph = SceneGraph::new();
        let mut root = SceneNode::new("A", Transform::new(0.0, 0.0, 0.0, 1.0, 1.0));
        root.add_child(SceneNode::new("B", Transform::new(1.0, 0.0, 0.0, 1.0, 1.0)));
        root.add_child(SceneNode::new("C", Transform::new(0.0, 1.0, 0.0, 1.0, 1.0)));
        graph.add_root(root);
        let worlds = graph.world_matrices();
        let names: Vec<&str> = worlds.iter().map(|(n,_)| n.as_str()).collect();
        assert_eq!(names, vec!["A", "B", "C"]);
        // World positions: A=(0,0), B=(1,0), C=(0,1)
        assert!((worlds[0].1.m13 - 0.0).abs() < 1e-6);
        assert!((worlds[1].1.m13 - 1.0).abs() < 1e-6);
        assert!((worlds[2].1.m13 - 0.0).abs() < 1e-6);
        assert!((worlds[2].1.m23 - 1.0).abs() < 1e-6);
    }
}
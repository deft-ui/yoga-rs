use std::collections::HashMap;
use ordered_float::OrderedFloat;
use yoga::{Node, NodeRef, StyleUnit};
use yoga::Direction::LTR;
use yoga::internal::{YGNodeGetChild, YGNodeGetChildCount, YGNodeLayoutGetHeight, YGNodeLayoutGetLeft, YGNodeLayoutGetTop, YGNodeLayoutGetWidth, YGNodeRef, YGNodeSetHasNewLayout};

pub struct Tree {
    names: HashMap<NodeRef, String>,
    a: Node,
    b: Node,
    d: Node,
    e: Node,
    f: Node,
    g: Node,
    h: Node,
    i: Node,
    j: Node,
    k: Node,
    l: Node,
    m: Node,
}

impl Tree {
    pub fn new_node(leaf: bool) -> Node {
        let mut node = Node::new();
        if leaf {
            node.set_width(StyleUnit::Point(OrderedFloat(100.0)));
            node.set_height(StyleUnit::Point(OrderedFloat(100.0)));
        }
        node
    }
    pub fn new() -> Self {
        let mut a = Self::new_node(false);
        let mut b = Self::new_node(false);
        let mut d = Self::new_node(false);
        let mut e = Self::new_node(true);
        let mut f = Self::new_node(false);
        let mut g = Self::new_node(false);
        let mut h = Self::new_node(true);
        let mut i = Self::new_node(true);
        let mut j = Self::new_node(false);
        let mut k = Self::new_node(false);
        let mut l = Self::new_node(true);
        let mut m = Self::new_node(true);
        a.insert_child(&mut b, 0);
        a.insert_child(&mut j, 1);
        b.insert_child(&mut d, 0);
        j.insert_child(&mut k, 0);
        d.insert_child(&mut e, 0);
        d.insert_child(&mut f, 1);
        k.insert_child(&mut l, 0);
        k.insert_child(&mut m, 1);
        f.insert_child(&mut g, 0);
        f.insert_child(&mut i, 0);
        g.insert_child(&mut h, 0);
        let mut names = HashMap::new();
        names.insert(a.inner_node, "a".to_string());
        names.insert(b.inner_node, "b".to_string());
        names.insert(d.inner_node, "d".to_string());
        names.insert(e.inner_node, "e".to_string());
        names.insert(f.inner_node, "f".to_string());
        names.insert(g.inner_node, "g".to_string());
        names.insert(h.inner_node, "h".to_string());
        names.insert(i.inner_node, "i".to_string());
        names.insert(j.inner_node, "j".to_string());
        names.insert(k.inner_node, "k".to_string());
        names.insert(l.inner_node, "l".to_string());
        names.insert(m.inner_node, "m".to_string());
        Self {
            names,
            a,
            b,
            d,
            e,
            f,
            g,
            h,
            i,
            j,
            k,
            l,
            m,
        }
    }

    pub fn layout(&mut self) {
        self.a.calculate_layout(100.0, 1000.0, LTR);
    }

    pub fn update_h(&mut self) {
        self.h.set_height(StyleUnit::Point(OrderedFloat(300.0)));
    }

    fn format_node(node: NodeRef, result: &mut String, names: &HashMap<NodeRef, String>) {
        let x = unsafe { YGNodeLayoutGetLeft(node) };
        let y = unsafe { YGNodeLayoutGetTop(node) };
        let width = unsafe { YGNodeLayoutGetWidth(node) };
        let height = unsafe { YGNodeLayoutGetHeight(node) };
        let str = format!("<div style='border: solid 1px #000; position: absolute; left: {}px; top: {}px; width: {}px; height: {}px'>\n", x, y, width, height);
        result.push_str(&str);
        let child_count = unsafe { YGNodeGetChildCount(node) };
        for i in 0..child_count {
            let child = unsafe { YGNodeGetChild(node, i) };
            Self::format_node(child, result, names);
        }
        let name = if child_count <= 0 {
            names.get(&node).unwrap().to_string()
        } else {
            String::from("")
        };
        result.push_str(&format!("{}</div>\n", name));
    }

    fn print_code(&mut self) -> String {
        let mut str = String::new();
        Self::format_node(self.a.inner_node, &mut str, &self.names);
        str
    }

    pub fn apply_update(&mut self) {
        Self::do_apply_updates(self.a.inner_node, &self.names);
    }

    fn do_apply_updates(node: YGNodeRef, names: &HashMap<NodeRef, String>) {
        if !Node::has_new_layout(node) {
            return;
        }
        unsafe {
            YGNodeSetHasNewLayout(node, false);
        }
        let child_count = unsafe { YGNodeGetChildCount(node) };
        for i in 0..child_count {
            let child = unsafe { YGNodeGetChild(node, i) };
            Self::do_apply_updates(child, names);
        }
        let name = names.get(&node).unwrap();
        println!("updating {}", name);
    }
}

pub fn main() {
    let mut tree = Tree::new();
    tree.layout();
    tree.apply_update();
    let code1 = tree.print_code();
    println!("after update h");
    tree.update_h();
    tree.layout();
    tree.apply_update();
    let code2 = tree.print_code();
    println!("<div style='display:flex;'><div style='position:relative;width: 50%'>{}</div><div style='position:relative'>{}</div></div>", code1, code2);
}

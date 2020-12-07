use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};

#[derive(Debug, Clone)]
struct BagNode<'a> {
    id: &'a str,
    children: Vec<(usize, &'a str)>
}

fn trim_bag_id(fancy_bag: &str) -> &str {
    fancy_bag.trim_end_matches(".").trim_end_matches("s").trim_end_matches(" bag")
}

fn parse_line(text: &str) -> Result<BagNode, Box<dyn Error>> {
    let parts = text.split(" bags contain ").collect::<Vec<_>>();
    let id = trim_bag_id(parts.get(0).ok_or("Invalid bag rule text.")?);
    let rest = parts.get(1).ok_or("Invalid bag rule text.")?;
    if rest == &"no other bags." {
        Ok(BagNode { id, children: Vec::new() })
    } else {
        let mut children = Vec::new();
        for child in rest.split(", ") {
            let (n, child_id) = trim_bag_id(child).split_at(1);
            children.push((n.parse()?, child_id.trim()));
        }
        Ok(BagNode { id, children })
    }
}

fn count_children(nodes: &HashMap<&str, BagNode>, root: &str) -> usize {
    let mut count = 0;
    let node = nodes.get(root).expect("Invalid ID given.");
    for (child_n, child_id) in node.children.iter() {
        count += child_n + child_n * count_children(nodes, child_id);
    }
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let lines = input.trim().split("\n").collect::<Vec<_>>();
    let mut nodes = HashMap::new();
    for line in lines {
        let node = parse_line(line)?;
        println!("Parsed node: {:?}", node);
        nodes.insert(node.id, node);
    }
    println!("Result: {:?}", count_children(&nodes, "shiny gold"));
    Ok(())
}

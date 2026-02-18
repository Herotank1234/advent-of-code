use std::fs;
use std::collections::HashMap;
use rand::random_range;

#[derive(Clone, Debug)]
struct SuperNode {
  name: String,
  nodes: Vec<String>
}

#[derive(Clone, Debug)]
struct SuperEdge {
  v1: String,
  v2: String
}

fn karger(mut nodes: HashMap<String, SuperNode>, mut edges: Vec<SuperEdge>) -> (HashMap<String, SuperNode>, Vec<SuperEdge>) {
  while nodes.len() > 2 {
    /* 
      Generate random index 0-edges.len
      Store edge in variable
      Combine supernodes
      Change all edges that point to old supernodes to new supernodes
    */
    let index = random_range(..edges.len());
    let removed_edge = edges[index].clone();
    let mut indexes = Vec::new();
    for i in 0..edges.len() {
      if edges[i].v1 == removed_edge.v1 && edges[i].v2 == removed_edge.v2 ||
        edges[i].v1 == removed_edge.v2 && edges[i].v2 == removed_edge.v1
      {
        indexes.push(i);
      }
    }

    for index in indexes.iter().rev() {
      edges.remove(*index);
    }

    let removed_node_1 = nodes.get(&removed_edge.v1).unwrap().clone();
    let removed_node_2 = nodes.get(&removed_edge.v2).unwrap().clone();
    nodes.remove(&removed_edge.v1);
    nodes.remove(&removed_edge.v2);

    let new_name = format!("{}{}", removed_node_1.name.clone(), removed_node_2.name.clone());
    let mut new_nodes = removed_node_1.nodes.clone();
    new_nodes.extend(removed_node_2.nodes);

    nodes.insert(new_name.clone(), SuperNode{name: new_name.clone(), nodes: new_nodes});

    for edge in edges.iter_mut() {
      if edge.v1 == removed_edge.v1 {
        edge.v1 = new_name.clone();
      }
      if edge.v2 == removed_edge.v1 {
        edge.v2 = new_name.clone();
      }
      if edge.v1 == removed_edge.v2 {
        edge.v1 = new_name.clone();
      }
      if edge.v2 == removed_edge.v2 {
        edge.v2 = new_name.clone();
      }
    }
  }

  return (nodes, edges);
}

fn initialise_karger(adj_list: &HashMap<String, Vec<String>>) -> (HashMap<String, SuperNode>, Vec<SuperEdge>) {
  let mut supernodes = HashMap::new();
  let mut superedges = Vec::new();

  for(src, dsts) in adj_list.iter() {
    supernodes.insert(src.clone(), SuperNode{name: src.clone(), nodes: vec![src.clone()]});
    for dst in dsts.iter() {
      superedges.push(SuperEdge{v1: src.clone(), v2: dst.clone()});
    }
  }

  return (supernodes, superedges);
}

fn multiply_group_sizes(adj_list: &HashMap<String, Vec<String>>) -> u32 {
  let (supernodes, superedges) = initialise_karger(adj_list);
  let (mut res_nodes, mut res_edges) = karger(supernodes.clone(), superedges.clone());
  while res_edges.len() > 3 {
    (res_nodes, res_edges) = karger(supernodes.clone(),superedges.clone());
  }
  return res_nodes.values().fold(1, |acc, s| acc * s.nodes.len() as u32);
}

fn main() {
  let contents = fs::read_to_string("./data/q25.txt")
    .expect("Should have been able to read file");
  let connections = contents.split('\n').map(|line| {
    let parts = line.split(':').collect::<Vec<&str>>();
    let connections = parts[1].trim().split(' ').map(|c| c.to_string()).collect::<Vec<String>>();
    return (parts[0].to_string(), connections);
  }).collect::<Vec<(String, Vec<String>)>>();

  let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();

  for (src, dsts) in connections.iter() {
    for dst in dsts.iter() {
      if adj_list.contains_key(src) {
        adj_list.get_mut(src).unwrap().push(dst.clone());
      } else {
        adj_list.insert(src.clone(), vec![dst.clone()]);
      }
      if !adj_list.contains_key(dst) {
        adj_list.insert(dst.clone(), vec![]);
      }
    }
  }

  println!("Part 1: {}", multiply_group_sizes(&adj_list));
}
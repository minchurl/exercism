fn find_edge(node_name: u8, edges: &mut Vec<(u8, u8)>) -> Option<usize> {
    edges.iter().position(|&(x, y)| {
        node_name == x || node_name == y
    })
}

fn remove_edge(edges: &mut Vec<(u8, u8)>, idx: usize) {
    let l = edges.len();
    edges.swap(idx, l - 1);
    edges.pop();
}

fn dfs(node_name: u8, edges: &mut Vec<(u8, u8)>, res: &mut Vec<(u8, u8)>) {
    while let Some(idx) = find_edge(node_name, edges) {
        let edge = 
            if edges[idx].0 == node_name {
                (edges[idx].1, edges[idx].0)
            }
            else {
                (edges[idx].0, edges[idx].1)
            }
        ;
        remove_edge(edges, idx);
        dfs(edge.0, edges, res);
        res.push(edge);
    }
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.len() == 0 {
        return Some(Vec::new())
    }
    let mut edges: Vec<(u8, u8)> = input.to_vec();
    let mut res: Vec<(u8, u8)> = Vec::new();

    dfs(input[0].0, &mut edges, &mut res);

    let l = input.len();

    if res.len() != l || res[0].0 != res[l - 1].1 {
        return None
    }
    Some(res)
}

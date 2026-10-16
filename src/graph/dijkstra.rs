use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(dead_code)]
pub fn dijkstra(sv: usize, graph: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![usize::MAX; n];
    dist[sv] = 0;

    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), sv));

    while let Some((Reverse(cost), v)) = pq.pop() {
        if dist[v] < cost {
            continue;
        }

        for &(nv, w) in &graph[v] {
            let ncost = cost + w;
            if ncost < dist[nv] {
                dist[nv] = ncost;
                pq.push((Reverse(ncost), nv));
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use crate::graph::dijkstra::dijkstra;
    use itertools::Itertools;

    #[test]
    fn tessoku_A64() {
        let n = 6;
        let abc = vec![(1, 2, 15), (1, 4, 20), (2, 3, 65), (2, 5, 4), (3, 6, 50), (4, 5, 30), (5, 6, 8)];

        let mut graph = vec![vec![]; n];
        for (a, b, c) in abc {
            graph[a - 1].push((b - 1, c));
            graph[b - 1].push((a - 1, c));
        }

        let dist = dijkstra(0, &graph);
        let res = dist.iter().map(|&u| if u != usize::MAX { u as isize } else { -1 }).join("\n");
        assert_eq!(res, format!("0\n15\n77\n20\n19\n27"));
    }
}

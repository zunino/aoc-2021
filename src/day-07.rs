#[derive(Debug, PartialEq, Clone)]
pub struct Cluster(u16, u16);

impl Cluster {
    fn inc(&mut self) {
        self.1 += 1;
    }
}

fn parse_input(input: &str) -> Vec<u16> {
    input
        .split(",")
        .map(|v| v.trim().parse::<u16>().unwrap())
        .collect()
}

fn clusterize(data: &Vec<u16>) -> Vec<Cluster> {
    let mut sorted = data.clone();
    sorted.sort();
    let mut clusters = Vec::<Cluster>::new();
    if sorted.is_empty() {
        return clusters;
    }

    let mut last_v = u16::MAX;
    for curr_v in sorted {
        if curr_v != last_v {
            let cluster = Cluster(curr_v, 0);
            clusters.push(cluster.clone());
            last_v = curr_v;
        }
        let last_index = clusters.len() - 1;
        clusters[last_index].inc();
    }

    clusters
}

type CostFunction = fn(clusters: &[Cluster], pos: u16) -> u32;

pub fn find_optimal_pos(
    clusters: &[Cluster],
    base_idx: usize,
    cost_fn: CostFunction,
) -> (u16, u32) {
    let cost_c = cost_fn(clusters, clusters[base_idx].0);
    let cost_l = cost_fn(clusters, clusters[base_idx - 1].0);
    let cost_r = cost_fn(clusters, clusters[base_idx + 1].0);
    if cost_l < cost_c && cost_l < cost_r {
        return find_optimal_pos(clusters, base_idx - 1, cost_fn);
    }
    if cost_r < cost_c {
        return find_optimal_pos(clusters, base_idx + 1, cost_fn);
    }
    (clusters[base_idx].0, cost_c)
}

mod part_1 {
    use super::Cluster;

    pub fn cost_function(clusters: &[Cluster], base: u16) -> u32 {
        let base = base as u32;
        let mut cost = 0u32;
        for c in clusters {
            let cluster_value = (*c).0 as u32;
            let cluster_count = (*c).1 as u32;
            cost += cluster_count
                * if base > cluster_value {
                    base - cluster_value
                } else {
                    cluster_value - base
                };
        }
        cost
    }
}

mod part_2 {
    use super::Cluster;

    fn cummulative_cost(n: u32) -> u32 {
        if n < 2 {
            n
        } else {
            n + cummulative_cost(n - 1)
        }
    }

    pub fn cost_function(clusters: &[Cluster], base: u16) -> u32 {
        let base = base as u32;
        let mut cost = 0u32;
        for c in clusters {
            let cluster_value = (*c).0 as u32;
            let cluster_count = (*c).1 as u32;
            cost += cluster_count
                * if base > cluster_value {
                    cummulative_cost(base - cluster_value)
                } else {
                    cummulative_cost(cluster_value - base)
                };
        }
        cost
    }
}

fn main() {
    let data = parse_input(&std::fs::read_to_string("data/day-07.txt").unwrap());
    let clusters = clusterize(&data);

    println!("== PART 1");
    let (pos, fuel) = find_optimal_pos(&clusters, clusters.len() / 2, part_1::cost_function);
    println!("Optimal position: {pos}");
    println!("Fuel consumption: {fuel}");

    println!("== PART 2");
    let (pos, fuel) = find_optimal_pos(&clusters, clusters.len() / 2, part_2::cost_function);
    println!("Optimal position: {pos}");
    println!("Fuel consumption: {fuel}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_should_be_grouped_in_clusters() {
        let values: Vec<u16> = [8, 5, 2, 5, 1, 8, 9].to_vec();
        let clusters = clusterize(&values);
        assert_eq!(clusters.len(), 5);
        assert_eq!(clusters[0], Cluster(1, 1));
        assert_eq!(clusters[1], Cluster(2, 1));
        assert_eq!(clusters[2], Cluster(5, 2));
        assert_eq!(clusters[3], Cluster(8, 2));
        assert_eq!(clusters[4], Cluster(9, 1));
    }

    mod part_1 {
        use super::*;
        use crate::part_1::*;

        #[test]
        fn cost_function_with_different_bases() {
            let clusters = clusterize(&vec![1, 2, 5, 5, 8, 8, 9, 10]);
            assert_eq!(cost_function(&clusters, 1), 40);
            assert_eq!(cost_function(&clusters, 2), 34);
            assert_eq!(cost_function(&clusters, 5), 22);
            assert_eq!(cost_function(&clusters, 8), 22);
            assert_eq!(cost_function(&clusters, 9), 26);
            assert_eq!(cost_function(&clusters, 10), 32);
        }

        #[test]
        fn cost_function_with_different_bases_example_sequence() {
            let clusters = clusterize(&vec![0, 1, 1, 2, 2, 2, 4, 7, 14, 16]);
            assert_eq!(cost_function(&clusters, 2), 37);
        }

        #[test]
        fn test_optimal_pos_calculation() {
            let clusters = clusterize(&vec![0, 1, 1, 2, 2, 2, 4, 7, 14, 16]);
            let (pos, fuel) = find_optimal_pos(&clusters, clusters.len() / 2, cost_function);
            assert_eq!(pos, 2);
            assert_eq!(fuel, 37);
        }
    }

    mod part_2 {
        use super::*;
        use crate::part_2::*;

        #[test]
        fn cost_function_with_different_bases() {
            let clusters = clusterize(&vec![1, 2, 5, 5, 8, 8, 9, 10]);
            assert_eq!(cost_function(&clusters, 1), 158);
            assert_eq!(cost_function(&clusters, 2), 119);
            assert_eq!(cost_function(&clusters, 5), 53);
            assert_eq!(cost_function(&clusters, 8), 65);
            assert_eq!(cost_function(&clusters, 9), 87);
            assert_eq!(cost_function(&clusters, 10), 118);
        }
    }
}

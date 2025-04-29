
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};
    use crate::projectutils::read_higgs_dataset;
    use crate::{most_tweets, find_layers, average_interactions_per_layer, find_first_timestamp};
    #[test]
    pub fn test_first_timestamp() {
        // test the first timestamp function
        let higgs_dataset = read_higgs_dataset("higgs-activity_time.txt.gz")
        .expect("Failed to load Higgs dataset");
        let Some((_, _, timestamp, _)) = find_first_timestamp(&higgs_dataset) else {
            panic!("Didn't work");
        };
    
        assert_eq!(timestamp, "2012-07-01 00:02:52.000000000", "This is not the first timestamp");
    }
    #[test]
    pub fn test_most_tweets(){
        // Make sure the most interactions is 656
        let higgs_dataset = read_higgs_dataset("higgs-activity_time.txt.gz")
        .expect("Failed to load Higgs dataset");
        let (user, interactions) = most_tweets(&higgs_dataset);
        //println!("User {} has the most interactions with {}", user, interactions);
        assert!(interactions == 656, "Interactions is not 656");
        assert_eq!(user, 89805, "Top user ID is not 89805");
    }
    #[test]
    pub fn test_average_interactions(){
        let higgs_dataset = read_higgs_dataset("higgs-activity_time.txt.gz")
            .expect("Failed to load Higgs dataset");
        let map_of_layers = find_layers(&higgs_dataset);
        let interactions_per_layer = average_interactions_per_layer(&higgs_dataset, map_of_layers);

        let expected: HashMap<usize, usize> = HashMap::from([ // from chatgpt for simplicity and conciseness
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 3),
            (4, 2),
            (5, 3),
            (6, 0),
        ]);
        assert_eq!(interactions_per_layer, expected, "The averages didn't work out");
    }
    #[test]
    pub fn test_number_of_nodes(){
        // the internet says there should be 304691 nodes to my specific file
        // https://networks.skewed.de/net/twitter_higgs
        let higgs_dataset = read_higgs_dataset("higgs-activity_time.txt.gz")
            .expect("Failed to load Higgs dataset");
            let mut nodes: HashSet<usize> = HashSet::new();

        for (source, edges) in &higgs_dataset {
            nodes.insert(*source); // include source node
            for (target, _timestamp, _interaction) in edges {
                nodes.insert(*target); // include target part of the node
            }
        }
        assert_eq!(nodes.len(), 304691, "Not 304691 nodes");
    }
}
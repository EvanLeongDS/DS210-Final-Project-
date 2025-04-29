// Evan Leong DS210 Project Higgs Boson dataset analysis 
use std::fs::File;
use std::collections::HashMap;
use std::collections::VecDeque;
mod projectutils;

fn find_first_timestamp(dataset: &HashMap<usize, Vec<(usize, usize, String)>>) -> Option<(usize, usize, String, String)>{
    // See who tweeted about higgs first in the dataset
    let mut lowest_timestamp = usize::MAX; // start big work our way down
    let mut result: Option<(usize, usize, String, String)> = None; // to store the best result

    for (source, vector) in dataset.iter(){
        for (target, unix_timestamp, interaction) in vector {
            if *unix_timestamp < lowest_timestamp {
                lowest_timestamp = *unix_timestamp;
                let normalized_time = projectutils::convert_timestamp(*unix_timestamp as u64);
                result = Some((*source, *target, (*normalized_time).to_string(), interaction.clone()));
            }
        }
    }
    return result;
}

fn find_last_timestamp(dataset: &HashMap<usize, Vec<(usize, usize, String)>>) -> Option<(usize, usize, String, String)>{
    // See who tweeted about higgs last in the dataset 
    let mut highest_timestamp: usize = 0; // start small work our way up 
    let mut result: Option<(usize, usize, String, String)> = None; // to store the best result

    for (source, vector) in dataset.iter(){
        for (target, unix_timestamp, interaction) in vector {
            if *unix_timestamp > highest_timestamp {
                highest_timestamp = *unix_timestamp;
                let normalized_time = projectutils::convert_timestamp(*unix_timestamp as u64);
                result = Some((*source, *target, (*normalized_time).to_string(), interaction.clone()));
            }
        }
    }
    return result;
}

fn higgs_bfs(dataset: &HashMap<usize, Vec<(usize, usize, String)>>, start: usize) -> (usize, u32) {
    // find how many levels there are from the first user who sent out a tweet (223789)

    // from the lecture notes
    let mut distance: HashMap<usize, u32> = HashMap::new(); //since my dataset is so big it is better to have a hashmap
    let mut queue: VecDeque<usize> = VecDeque::new();

    distance.insert(start, 0);
    queue.push_back(start);

    // perform bfs
    while let Some(v) = queue.pop_front() { // new unprocessed vertex
        if let Some(targets) = dataset.get(&v) { // consider all unprocessed neighbors of v
            for (target, _timestamp, _interaction) in targets {
                if !distance.contains_key(target) {
                distance.insert(*target, distance[&v] + 1); // if there is another layer add 1 to the distance
                queue.push_back(*target);
                }
            }
        }
    };

    let mut max_distance: u32 = 0; 
    let mut max_user = 0;
    for (key, value) in distance.iter() {
        if *value > max_distance{
            max_distance = *value;
            max_user = *key;
        }
    }
    return (max_user, max_distance);
}

fn most_tweets(dataset: &HashMap<usize, Vec<(usize, usize, String)>>) -> (usize, u32) {
    // Figure out who tweeted the most about the higgs boson 

    let mut tweet_map: HashMap<usize, u32> = HashMap::new(); // user, # of edges
    for (source, target) in dataset.iter() {
        tweet_map.insert(*source, target.len().try_into().unwrap());
    }

    let mut max_value: u32 = 0;
    let mut max_key: usize = 0;
    for (key, value) in tweet_map.iter() {
        if *value > max_value {
            max_key = *key;
            max_value = *value;
        }
    }
    return (max_key, max_value);
}

fn find_layers(dataset: &HashMap<usize, Vec<(usize, usize, String)>>) -> HashMap<usize, usize> {
    // find what layer each user is at in the graph

    let mut layer_map: HashMap<usize, usize> = HashMap::new();
    let mut queue: VecDeque<usize> = VecDeque::new(); // for bfs

    // this portion from chatgpt
    let mut receivers = std::collections::HashSet::new();
    for vec in dataset.values() {
        for (target, _timestamp, _interaction) in vec {
            receivers.insert(*target);
        }
    }
    // figure out the origin points in the graph 
    for node in dataset.keys() {
        if !receivers.contains(node) { // if its not in there add it 
            layer_map.insert(*node, 0); // start at distance work our way up 
            queue.push_back(*node); 
        } 
    }
    // find the undirected edges with bfs and get layers from them 
    while let Some(v) = queue.pop_front() { // new unprocessed vertex
        if let Some(targets) = dataset.get(&v) {
            for (target, _timestamp, _interaction) in targets {
                if !layer_map.contains_key(target) {
                    layer_map.insert(*target, layer_map[&v] + 1);
                    queue.push_back(*target);
                }
            }
        }
    }
    return layer_map;
}

fn average_interactions_per_layer(dataset: &HashMap<usize, Vec<(usize, usize, String)>>, layer_map: HashMap<usize, usize>) -> HashMap<usize, usize> {
    // Find how many outgoing edges there are for each layer 

    let mut interactions_per_layer: HashMap<usize, usize> = HashMap::new();
    let mut users_in_layer: HashMap<usize, Vec<usize>> = HashMap::new();

    for (user, layer) in layer_map {
        users_in_layer.entry(layer).or_insert_with(Vec::new).push(user); // push in the vector of users
    }
    // go through each layer like layer 0 layer 1 layer 2 
    for (layer, users) in users_in_layer {
        // for each user count up the edges
        let mut total_interactions = 0;
        for user in &users {
            if let Some(interaction_count) = dataset.get(&user) {
            total_interactions += interaction_count.len();
            } else {
                // well nothing is gonna happen
            }
        }
        // find the average number of edges in each layer count and return it in a hashmap 
        let average: usize = total_interactions / users.len();
        interactions_per_layer.insert(layer, average);
        
    }
    return interactions_per_layer;
}

fn main() {
    // read and test my functions 

    let higgs_dataset = projectutils::read_higgs_dataset("/Users/evanl/OneDrive/Desktop/DS210Project/higgs-activity_time.txt.gz")
        .expect("Failed to load Higgs dataset");

    projectutils::print_timestamp(
        "For who found out first",
        find_first_timestamp(&higgs_dataset)
    );
    
    //projectutils::print_timestamp(
      //  "For who found out last",
        //find_last_timestamp(&higgs_dataset)
   // );

    let (node, distance) = higgs_bfs(&higgs_dataset, 223789);
    //println!("The farthest node is {} which has distance {:?}", node, distance);

    let (user, interactions) = most_tweets(&higgs_dataset);
    //println!("User {} has the most interactions with {}", user, interactions);

    let top_ten = projectutils::top_ten_influencers(&higgs_dataset);
    //println!("Top ten: {:?}", top_ten);

    let map_of_layers = find_layers(&higgs_dataset);
    let layer_visual = projectutils::layer_map_count(map_of_layers.clone());

    let interactions_per_layer = average_interactions_per_layer(&higgs_dataset, map_of_layers);
    println!("Interactions per layer: {:?}", interactions_per_layer);
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn test_first_timestamp() {
        // test the first timestamp function
        let higgs_dataset = projectutils::read_higgs_dataset("/Users/evanl/OneDrive/Desktop/DS210Project/higgs-activity_time.txt.gz")
        .expect("Failed to load Higgs dataset");
        let Some((_, _, timestamp, _)) = find_first_timestamp(&higgs_dataset) else {
            panic!("Didn't work");
        };
    
        assert_eq!(timestamp, "2012-07-01 00:02:52.000000000", "This is not the first timestamp");
    }
    #[test]
    fn test_most_tweets(){
        // Make sure the most interactions is 656
        let higgs_dataset = projectutils::read_higgs_dataset("/Users/evanl/OneDrive/Desktop/DS210Project/higgs-activity_time.txt.gz")
        .expect("Failed to load Higgs dataset");
        let (user, interactions) = most_tweets(&higgs_dataset);
        //println!("User {} has the most interactions with {}", user, interactions);
        assert!(interactions == 656, "Interactions is not 656");
        assert_eq!(user, 89805, "Top user ID is not 89805");
    }
    #[test]
    fn test_average_interactions(){
        let higgs_dataset = projectutils::read_higgs_dataset("/Users/evanl/OneDrive/Desktop/DS210Project/higgs-activity_time.txt.gz")
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
}
use std::fs::File;
use std::io::BufRead;
use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::collections::VecDeque;
extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{UNIX_EPOCH, Duration};

pub fn read_higgs_dataset(path: &str) -> Result<HashMap<usize, Vec<(usize, usize, String)>>, Box<dyn std::error::Error>> {
    // Read the txt file from the Higgs Boson dataset

    let file = File::open(path).expect("Could not open file");
    let decoder = GzDecoder::new(file); // gz file so this line is needed
    let buf_reader = std::io::BufReader::new(decoder).lines();

    let mut graph: HashMap<usize, Vec<(usize, usize, String)>> = HashMap::new(); // for returning later
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let parts: Vec<&str> = line_str
            .trim() 
            .split(' ') 
            .collect(); 

        let source: usize = parts[0].parse().expect("Error parsing source");
        let target: usize = parts[1].parse().expect("Error parsing target");
        let unix_timestamp: usize = parts[2].parse().expect("Error parsing timestamp"); 
        let interaction_type = parts[3]; // MT stand for mention, RT stands for retweet, RE stands for replying

        graph.entry(source) // from chatgpt
            .or_insert_with(Vec::new)
            .push((target, unix_timestamp, interaction_type.to_string()));
    }
    Ok(graph)
}

pub fn convert_timestamp(unix_time: u64) -> String {
    // Helper function 
    // I found out the timestamp is in unix so I want to convert it to a more 

    // https://stackoverflow.com/questions/50072055/converting-unix-timestamp-to-readable-time-string-in-rust
    let d = UNIX_EPOCH + Duration::from_secs(unix_time);
    let datetime = DateTime::<Utc>::from(d);

    let standard_timestamp = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    return standard_timestamp;
}

pub fn print_timestamp(phrase: &str, result: Option<(usize, usize, String, String)>) -> (){
    // helper function for printing the timestamps first and last!
    if let Some((sender, receiver, standard_time, interaction)) = result {
      println!("{}\n The sender is {}\nThe receiver is {}\nThe timestamp is {}\nThe type is {}",phrase, sender, receiver, standard_time, interaction);
    } else {
        println!("No timestamp found");
    }
}

pub fn top_ten_influencers(dataset: &HashMap<usize, Vec<(usize, usize, String)>>) -> Vec<(usize, u32)> {
    // helper function for most tweets to see the top ten most influential ppl

    let mut tweet_map: HashMap<usize, u32> = HashMap::new();
    for (source, target) in dataset.iter() {
        tweet_map.insert(*source, target.len().try_into().unwrap());
    }

    // from hw6
    let mut sorted_popularity: Vec<(&usize, &u32)> = tweet_map.iter().collect();
    sorted_popularity.sort_by(|a, b| {
        b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)) // tie breaker
    }); // https://stackoverflow.com/questions/34555837/sort-hashmap-data-by-value

    let mut result: Vec<(usize, u32)> = Vec::new();
    for (user, count) in sorted_popularity.iter().take(10) { 
        result.push((**user,**count));
    }
    return result;
}

pub fn layer_map_count(layer_map: HashMap<usize, usize>) -> HashMap<usize, usize> {
    // helper function for visualizing the layers in my layer map
    let mut layer_count: HashMap<usize, usize> = HashMap::new();
    for (key, value) in layer_map {
        *layer_count.entry(value).or_insert(0)+=1;
    }
    println!("Layer map: {:?}", layer_count);
    return layer_count;
}
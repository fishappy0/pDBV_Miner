use bitvec::prelude::*;
use std::collections::HashMap;

use crate::types::Node;

/*
    function: create_dataset_from_dataset
    description: creates a dataset from a vector of strings consisting of a transaction id and a list of items
    input: a vector of strings
    output: a nested vector of strings with the following structure:
        [
            [[transaction_id], [item1, item2, item3, ...]],
            [[transaction_id], [item1, item2, item3, ...]],
            [[transaction_id], [item1, item2, item3, ...]],
            ...
        ]
*/
pub fn create_dataset_from_dataset(raw_data: &Vec<String>) -> Vec<Vec<Vec<String>>> {
    let mut dataset = Vec::new();
    (0..raw_data.len()).for_each(|i| {
        let mut item = Vec::new();
        let mut tid = Vec::new();
        let mut raw_item = raw_data[i].split_whitespace();
        tid.push(
            raw_item
                .next()
                .unwrap()
                .to_string()
                .split(":")
                .collect::<Vec<&str>>()[0]
                .to_string(),
        );
        raw_item.for_each(|x| {
            if !item.contains(&x.to_string()) {
                item.push(x.to_string())
            }
        });
        dataset.push(vec![tid, item]);
    });
    dataset
}
/*
    function: create_root_nodes
    description: creates and filters a root node list from a dataset with a minimum support
    input: a dataset and a minimum support
    output: a hashmap with the following structure:
        {
            "item1": [support],
            "item2": [support],
            "item3": [support],
            ...
        }
*/
pub fn create_root_nodes(
    dataset: &Vec<Vec<Vec<String>>>,
    min_sup: f32,
) -> HashMap<&String, Vec<i32>> {
    let mut root_node_list: HashMap<&String, Vec<String>> = HashMap::new();
    // create the root node hashmap
    dataset.iter().for_each(|x| {
        x[1].iter().for_each(|y| {
            if root_node_list.contains_key(y) {
                root_node_list.get_mut(y).unwrap().push(x[0][0].to_string());
            } else {
                root_node_list.insert(y, vec![x[0][0].to_string()]);
            }
        });
    });
    // filter by minimum support
    root_node_list
        .iter()
        .filter(|(_, v)| v.len() >= ((v.len() as f32) * min_sup).round() as usize)
        .map(|(k, v)| {
            let mut vec = Vec::new();
            v.iter().for_each(|v| vec.push(v.parse::<i32>().unwrap()));
            vec.sort();
            (*k, vec)
        })
        .collect()
}

pub fn convert_to_dbvec(
    itemlist: HashMap<&String, Vec<i32>>,
) -> HashMap<String, HashMap<i32, BitVec>> {
    let mut dbvec: HashMap<String, HashMap<i32, BitVec>> = HashMap::new();
    itemlist.iter().for_each(|(k, sorted_vec)| {
        if sorted_vec.iter().next().unwrap() > sorted_vec.iter().last().unwrap() {
            panic!("Error: convert_to_dbvec: array is not sorted");
        }
        let support_len = sorted_vec.last().unwrap();
        // let mut bvec_container: Vec<BitVec> = Vec::new();
        let mut bvec = BitVec::new();
        let mut dbvec_temp: HashMap<i32, BitVec> = HashMap::new();

        let mut count = 0;
        let mut array_order = 0;

        let mut queue = sorted_vec.clone();
        queue.reverse();
        let mut temp = queue.pop().unwrap();
        let start = *sorted_vec.iter().next().unwrap();
        let end = *sorted_vec.iter().last().unwrap();
        (start..(start + end)).for_each(|x| {
            if count > 127 {
                count = 0;
                array_order += 1;
            } else {
                if x == temp as i32 {
                    bvec.push(true);
                    temp = queue.pop().unwrap_or_else(|| -99);
                } else if temp != -99 {
                    bvec.push(false);
                }
                count += 1;
            }
        });

        dbvec_temp.insert(sorted_vec[0], bvec);
        dbvec.insert(k.to_string(), dbvec_temp);
    });
    dbvec
}
pub fn get_frequent_itemsets(
    node_param: Node<HashMap<String, HashMap<i32, BitVec>>>,
    min_sup: usize,
) {
    let mut node_list: Vec<Node<HashMap<String, HashMap<i32, BitVec>>>> = node_param.children;
    for f_node in node_list {
        for s_node in node_list {
            let merged_node = merge_nodes(f_node, s_node);
            if merged_node
                .item
                .iter()
                .next()
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .1
                .iter()
                .next()
                .unwrap()
                .1
                .len()
                >= min_sup
            {
                node_param.children.push(merged_node);
            }
        }
    }
}
fn merge_nodes(
    dbv_2: Node<HashMap<String, HashMap<i32, BitVec>>>,
    dbv_1: Node<HashMap<String, HashMap<i32, BitVec>>>,
) -> Node<HashMap<String, HashMap<i32, BitVec>>> {
    // Reading stuff
    let dbv_1_content = dbv_1.item.unwrap().into_iter().next().unwrap();
    let dbv_2_content = dbv_2.item.unwrap().into_iter().next().unwrap();
    let node_name = dbv_1_content.0.to_string()
        + ":"
        + dbv_2_content
            .0
            .split(":")
            .collect::<Vec<&str>>()
            .last()
            .unwrap();

    let dbv_1_start = dbv_1_content.1.into_iter().next().unwrap().0;
    let dbv_1_bitvec = dbv_1_content.1.into_iter().next().unwrap().1;

    let dbv_2_start = dbv_2_content.1.into_iter().next().unwrap().0;
    let dbv_2_bitvec = dbv_2_content.1.into_iter().next().unwrap().1;

    // Assigning stuff and checking stuff
    let mut i = 0;
    let mut j = 0;
    if (dbv_1_start < dbv_2_start) {
        i = dbv_2_start - dbv_1_start;
        j = 0;

        // Terminate Early on the case of no overlap
        // Example:
        //  start_1 = 40, dbv_1 = [1,1,0,0]
        //  start_2 = 50, dbv_2 = [1,1,0,0]
        if dbv_1_start + (dbv_1_bitvec.len() as i32) < dbv_2_start {
            let return_item = HashMap::new();
            let return_dbv = HashMap::new();
            return_dbv.insert(0, BitVec::new());
            return_item.insert(node_name, return_dbv);
            return Node {
                item: Some(return_item.clone()),
                children: vec![],
            };
        }
    } else {
        i = 0;
        j = dbv_1_start - dbv_2_start;

        // Another case of no overlap
        if dbv_2_start + (dbv_2_bitvec.len() as i32) < dbv_1_start {
            let return_item = HashMap::new();
            let return_dbv = HashMap::new();
            return_dbv.insert(0, BitVec::new());
            return_item.insert(node_name, return_dbv);
            return Node {
                item: Some(return_item.clone()),
                children: vec![],
            };
        }
    };
}

//     let mut f_len = 0;
//     let mut s_len = 0;
//     // extract the i16 from the second inner hashmap
//     dbv_2.iter().for_each(|(_, v)| {
//         v.iter().for_each(|(k, _)| {
//             s_len = *k;
//         });
//     });
//     dbv_1.iter().for_each(|(_, v)| {
//         v.iter().for_each(|(k, _)| {
//             f_len = *k;
//         });
//     });
//     let pos = std::cmp::max(f_len, s_len);
//     let mut i = 0;
//     let mut j = 0;
//     if f_len < s_len {
//         i = s_len - f_len;
//         j = 0;
//     } else {
//         i = 0;
//         j = f_len - s_len;
//     }
// }
// // fn get_frequent_itemsets(dbv: &HashMap<String, HashMap<i16, BitVec>>) -> HashMap<&String, Vec<HashMap<>>>> {
// //     let child_nodes_of_dbg =
// // }

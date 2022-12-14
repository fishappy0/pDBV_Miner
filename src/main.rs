use plotlib;
use plotlib::view::ContinuousView;
use plotlib::view::View;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert;
use std::fs;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

use crate::types::Node;
use crate::types::Tree;
use crate::utils::get_frequent_itemsets;
use bitvec::prelude::*;
mod types;
mod utils;
fn main() {
    let test_data = vec![
        String::from("10: AD D B CA"),
        String::from("2: A B CA D"),
        String::from("5: D CA AD "),
        String::from("4: AD B CA D"),
    ];

    // // parse csv
    // let csv_data = std::fs::read_to_string("resources/transactional_T10I4D100K.csv").unwrap();
    // let mut test_data = Vec::new();
    // let mut count = 1;
    // csv_data.lines().for_each(|line| {
    //     let stripped_line = line.replace(",", " ");
    //     let string = count.to_string() + ": " + &stripped_line;
    //     test_data.push(string);
    //     count += 1;
    // });

    // create the benchmark loop
    let mut time_vec = Vec::new();
    let mut support_vec = Vec::new();

    for i in 1..5 {
        println!("Run: {i}");
        let start = Instant::now();

        // Extracting TIDset and Itemset from tput data
        let dataset = utils::create_dataset_from_dataset(&test_data);

        // Creating the root node list and filter by minimum support
        let convert = utils::create_root_nodes(&dataset, 0.8);

        // Converting the traditional root node list to a Dynamic Bit Vector data type
        let dbv = utils::convert_to_dbvec(convert);

        // let mut tree = BTreeMap::new();

        // for node in dbv.iter() {
        //     tree.insert(node.0, node.1);
        // }
        // println!("{:#?}", tree);

        // building tree from scratch... lol
        let mut root_node = HashMap::new();
        let mut root_node_bitvec = HashMap::new();
        let mut placeholder_bitvec = BitVec::new();

        let mut iter_array = vec![];

        placeholder_bitvec.push(false);
        root_node_bitvec.insert(0 as i32, placeholder_bitvec);

        let placeholder_string = "root".to_string();
        root_node.insert(placeholder_string, root_node_bitvec);
        let mut custom_tree = Tree {
            root: Node {
                item: Some(root_node),
                children: vec![],
                has_iterated: false,
            },
            // closed_sets: std::collections::HashSet::new(),
        };

        dbv.iter().for_each(|node| {
            let current_node = &mut custom_tree.root;
            let mut node_item = HashMap::new();
            node_item.insert(node.0.to_owned(), node.1.to_owned());
            iter_array.push(node_item.clone());
            let new_node = Node {
                item: Some(node_item),
                children: vec![],
                has_iterated: false,
            };
            current_node.children.push(new_node);
        });
        print!("{:#?}", get_frequent_itemsets(&mut custom_tree.root, 100));
        let elapsed = start.elapsed();
        // custom_tree.root.children.borrow_mut().iter().for_each(|v| {
        //     println!("{v:#?}");
        // });
        // print!("{:#?}", custom_tree.root.children.borrow_mut());
        // print!("{:#?}", get_frequent_itemsets(, min_sup));

        // generate frequent itemsets

        // print!("{:#?}", custom_tree);
        // let frequent_itemsets: HashMap<&String, i64> = utils::get_frequent_itemsets(&dbv, 3);
        // let tree = rctree::Node::new(&dbv);

        // tree.traverse().for_each(|x| {
        //     println!("{:#?}", x);
        // });

        // for node in tree.traverse() {
        //     println!("node: {:#?}", node);
        // }
        // print!("{:#?}", tree);

        // let mut file = std::fs::File::create("output.txt").unwrap();
        // dbv.iter().for_each(|x| {
        //     file.write("---".as_bytes()).unwrap();
        //     let sequence_name = "Sequence: ".to_string() + &x.0.to_string() + " ";
        //     file.write(sequence_name.as_bytes()).unwrap();
        //     x.1.iter().for_each(|y| {
        //         let start = "Start: ".to_string() + &y.0.to_string() + " ";
        //         file.write(start.as_bytes()).unwrap();
        //         let bit = "Bit: ".to_string() + &y.1.to_string() + " ";
        //         file.write(bit.as_bytes()).unwrap();
        //     })
        // });
        // dbv.iter().for_each(|x| {
        //     println!("---");
        //     println!("Sequence: {}", x.0);
        //     x.1.iter().for_each(|y| {
        //         println!("start: {}", y.0);
        //         print!("itemset dbv is:");
        //         // let bit2 = y.1.get(1);
        //         // let bit = y.1.get(0).eq(&bit2);
        //         // println!("compared {:?} ", bit);

        //         y.1.iter().for_each(|z| {
        //             print!("{} ", z);
        //         });
        //     });
        //     println!();
        // });
        time_vec.push(elapsed.as_micros() as f64);
        support_vec.push(i as f64 / 10.0);
    }
    support_vec.reverse();
    let plot_vec: Vec<(f64, f64)> = support_vec
        .iter()
        .zip(time_vec.iter())
        .map(|(x, y)| (*x, *y))
        .collect();
    let graph = plotlib::repr::Plot {
        data: plot_vec,
        line_style: Some(plotlib::style::LineStyle::new().colour("blue")),
        point_style: Some(plotlib::style::PointStyle::new().colour("red")),
        legend: Some("".to_owned()),
    };
    let mut view = ContinuousView::new().add(graph);
    view.add_grid(plotlib::grid::Grid::new(50, 10));
    let page = plotlib::page::Page::single(&view);
    page.save("plot.svg").unwrap();
    // page.to_svg();
    // plotlib::repr::Plot::new(plot_vec);
    // println!("{print:?}");
}

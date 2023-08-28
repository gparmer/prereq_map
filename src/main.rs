use anyhow::{anyhow, Context};
use core::fmt::Display;
//use dot_writer::{Attributes, DotWriter, Shape, Style};
//use itertools::Itertools; // for join on hashset
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use clap::Parser;

type ClassNum = String; // e.g. CSCI3411
type ClassName = String; // e.g. Operating Systems

#[derive(Debug, Deserialize, Serialize, Clone)]
enum Prerequisite<C> {
    #[serde(rename = "course number")]
    Class(C),
    #[serde(rename = "or")]
    Or(Vec<Prerequisite<C>>),
    #[serde(rename = "and")]
    And(Vec<Prerequisite<C>>),
}

type ClassPrerequisite = Prerequisite<ClassNum>;

#[derive(Debug, Deserialize, Serialize, Clone)]
enum Semesters {
    #[serde(rename = "spring")]
    Spring,
    #[serde(rename = "fall")]
    Fall,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ClassRecord {
    #[serde(rename = "course number")]
    num: ClassNum,
    #[serde(rename = "course name")]
    name: ClassName,
    prerequisite: Option<ClassPrerequisite>,
    semesters: Option<Vec<Semesters>>,
}

impl ClassRecord {
    fn new(
        num: ClassNum,
        name: ClassName,
        prerequisite: Option<ClassPrerequisite>,
        semesters: Option<Vec<Semesters>>,
    ) -> Self {
        Self {
            num,
            name,
            prerequisite,
            semesters,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ClassCollection {
    classes: Vec<ClassRecord>,
}

#[derive(Debug)]
struct ClassGraph {
    classes: HashMap<ClassName, ClassRecord>,
}

impl ClassGraph {
    fn new(b: ClassGraphBuilder) -> Self {
        Self {
            classes: b.classes.into_iter().map(|c| (c.name.clone(), c)).collect(),
        }
    }

    // TODO convert into iter, rather than Vec
    fn classes(&self) -> Vec<&ClassRecord> {
        self.classes.values().collect()
    }

    fn prerequisites(&self, n: &ClassName) -> Option<&ClassPrerequisite> {
        self.classes.get(n)?.prerequisite.as_ref()
    }
}

struct ClassGraphBuilder {
    classes: Vec<ClassRecord>,
}

impl ClassGraphBuilder {
    fn new() -> ClassGraphBuilder {
        Self { classes: vec![] }
    }

    fn add(&mut self, c: ClassRecord) {
        self.classes.push(c);
    }

    fn build(self) -> ClassGraph {
        ClassGraph::new(self)
    }
}

trait DotGraph {
    fn graph(&self) -> Vec<u8>;
}

impl DotGraph for ClassGraph {
    fn graph(&self) -> Vec<u8> {
        let output_bytes = Vec::new();
        // {
        //     let mut writer = DotWriter::from(&mut output_bytes);
        //     let mut digraph = writer.digraph();
        //     let mut colors = HashMap::new();
        //     let colormap = [
        //         "cadetblue1",
        //         "chocolate1",
        //         "darkgoldenrod1",
        //         "darkorchid1",
        //         "deeppink",
        //         "dodgerblue2",
        //         "firebrick1",
        //         "gray38",
        //         "green3",
        //         "navy",
        //         "orchid",
        //         "teal",
        //         "violetred",
        //         "yellow1",
        //         "tomato1",
        //     ];
        //     let mut color_idx = 0;

        //     digraph.node_attributes().set("penwidth", "2.5", false);
        //     for c in &self.concepts {
        //         if !colors.contains_key(&c.category) {
        //             assert_ne!(color_idx, colormap.len()); // don't support more than this many categories
        //             colors.insert(&c.category, &colormap[color_idx]);
        //             color_idx += 1;
        //         }
        //     }
        //     for c in &self.concepts {
        //         digraph.node_named(c.graph_name.to_string()).set(
        //             "color",
        //             &colors.get(&c.category).unwrap().to_string(),
        //             true,
        //         );
        //         // unwrap: added in previous loop
        //     }
        //     let summary_name = format!(
        //         "\"Summary\nLecture {:.2} weeks\nLab {:.2} weeks\nHW {:.2} weeks\"",
        //         self.total_weights[0], self.total_weights[1], self.total_weights[2]
        //     );
        //     digraph
        //         .node_named(summary_name.to_string())
        //         .set_shape(Shape::None)
        //         .set_font_size(20.0);

        //     for (cat, col) in &colors {
        //         digraph
        //             .node_named(cat.to_string())
        //             .set_style(Style::Filled)
        //             .set_shape(Shape::Rectangle)
        //             .set("color", col, true);
        //     }
        //     self.dependency_order
        //         .iter()
        //         .map(|o| self.dependency_to_concept(o))
        //         .for_each(|c| {
        //             for d in &c.dependencies {
        //                 let dep_c = self.dependency_to_concept(d);

        //                 digraph.edge(c.graph_name.to_string(), dep_c.graph_name.to_string());
        //             }
        //         });
        // }
        output_bytes
    }
}

fn json_classlist<P: AsRef<std::path::Path>>(p: P) -> std::io::Result<ClassGraph> {
    let file = File::open(p).with_context(|| format!("Could not read in JSON file {}.", p.as_ref().display()))?;
    let buff_file = BufReader::new(file);
    let collection: ClassCollection = serde_json::from_reader(buff_file)?;
    let mut cb = ClassGraphBuilder::new();

    for class in &collection.classes {
        cb.add(class.clone());
    }

    Ok(cb.build())
}

fn english_classlist<P: AsRef<std::path::Path>>(p: P) -> std::io::Result<ClassGraph> {
    let mut cb = ClassGraphBuilder::new();

    Ok(cb.build())
}

/// Simple program to graphically display a prerequisite chain
#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    /// utf-8-formatted list of classes in english
    #[arg(short,long)]
    input: Option<std::path::PathBuf>,
    /// Json-formatted list of classes
    #[arg(short,long)]
    jsinput: Option<std::path::PathBuf>,
    /// Json-formatted output file for all classes
    #[arg(short,long)]
    output: Option<std::path::PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let classes = match (args.input, args.jsinput) {
	(_, Some(ref js)) => json_classlist(js),
	(Some(ref eng), _) => english_classlist(eng),
	(_, _) => Err(anyhow!("Must provide either an english or json class specification."))
    }?;



    Ok(())
}

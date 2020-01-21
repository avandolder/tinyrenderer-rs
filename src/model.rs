use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::geometry::Vec3;

pub struct Model {
    pub verts: Vec<Vec3<f32>>,
    pub faces: Vec<Vec<usize>>,
}

impl Model {
    pub fn new(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut verts = vec![];
        let mut faces = vec![];

        for line in reader.lines() {
            let mut line = line?;
            if line.len() < 2 {
                continue;
            }

            let points = line.split_off(2);
            match &line[0..2] {
                "v " => {
                    let pt = points.split(" ")
                        .flat_map(|x| x.parse::<f32>())
                        .collect::<Vec<f32>>();
                    verts.push(Vec3::new(pt[0], pt[1], pt[2]));
                }
                "f " => {
                    let pt = points.split(" ")
                        .flat_map(|x| x.split("/").next().unwrap().parse::<usize>())
                        .collect::<Vec<usize>>();
                    faces.push(vec![pt[0] - 1, pt[1] - 1, pt[2] - 1]);
                }
                _ => (),
            }
        }

        println!("# v# {} f# {}", verts.len(), faces.len());
        Ok(Self { verts, faces })
    }
}

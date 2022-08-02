/*- Global allowings -*/
#![allow(
    dead_code
)]

/*- Imports -*/
use std::{
    fs,
    path::Path,
    io::Write,
};
pub mod vertice;

/*- Constants -*/
const SCALE_MULTIPLIER:f32 = 40.0f32;
const PATH_TO_FUNCTIONS:&str = "/Users/artur/Library/Application Support/minecraft/saves/dick/datapacks/mc-datapack/data/dick/functions";
const FACE_FILLING_PRECISION:u32 = 1;

/*- Initialize -*/
fn main() -> () {
    let file = ask("File:");
    let vertices:Vec<vertice::Vertice> = get_vertices(&file);
    let _:Vec<vertice::Vertice> = get_faces(&file);

    /*- Get the filename and if the user wants to override -*/
    let mut filename:String = String::new();
    while filename.len() < 1 {
        let f = format!("{}/{}.mcfunction", PATH_TO_FUNCTIONS, ask("Filename"));
        filename = match Path::new(&f).is_file() {
            true => {
                if ask("File already exists, would you like to override? (y/n)") == "y" {
                    /*- Delete the file -*/
                    fs::remove_file(&f).unwrap();

                    /*- Return the filename -*/
                    f
                }else {
                    String::new()
                }
            },

            /*- Return the filename -*/
            false => f
        }
    }

    /*- Output minecraft-function file -*/
    let mut mcfunction = match std::fs::File::create(filename) {
        Ok(e) => e,
        Err(_) => panic!("Can't create mcfunction file. Check r/w permissions")
    };

    /*- Add data to the file -*/
    for vertice in &vertices {
        mcfunction.write(
            &format!(
                "setblock ~{} ~{} ~{} stone\n",
                vertice.0 as i32, vertice.1 as i32, vertice.2 as i32
            ).as_bytes()
        ).unwrap_or(0);
    };
}

/*- Functions -*/
// Extract vertices from .obj file
fn get_vertices(file:&String) -> Vec<vertice::Vertice> {
    /*- Check if file exists and is a .obj file -*/
    let path = Path::new(&file);
    if !path.is_file() && !path.ends_with(".obj") {
        println!("File must be .obj");

        /*- Recurse / ask again for a file -*/
        return get_vertices(&ask("File:"));
    };

    /*- Read file contents as str -*/
    let vertices:Vec<Vec<f32>> = match fs::read_to_string(path) {
        Ok(content) => {
            let lines:Vec<Vec<f32>> = content.split("\n")
                .collect::<Vec<&str>>()
                .into_iter()  // Make it an iterator
                .filter(|item| {
                    item.starts_with("v\u{0020}") // Get only vertices
                })
                .collect::<Vec<&str>>() // Make it a vector of strs again
                .into_iter()
                .map(|element| {
                    element.split("\u{0020}") // Get every vertice in the line (x, y, z)
                    .collect::<Vec<&str>>()
                    .get(1..) // Get every vertice but remove the "v" in the beginning
                    .unwrap_or(vec![].as_slice())
                    .iter().map(|num| {
                        num.parse::<f32>() // Parse every vertice-axis to a number
                            .unwrap_or(0.0)
                    })
                    .collect::<Vec<f32>>()
                })
                .collect::<Vec<Vec<f32>>>();

            lines
        },
        Err(_) => {
            println!("Couldn't load file into buffer");
    
            /*- Recurse / ask again for a file -*/
            return get_vertices(&ask("File:"));
        }
    };

    /*- Now make it into an array of the struct "Vertice" -*/
    let mut output:Vec<vertice::Vertice> = Vec::with_capacity(vertices.len());
    for i in 0..vertices.len() {
        let xyz = vertices[i].clone();
        output.push(
            vertice::Vertice(
                *xyz.get(0).unwrap_or(&0.0) * SCALE_MULTIPLIER,
                *xyz.get(1).unwrap_or(&0.0) * SCALE_MULTIPLIER,
                *xyz.get(2).unwrap_or(&0.0) * SCALE_MULTIPLIER
            )
        );
    };

    output
}

// Extract faces from .obj file
fn get_faces(file:&String) -> Vec<vertice::Vertice> {
    /*- Check if file exists and is a .obj file -*/
    let path = Path::new(&file);
    if !path.is_file() && !path.ends_with(".obj") {
        println!("File must be .obj");

        /*- Recurse / ask again for a file -*/
        return get_vertices(&ask("File:"));
    };

    /*- Read file contents as str -*/
    let faces:Vec<Vec<u32>> = match fs::read_to_string(path) {
        Ok(content) => {
            let faces:Vec<Vec<u32>> = content.split("\n")
                .collect::<Vec<&str>>()
                .into_iter()  // Make it an iterator
                .filter(|item| {
                    item.starts_with("f\u{0020}") // Get only faces
                })
                .collect::<Vec<&str>>() // Make it a vector of strs again
                .into_iter()
                .map(|element| {
                    element.split("\u{0020}") // Get every face in the line v/vn/vp v2/vn2/vp2
                    .collect::<Vec<&str>>()
                    .get(1..) // Get every vertice but remove the "f" in the beginning
                    .unwrap_or(vec![].as_slice())
                    .iter().map(|vertice| {
                        vertice.split("/")
                            .collect::<Vec<&str>>()
                            .get(0).unwrap_or(&"0")
                            .parse::<u32>() // Parse every vertice-axis to a number
                            .unwrap_or(0)
                    })
                    .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>();

            println!("{faces:?}");
            faces
        },
        Err(_) => {
            println!("Couldn't load file into buffer");
    
            /*- Recurse / ask again for a file -*/
            return get_faces(&ask("File:"));
        }
    };

    /*- Now make it into an array of the struct "Vertice" -*/
    let output:Vec<vertice::Vertice> = Vec::with_capacity(faces.len());
    // for i in 0..faces.len() {
    //     let xyz = vertices[i].clone();
    //     output.push(
    //         vertice::Vertice(
    //             *xyz.get(0).unwrap_or(&0.0) * SCALE_MULTIPLIER,
    //             *xyz.get(1).unwrap_or(&0.0) * SCALE_MULTIPLIER,
    //             *xyz.get(2).unwrap_or(&0.0) * SCALE_MULTIPLIER
    //         )
    //     );
    // };

    output
}

// Get input from the user
fn ask(question:&str) -> String {
    print!("==> {question} ");
    std::io::stdout().flush().unwrap_or(());

    /*- Input will be stored here -*/
    let mut input:String = String::new();

    /*- Get the stdin -*/
    match std::io::stdin()
        .read_line(&mut input) {
        
            Ok(e) => e,
            Err(_) => panic!("Error parsing input!")
    };

    /*- Return -*/
    input.replace("'", "").trim().to_string()
}
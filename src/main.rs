use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::{Event, Parser};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    println!("Linting {}", path);

    let mut content = String::new();
    let optSvg = svg::open(path, &mut content);

    match optSvg {
        Ok(svg) => lintSvg(&svg),
        _ => {},
    }
}

fn lintSvg(svg: &Parser) {
    for event in svg {
        match event {
            Event::Tag(Path, _, attributes) => {
                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();
                for command in data.iter() {
                    match command {
                        &Command::Move(..) => { /* … */ },
                        &Command::Line(..) => { /* … */ },
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::fs;
use std::io::{Read, Write};

// TODO:
//      multiple target files
//      commands to execute afterwards
//      create example json file

pub struct Config {
    pub target_filename: String,
    pub pattern_filename: String,
    pub verbosity: u64,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let patterns = patternize(&config.pattern_filename)?;

    // open targetfile and copy content into string contents
    let mut target_src = fs::File::open(&config.target_filename)?;
    let mut contents = String::new();
    target_src.read_to_string(&mut contents)?;
    drop(target_src);  // Close the file early

    // perform search and replacement
    repler_replace(&mut contents, &patterns);

    // write file and save the new content to it
    let mut target_dst = fs::File::create(&config.target_filename)?;
    target_dst.write(contents.as_bytes())?;

    Ok(())
}

#[derive(Deserialize, Debug, PartialEq)]
struct Item {
    name: Option<String>,
    from: String,
    to: String,
    times: Option<usize>,
}

/// extracts the patterns of the patternfile into a Vec<Item>
fn patternize(filename: & str) -> Result<Vec<Item>, Box<Error>> {
    let mut content = String::new();
    let mut file = fs::File::open(filename)?;
    file.read_to_string(&mut content)?;
    let result = serde_json::from_str::<Vec<Item>>(&content)?;
    Ok(result)
}

/// replacing the text of the target text with the patterns
fn repler_replace(contents: &mut String, patterns: &Vec<Item>) {
    for pattern in patterns {
        *contents = match pattern.times {
            Some(n) => contents.replacen(&pattern.from, &pattern.to, n),
            None => contents.replace(&pattern.from, &pattern.to)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replacing_in_sample_text() {
        let patterns = vec![
            Item {
                name: Some(String::from("Duct repl")),
                from: String::from("Duct t"),
                to: String::from("gr"),
                times: None,
            },
            Item {
                name: Some(String::from("make better")),
                from: String::from("e"),
                to: String::from("er"),
                times: Some(2),
            }
        ];
        let mut contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.".to_string();

        repler_replace(&mut contents, &patterns);

        assert_eq!("\
Rust:
safer, fast, productiver.
Pick three.
grape.", &contents);
    }

    #[test]
    fn patternize_from_nonexisting_file() {
        let pattern_filename = "pat.json";
        let val = patternize(pattern_filename);
        match val {
            Ok(_) => panic!("hmm"),
            Err(_) => {}
        }
    }

    #[test]
    fn patternize_from_existing_file() {
        let pattern_filename = "patterns.json";
        let val = patternize(pattern_filename).unwrap();
        assert_eq!(vec![Item {
            name: Some(r#""A" to "E""#.to_string()),
            from: "a".to_string(),
            to: "e".to_string(),
            times: None,
        }, Item {
            name: None,
            from: "S".to_string(),
            to: "T".to_string(),
            times: Some(3),
        }], val);
    }

    #[test]
    fn replace_in_created_textfile() {
        let config = Config {
            target_filename: "Osterspaziergang.txt".to_string(),
            pattern_filename: "patterns.json".to_string(),
            verbosity: 2,
            };
        let mut file = fs::File::create(&config.target_filename).unwrap();
        let text = "Vom Eise befreit sind Strom und Bäche
Durch des Frühlings holden, belebenden Blick,
Im Tale grünet Hoffnungsglück;
Der alte Winter, in seiner Schwäche,
Zog sich in rauhe Berge zurück.
Von dort her sendet er, fliehend, nur
Ohnmächtige Schauer körnigen Eises
In Streifen über die grünende Flur.
Aber die Sonne duldet kein Weißes,
Überall regt sich Bildung und Streben,
Alles will sie mit Farben beleben;...";
        file.write_all(text.as_bytes()).unwrap();
        run(config).unwrap();
        let mut file = fs::File::open("Osterspaziergang.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, text.replace("a", "e").replacen("S", "T", 3));
    }
}
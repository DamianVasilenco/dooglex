use std::{env, fs};
use std::collections::HashMap;
use std::ffi::OsStr;
use docx::document::ParagraphContent::Run;
use docx::document::{BodyContent, RunContent};
use docx::DocxFile;

struct DoogleX {}

impl DoogleX {
    fn search(input_dir: String, string_to_search: String) -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();
        let paths = fs::read_dir(input_dir).unwrap();

        for file in paths {
            let path = file.unwrap().path();
            let extension = path.extension().and_then(OsStr::to_str);

            match extension {
                None => {continue;}
                Some(ext) => {
                    if ext != "docx" {
                        continue;
                    }
                }
            }

            let file_name = path.into_os_string().into_string().unwrap();
            let docx = DocxFile::from_file(file_name.clone()).unwrap();
            let docx = docx.parse().unwrap();

            for content in docx.document.body.content.iter() {
                if let BodyContent::Paragraph(paragraph) = content {
                    let paragraph_content = paragraph.content.get(0).unwrap();
                    if let Run(run) = paragraph_content {
                        let run_content = run.content.get(0);
                        match run_content {
                            None => {}
                            Some(content) => {
                                if let RunContent::Text(text) = content {
                                    let cow = &text.text;
                                    let mut string = String::new();
                                    string.push_str(cow);

                                    if string.contains(&string_to_search) {
                                        // println!("{}", file_name);
                                        // println!("- {}", string);
                                        // println!("");
                                        map.entry(String::from(&file_name)).or_insert(Vec::new()).push(String::from(&string));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        map
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let input_dir = args[1].clone();
    let string_to_search = args[2].clone();
    let map = DoogleX::search(input_dir, string_to_search);

    // Print the map
    for (key, value) in &map {
        println!("{}", key);

        for v in value {
            println!("- {}", v);
        }

        println!()
    }
}

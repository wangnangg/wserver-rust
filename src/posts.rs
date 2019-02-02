#![feature(proc_macro_hygiene, decl_macro)]

extern crate regex;

use regex::Regex;
use rocket::get;

use rocket_contrib::templates::Template;
use std::cmp::{Ord, Ordering, Reverse};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Write;
use std::fs;
use std::io;
use std::io::BufRead;

use lazy_static;

const TEMPLATE_DIR: &str = "root/templates";
const POSTS_DIR: &str = "root/posts";

fn file_with_ext(dentry: &fs::DirEntry, target_ext: &str) -> bool {
    if let Some(ext) = dentry.path().extension() {
        if ext.to_str() == Some(target_ext) {
            return true;
        }
    }
    return false;
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Copy, Clone)]
struct Date {
    year: i32,
    month: u8,
    day: u8,
}
impl Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

struct PostMeta {
    url: String,
    title: String,
    date: Date,
}

fn extract_title(line: &str) -> Option<String> {
    lazy_static! {
        static ref re: Regex = Regex::new(r"^title:\s*(.+)$").unwrap();
    }
    let caps = match re.captures(line) {
        Some(caps) => caps,
        None => return None,
    };
    match caps.get(1) {
        Some(m) => Some(m.as_str().to_owned()),
        None => None,
    }
}

fn extract_date(line: &str) -> Option<Date> {
    lazy_static! {
        static ref re: Regex = Regex::new(r"^date:\s*(\d+)-(\d+)-(\d+)\s*$").unwrap();
    }
    let caps = match re.captures(line) {
        Some(caps) => caps,
        None => return None,
    };
    let year: i32 = match caps.get(1) {
        Some(m) => m.as_str().parse().unwrap(),
        None => return None,
    };
    let month: u8 = match caps.get(2) {
        Some(m) => m.as_str().parse().unwrap(),
        None => return None,
    };
    let day: u8 = match caps.get(3) {
        Some(m) => m.as_str().parse().unwrap(),
        None => return None,
    };
    Some(Date { year, month, day })
}

fn get_post_meta(post_name: &str) -> Option<PostMeta> {
    let file = fs::File::open(POSTS_DIR.to_owned() + "/" + &post_name + ".md").unwrap();
    let reader = io::BufReader::new(file);

    let mut title = None;
    let mut date = None;
    for line_res in reader.lines() {
        if let Ok(line) = line_res {
            if title.is_none() {
                title = extract_title(&line);
            }
            if date.is_none() {
                date = extract_date(&line);
            }
        }
    }

    if let Some(t) = title {
        if let Some(d) = date {
            Some(PostMeta {
                url: "/posts/".to_owned() + post_name,
                title: t,
                date: d,
            })
        } else {
            None
        }
    } else {
        None
    }
}

#[get("/raw")]
pub fn index_markdown() -> String {
    let read_dir = fs::read_dir(POSTS_DIR).unwrap();
    let mut posts_vec = vec![];
    for f in read_dir {
        if let Ok(dentry) = f {
            if file_with_ext(&dentry, "md") {
                posts_vec.push(dentry);
            }
        }
    }

    let mut meta_vec = vec![];
    for d in posts_vec {
        let fpath = d.path();
        let fname = fpath.file_stem().unwrap().to_str().unwrap();
        meta_vec.push(match get_post_meta(fname) {
            Some(m) => m,
            None => panic!("unable to get meta from {}", fname),
        });
    }

    meta_vec.sort_by_key(|meta| Reverse(meta.date));

    let mut markdown = String::new();

    for meta in meta_vec {
        writeln!(
            markdown,
            "* [{} ({:?})]({})",
            meta.title, meta.date, meta.url
        )
        .unwrap();
    }

    markdown
}

#[get("/")]
pub fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("posts/index", &context)
}

use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use std::path::Path;

#[get("/<post_name>/raw")]
pub fn post_markdown(post_name: String) -> Option<NamedFile> {
    NamedFile::open(Path::new(POSTS_DIR).join(post_name + ".md")).ok()
}

#[get("/<post_name>")]
pub fn post(post_name: String) -> Result<Template, NotFound<String>> {
    let mut context: HashMap<String, String> = HashMap::new();

    context.insert(
        "markdown_url".to_owned(),
        format!("/posts/{}/raw", post_name),
    );

    let meta = get_post_meta(&post_name).unwrap();
    context.insert("title".to_owned(), meta.title);
    context.insert("date".to_owned(), format!("{:?}", meta.date));
    Ok(Template::render("posts/post", &context))
}
#[cfg(test)]
mod tests {
    use super::*;

}

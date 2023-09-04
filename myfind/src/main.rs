use regex::Regex;
use std::env;

use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    // 参数 1：搜索目录；参数 2：要搜索的正则表达式。
    if args.len() < 3 {
        eprintln!("使用方式：{} <目标目录> <要搜索的正则表达式>", args[0]);
        process::exit(1);
    }
    //思考一下：如果用户输入的参数太多，应该怎么样？
    let pattern = &args[2]; //输入的正则表达式
    let regex = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("无效的正则表达式'{}':{}", pattern, err);
            process::exit(1);
        }
    };
    match FindMod::find(&args[1], &regex) {
        Ok(matches) => {
            if matches.is_empty() {
                println!("未找到匹配项。");
            } else {
                println!("找到以下匹配项：");
                for file in matches {
                    println!("{}", file);
                }
            }
        }
        Err(error) => {
            eprintln!("发生错误：{}", error);
            process::exit(1);
        }
    }
}

mod find_mod {
    use regex::Regex;
    use std::path::Path;

    use crate::walk_treeMod;

    pub fn find<P: AsRef<Path>>(
        root: P,
        regex: &Regex,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut matches = Vec::new();
        walk_treeMod::walk_tree(root.as_ref(), regex, &mut matches)?;
        Ok(matches)
    }
}

mod walk_tree_mod {

    use regex::Regex;
    use std::fs;
    use std::path::Path;

    pub fn walk_tree(
        dir: &Path,
        regex: &Regex,
        matches: &mut Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 如果不是，应该怎么办呢？
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    walk_tree(&path, regex, matches)?;
                } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    if regex.is_match(filename) {
                        matches.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
        Ok(())
    }
}

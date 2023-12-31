/*
3210103818
杨朗骐

完成：
1.将该代码重构到多个模块中
2.增加新的功能，⽐如 –v/--verbose 参数输出所有遍历到的⽂件
3.同时⽀持匹配多个正则
4.给输出结果去重排序
*/

use regex::Regex;
use std::env;
use clap::Parser;
use std::process;
#[derive(Debug,Parser)]
struct Cli{
    location: String,
    regex: Vec<String>,
    #[arg(short, long)]
    verbose: bool
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let cli=Cli::parse();
    // 参数 1：搜索目录；参数 2：要搜索的正则表达式。
    if args.len() < 3 {
        eprintln!("使用方式：{} <目标目录> <要搜索的正则表达式>", args[0]);
        process::exit(1);
    }

    //思考一下：如果用户输入的参数太多，应该怎么样？
    //答：在这里采用多条件“或”的方式
    let location = cli.location;
    let patterns=cli.regex;
    let verbose = cli.verbose;
    let mut counter = 0;

    let mut all: Vec<String> = Vec::new();

    loop {
        if counter >= patterns.len() {
            break;
        }
        eprintln!("正在处理正则表达式[{}]...", counter + 1);
        let pattern = &patterns[counter];
        counter = counter + 1;

        let regex = match Regex::new(pattern) {
            Ok(re) => re,
            Err(err) => {
                eprintln!("无效的正则表达式'{}':{}", pattern, err);
                process::exit(1);
            }
        };

        match find_mod::find(&location, &regex,verbose) {
            Ok(matches) => {
                if matches.is_empty() {
                    println!("未找到匹配项。");
                } else {
                    let mut temp: Vec<String> = matches.clone();
                    all.append(&mut temp);
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

    if !all.is_empty(){
        all.sort();
        all.dedup();
        println!("\n\n合并、去重、排序后的匹配项如下：");
        for file in all {
            println!("{}", file);
        }
    }
    
}

mod find_mod {
    use crate::walk_tree_mod;
    use regex::Regex;
    use std::path::Path;

    pub fn find<P: AsRef<Path>>(
        root: P,
        regex: &Regex,
        verbose: bool
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut matches = Vec::new();
        walk_tree_mod::walk_tree(root.as_ref(), regex, &mut matches,verbose)?;
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
        verbose: bool
    ) -> Result<(), Box<dyn std::error::Error>> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                let regex_str = regex.to_string();
                if path.is_dir() {
                    walk_tree(&path, regex, matches,verbose)?;
                } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    if regex.is_match(filename) || verbose {
                        matches.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
        Ok(())
    }
}

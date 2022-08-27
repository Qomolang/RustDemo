use std::{fs, ops::Add};
use walkdir::{DirEntry, WalkDir};

//寻找目录下所有无结尾和非.md结尾的文件
fn main() {
    println!("step into");

    let dirFullPath = r"D:\ssh\Gamma";

    let mut file_not_md_number = 0;
    let mut file_total_number = 0;

    let entry = WalkDir::new(dirFullPath)
        .into_iter()
        //过滤隐藏文件
        .filter_entry(|file| !is_hidden(file))
        //过滤掉用户无目录权限等问题
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_type().is_file());

    //todo 采用了一种很丑陋的设计 clone 后面看下怎么搞
    for file in entry {
        let fileFullPath = file.path().display().to_string();
        let former_name = fileFullPath.clone();
        let mut rename = fileFullPath.clone();

        //给没有尾缀的文件添加尾缀
        if (!fileFullPath.contains(".")) {
            rename.push_str(".md");
            println!("{}", former_name);
            println!("{}", rename);
            fs::rename(former_name, rename);
        }

        // //替换txt为md
        // if (fileFullPath.contains(".txt")) {
        //     rename = rename.replace(".txt", ".md");
        //     println!("{}", former_name);
        //     println!("{}", rename);
        // }

    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

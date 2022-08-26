use walkdir::WalkDir;

//寻找目录下所有无结尾和非.md结尾的文件
fn main() {
    print!("step into");

    for entry in WalkDir::new(path)
         {
            let entry = entry.unwrap();
            println!("{}", entry.path().display());
        }

}

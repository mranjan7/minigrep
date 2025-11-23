use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read file");
    println!("contents {contents}");
    dbg!(args);
}
struct Config{
    query: String,
    file_path: String,
    }
impl Config{
    fn build(args: &[String]) -> Result<Config, 'static &str>{
        if args.len() < 3 {
            Err("not enough arguments")
            }
        let query = args[1].clone();
        let file_path = args[2].clone();
        println!("query {}",query);
        println!("file {}",file_path);
        Ok(Config{query, file_path})
     }
}
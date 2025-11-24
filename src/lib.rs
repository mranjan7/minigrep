pub fn search<'a> (query: &str,contents: &'a str) -> Vec<&'a str>{
    let mut res = Vec::new();
    for line in contents.lines(){
            if line.contains(query){
                    res.push(line);
                }
        }
        res
    }
#[cfg(test)]
mod tests{
    use crate::search;
    #[test]
    fn one_result(){
        let query = "duct";
        let content = "\
        duct,tape
        cello
        ";
        assert_eq!(vec!["duct,tape"],search(query,content));
        }
    }
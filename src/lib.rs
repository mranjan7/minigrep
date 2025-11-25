pub fn search<'a> (query: &str,contents: &'a str) -> Vec<&'a str>{
    contents.lines().filter(|line| line.contains(query)).collect()
    }
pub fn search_case_insensitive<'a> (query: &str,contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
    }
#[cfg(test)]
mod tests{
    use crate::search;
    use crate::search_case_insensitive;
    #[test]
    fn case_sensitive(){
        let query = "duct";
        let content = "\
        duct,tape
        cello
        ";
        assert_eq!(vec!["duct,tape"],search(query,content));
        }
    #[test]
    fn case_insensitive(){
        let query = "Duct";
        let content = "\
        duct,tape
        cello
        ";
        assert_eq!(vec!["duct,tape"],search_case_insensitive(query,content));
        }

    }
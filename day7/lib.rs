use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum FileOrDirInfo {
    Dir { name: String },
    File { name: String, size: i64 },
}

fn commands_to_filesizelist(commandlog: &str) -> Result<Vec<FileOrDirInfo>, String> {
    // cd .. pops
    // cd <x> pushes
    // cd / clears stack
    // ls causes to read lines to next command line.
    // Maintain current path as a vector.
    // Parse replace filenames with absolute path (joined current path)
    // Need type for file or dir?
    // Accumulate sizes in a hashmap - when you see a file, accumulate its size all the way up the path.
    let mut linenum = 0;
    let mut lsmode = false; // Expecting next line to be a command when true, else expecting output.
    let mut path = Vec::<String>::new();
    let mut filesizelist = Vec::<FileOrDirInfo>::new();
    path.push(String::from("/"));  // Assume we are at the root dir.
    let mut ii = commandlog.split("\n"); 
    // Ensure we start by going to the root dir.
    let firstlineopt = ii.next(); 
    linenum += 1;
    if let None = firstlineopt {
        return Err(format!("L{}: There should be at least 1 line", linenum));
    }
    if firstlineopt != Some("$ cd /") {
        return Err(format!("L{}: First line was not '$ cd /'", linenum));
    }
    filesizelist.push(FileOrDirInfo::Dir { name: "/".to_string(),});
    loop {
        // Examine the next line.
        let lineopt = ii.next();    
        linenum += 1;
        if let None = lineopt {
            break;
        }
        let line = lineopt.unwrap();
        if line == "" { continue; }
        // If line starts with a '$' then it is a command.  
        // If line does not and we are in ls mode, then collect the ls output lines until we hit another command.
        // If line does not start with '$' and we not in ls mode, then it is bad input.
        let words: Vec<&str> = line.split(" ").collect();
        if words.len() > 3 || words.len() < 2 {
            dbg!(words);
            return Err(format!("L{}: Line does not have 2 or 3 words", linenum));
        }
        let emptystr = "";
        let w1 = words[0];
        let w2 = words[1];
        let w3 = *words.get(2).unwrap_or(&emptystr);
        match (w1, w2, w3) {
            ("$", "ls", w3) => {
                if w3 != emptystr {
                    return Err(format!("L{}: Error: ls command has argument.", linenum));
                }
                lsmode = true;
                //println!("L{}: Line begins ls mode", linenum);

            },
            ("$", "cd", dir) => {
                if lsmode { 
                    //println!("L{}: Line ends ls mode", linenum);
                    lsmode = false;
                }
                match dir {
                    ".." =>  if let None = path.pop() { 
                        return Err(format!("L{}: Error: cd command tried to go above root dir.", linenum));
                    },
                    "/" => path.clear(),
                    _ => path.push(format!("{}/", dir)),
                };
                //println!("L{}: New working dir: {}", linenum, path.join(""));
            },
            (a, b, c)=> {
                if !lsmode {
                    return Err(format!("L{}: Did not expect output without preceeding ls command.", linenum));
                }
                if c != "" {
                    return Err(format!("L{}: Did not expect third word on an output line.", linenum));
                }
                match (a, b) {
                    ("dir", dirname) => {
                        let abspath = format!("{}{}", path.join(""), dirname);
                        //println!("L{}:  found dir with absolute path {:#?}", linenum, abspath);
                        let fdi = FileOrDirInfo::Dir {name: abspath};
                        filesizelist.push(fdi);
                    },
                    (sizestr, filename) => {
                        let abspath = format!("{}{}", path.join(""), filename);
                        let size: i64 = sizestr.parse::<i64>().unwrap();
                        //println!("L{}:  found file with (size, abspath): ({}, {})", linenum, size, abspath);
                        let fdi = FileOrDirInfo::File { name: abspath, size: size };
                        filesizelist.push(fdi);
                    },
                };
            },
        };
    }
    Ok(filesizelist)
}


#[test]
fn test_commands_to_filesizelist_goodinput() {
    const SIMPLE_TEST_INPUT: &str = "\
$ cd /
$ ls
dir a
10 x.txt
20 y.dat
$ cd a
$ ls
30 z.json
";
    let result = commands_to_filesizelist(SIMPLE_TEST_INPUT);
    assert!(result.is_ok());
    let fsl = result.unwrap();
    dbg!(&fsl);
    let expected = vec![
        FileOrDirInfo::Dir{ name: "/".to_string() },
        FileOrDirInfo::Dir{ name: "/a".to_string() },
        FileOrDirInfo::File{ name: "/x.txt".to_string(), size: 10_i64 }, 
        FileOrDirInfo::File{ name: "/y.dat".to_string(), size: 20_i64 }, 
        FileOrDirInfo::File{ name: "/a/z.json".to_string(), size: 30_i64 }, 
    ];
    assert_eq!(&fsl, &expected);
}

#[test]
fn test_commands_to_filesizelist_badinputs() {
    const NO_ROOT: &str = "\
$ ls
dir a
10 x.txt
";
    let result = commands_to_filesizelist(NO_ROOT);
    assert!(!result.is_ok());

    const NONSENSE: &str = "\
la la la la la 
";
    let result = commands_to_filesizelist(NONSENSE);
    assert!(!result.is_ok());
}

// Something like std::path::Path::Ancestors iterator, but without OsStr.
struct PathAncestors<'a> {
    v: Vec<&'a str>,
}

impl<'a> Iterator for PathAncestors<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        // Pop off the path component used in the previous iteration.
        // In the case of the first iteration, there is a dummy final empty string to pop.
        // This is ensured by the constructor.
        let ret = match (self.v.len(), self.v.get(0)) {
            (_, None) | (0, _) => None,
            (1, Some(&"")) =>  {
                // For absolute path, the final item popped is "", but we want to return "/"
                Some(String::from("/"))
            },
            (_, _) => {
                Some(self.v.join("/"))
            },
        };
        self.v.pop();
        ret
    }
}

/// Returns a PathAncestors sequence generator.
/// Does not check for degenerate paths like "/a//b".
/// Does not handle OS specific paths, like "c:\a\b".
/// Same iterations returned for "/a/b/" and "/a/b".
/// For absolute path, last iteration returns "/".
fn path_ancestors(s: &str) -> PathAncestors {
    // Remove any final / so we don't return something like "/a/b/".
    let mut tmp = s;
    if s.ends_with("/") {
        tmp = s.strip_suffix("/").expect("Should have been able to strip string.");
    }
    PathAncestors { 
        v: tmp.clone().split("/").collect(), 
    }
}

#[test]
fn test_path_ancestors(){
    let mut pa = path_ancestors("/foo/bar");
    assert_eq!(pa.next(), Some(String::from("/foo/bar")));
    assert_eq!(pa.next(), Some(String::from("/foo")));
    assert_eq!(pa.next(), Some(String::from("/")));
    assert_eq!(pa.next(), None);
    assert_eq!(pa.next(), None);
}


fn compute_dir_sizes(filesizelist: Vec<FileOrDirInfo>) -> HashMap<String, i64> {
   let mut h = HashMap::<String, i64>::new();
   for item in filesizelist {
    match item {
        FileOrDirInfo::Dir { name } => {
            // Add entry for dir as zero if not already present
            if !h.contains_key(&name) {
                h.insert(name, 0);
            }
        }, 
        FileOrDirInfo::File { name, size } => {
            // iterate up the dir ancestor, adding size to each ancestor dir.
            let ancestors = path_ancestors(name.as_str());
            let mut i: usize = 0;
            for path in ancestors {
                // Skip first component which is filename, not dir.
                if i == 0 { i+=1; continue; }
                i+=1;
                *h.get_mut(&path).expect("Should have found parent dir") += size;
            }
        }, 
    }
   }
   h
}

#[test]
fn test_compute_dir_sizes() {
    let filesizelist = vec![
        FileOrDirInfo::Dir{ name: "/".to_string() },
        FileOrDirInfo::Dir{ name: "/a".to_string() },
        FileOrDirInfo::File{ name: "/x.txt".to_string(), size: 10_i64 }, 
        FileOrDirInfo::File{ name: "/y.dat".to_string(), size: 20_i64 }, 
        FileOrDirInfo::File{ name: "/a/z.json".to_string(), size: 35_i64 }, 
    ];
    let hm = compute_dir_sizes(filesizelist);
    assert_eq!(hm.len(), 2);
    assert!(hm.contains_key("/"));
    assert!(hm.contains_key("/a"));
    assert_eq!(*hm.get("/").unwrap(), 65_i64);
    assert_eq!(*hm.get("/a").unwrap(), 35_i64);
}

/// Solve day7 problem from Advent of Code 2022.
pub fn do_day7(input: &str, mode: i32) -> String {
    let result = commands_to_filesizelist(input);
    if let Err(s) = result {
        panic!("{}", s)    
    }
    let fsl = result.unwrap();
    let hm = compute_dir_sizes(fsl);
    // filter to the directories with a total size of at most 100000, then calculate the sum of their total sizes.
    match mode {
        1 => {
            let total_100kb_or_less: i64 = hm.values().filter(|x| **x <= 100_000_i64).sum();
            format!("{}", total_100kb_or_less)
        },
        2 => {
            // Free space must be 30000000 of 70000000.
            let free_space_req: i64 = 30000000;
            let storage_size: i64 =   70000000;
            let used_space_start: i64 = *hm.get("/").unwrap();
            let free_space_start: i64 = storage_size - used_space_start;
            let mut delete_at_least: i64 = 0;
            if free_space_start < free_space_req {
                delete_at_least = free_space_req - free_space_start;
            }
            
            let size_of_dir_to_delete = hm.values()
                .filter(|x| **x >= delete_at_least)
                .min().unwrap();
            // Wasn't able to figure out how to get the key of the min dir with functional programming.
            // Where is argmin?
            // let mut dir_to_delete: (String, i64) = (String::from("ERROR"), i64::MAX);
            // for (k, v) in hm {
            //     if v < free_space_req { continue; }
            //    if v < dir_to_delete.1 {
            //        dir_to_delete = (k, v);
            //    }
            //}
            //dbg!(&dir_to_delete);
            //format!("used: {} free: {} need_to_free: {}, size_of_dir_to_delete: {}", used_space_start, free_space_start, delete_at_least, size_of_dir_to_delete)
            format!("{}", size_of_dir_to_delete)

        },
        _ => panic!("Invalid mode"),
    }
}

#[cfg(test)]
const ADVENT_TEST_INPUT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

#[test]
fn test_do_day7_prob1_test_input() {
    let expected = format!("{}", 95437);
    let actual = do_day7(ADVENT_TEST_INPUT, 1);
    assert_eq!(String::from(expected), actual);
}

#[test]
fn test_do_day7_prob2_test_input() {
    let expected = format!("{}", 24933642);
    let actual = do_day7(ADVENT_TEST_INPUT, 2);
    assert_eq!(String::from(expected), actual);
}

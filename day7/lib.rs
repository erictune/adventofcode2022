use std::collections::HashSet;

fn commands_to_filesizelist(commandlog: &str) -> Result<HashSet<i32>, String> {
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
                println!("L{}: Line begins ls mode", linenum);

            },
            ("$", "cd", dir) => {
                if lsmode { 
                    println!("L{}: Line ends ls mode", linenum);
                    lsmode = false;
                }
                match dir {
                    ".." =>  if let None = path.pop() { 
                        return Err(format!("L{}: Error: cd command tried to go above root dir.", linenum));
                    },
                    "/" => path.clear(),
                    _ => path.push(format!("{}/", dir)),
                };
                println!("L{}: New working dir: {}", linenum, path.join(""));
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
                        println!("L{}:  found dir with absolute path {}{}", linenum, path.join(""), dirname);
                    },
                    (size, filename) => {
                        println!("L{}:  found abspath file {}{} with size {}", linenum, path.join(""), filename, size);
                    },
                };
            },
        };
    }
    Ok(HashSet::new())
}

const TEST_INPUT: &str = "\
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
fn test_commands_to_filesizelist() {
    let result = commands_to_filesizelist(TEST_INPUT);
    assert!(result.is_ok());
    let _fsl = result.unwrap();
    let _expected_items = vec![
        ("/a/e", 584, 1), 
        ("/a", 94853, 2), 
        ("/d", 24933642, 3),
        ("/",  48381165, 4),
    ];
    // Compare row-wise or use a vector comparer?
    // assert_eq!(fsmap, expected);
}

/// Solve day7 problem from Advent of Code 2022.
pub fn do_day7(input: &str, _mode: i32) -> String {
    let result = commands_to_filesizelist(input);
    match result {
        Ok(fsl) => return format!("{:#?}", fsl),
        Err(s) => panic!("{}", s),
    }
    // find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes.
}

#[test]
fn test_do_day7_with_prob1_test_input() {
    let _expected = 95437;
    let _fsl = commands_to_filesizelist(TEST_INPUT);

}
//#[test]
//fn test_do_day7_with_prob1_test_inputs() {
//    for (input, expected, casenum) in cases {
//        assert_eq!(do_day7(input, 4), expected, "in case number {}", casenum);
//    }
//}

use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::PathBuf,
};

//use rust_fuzzy_search::fuzzy_compare;

/// searches the file from line start onwards until it finds max number of results, then returns results and last line it checked
pub async fn find(logfile: PathBuf, term: String) -> Result<Vec<Vec<String>>, std::io::Error> {
    let mut path: Vec<String> = Vec::new();
    let mut paths: Vec<Vec<String>> = Vec::new();
    let file = OpenOptions::new().read(true).open(logfile)?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        let line = line.trim();
        let token = line.trim_end_matches('{').trim().replace('\t', "");

        // found result
        if token.contains(&term) {
        // if fuzzy_compare(&token, &term) >= crate::TERM_THRESHHOLD {
        //if get_score(&token, &term) >= crate::TERM_THRESHHOLD {
            let mut found = path.clone();
            found.push(token.to_owned());
            paths.push(found);
            // found max number of paths
            if paths.len() >= crate::MAX_RESULTS {
                break;
            }
            continue;
        }

        // start token
        if line.ends_with('{') {
            path.push(token.to_owned());
            continue;
        }

        // end token, no result found
        if line.ends_with('}') {
            path.pop();
        }
    }
    Ok(paths)
}

/*
// calculates a percentage of how closely a term matches
pub fn get_score(src: &str, search: &str) -> f32 {
    let mut found = 0;
    let mut src_mut = src.chars().collect::<Vec<_>>();
    let mut fixup = 1.0;

    // we go through each character of the search term
    'outer: for (search_idx, elem_search) in search.chars().enumerate() {
        // we match to each char of our src string, if found, we get new src string part and compare with next character
        for (src_idx, elem_src) in src_mut.iter().enumerate() {
            // we found a character, so we take the remainder of src and try to find the next character
            if elem_search == *elem_src {
                // we take from current src and remove everthing before; then we start searching for the next string
                //println!("found {elem_search} at {src_idx} in {src_mut}");
                if src_idx < src.len() {
                    src_mut = src_mut[src_idx..].iter().cloned().collect();
                }
                found += 1;
                // fixup shows, how many tries we needed to find all the searched characters,
                // more than search.len() shows, that we needed to skip characters that didnt match, lowering score
                //fixup = search.len() as f32 / (search_idx + 1) as f32;
                // if we found every char from search, we stop completely
                if found == src.len() {
                    break 'outer;
                }
                // if we found a single char, we start looking for the next one
                break;
            }
        }
    }
    if search.len() == 0 {
        return -1.0;
    }

    // percentage of how many characters match + fixup
    (found as f32 / src.len() as f32) * fixup
}
*/

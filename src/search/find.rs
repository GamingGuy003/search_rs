use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub async fn find(logfile: PathBuf, term: String) -> Result<Vec<Vec<String>>, std::io::Error> {
    let mut path: Vec<String> = Vec::new();
    let mut paths: Vec<Vec<String>> = Vec::new();
    let file = OpenOptions::new().read(true).open(logfile)?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        let line = line.trim();
        let token = line.trim_end_matches('{').trim();

        // found result
        if token.contains(&term) {
            let mut found = path.clone();
            found.push(token.to_owned());
            paths.push(found);
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

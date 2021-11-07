use std::fs::File;
use std::io::{BufReader, Stderr};
// use std::env::{args, Args};
use std::process::{Command, Output, Stdio};
use std::str::FromStr;

#[derive(Debug)]
pub enum SearchType {
    Complexify,
    Phased,
    Blended,
}

pub struct SearchTypeParseError;

impl FromStr for SearchType {
    type Err = SearchTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "phased" => Ok(SearchType::Phased),
            "blended" => Ok(SearchType::Blended),
            "complexify" => Ok(SearchType::Complexify),
            _ => Err(SearchTypeParseError {}),
        }
    }
}

#[derive(Debug)]
pub enum ExperimentType {
    CfgXsx,
    Maze,
    RegexXyxy,
    RegexAba,
    Seq1bit2el,
    Seq1bit3el,
    Seq1bit4el,
    Seq1bit5el,
    Xor,
}

pub struct ExperimentNotSupported;

impl FromStr for ExperimentType {
    type Err = ExperimentNotSupported;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "xor" => Ok(ExperimentType::Xor),
            "cfg-XSX" => Ok(ExperimentType::CfgXsx),
            "maze" => Ok(ExperimentType::Maze),
            "regex-XYXY" => Ok(ExperimentType::RegexXyxy),
            "regex-aba" => Ok(ExperimentType::RegexAba),
            "seq-1bit-2el" => Ok(ExperimentType::Seq1bit2el),
            "seq-1bit-3el" => Ok(ExperimentType::Seq1bit3el),
            "seq-1bit-4el" => Ok(ExperimentType::Seq1bit4el),
            "seq-1bit-5el" => Ok(ExperimentType::Seq1bit5el),
            _ => Err(ExperimentNotSupported {}),
        }
    }
}

#[derive(Debug)]
pub struct AccNeatArgs {
    pub num_experiments: usize,
    pub force_delete: bool,
    pub rng_seed: usize,
    pub pop_size: usize,
    pub maxgens: usize,
    pub search_type: SearchType,
    pub experiment: String, // TODO: use ExperimentType
}

pub(crate) fn build_cmd(args: &[String]) -> std::io::Result<Output> {
    println!("build_cmd({:?})", &args);
    if cfg!(target_os = "windows") {
        // Command::new(".\\vendor\\accneat\\cmake-build-debug\\accneat.exe")
        Command::new("..\\..\\cmake-build-debug\\accneat.exe")
            .args(args)
            .output()
    } else {
        Command::new("accneat")
            .args(args)
            .stdout(Stdio::piped())
            .output()
    }
}

pub(crate) fn execute_cmd_line(args: &[String]) -> Result<(String, String), std::io::Error> {
    let o = build_cmd(args)?;
    // println!("Got result: {:?}", o);
    // assert!(o.status.success());
    let result = String::from_utf8(o.stdout)
        .unwrap_or("[ERROR] String::from_utf8(o.stdout).unwrap() failed".to_string());
    let errs = String::from_utf8(o.stderr).unwrap_or(String::default());
    // println!("result -- {}", &result);
    // println!("errors -- {}", &errs);
    Ok((result, errs))
}

const DEFAULT_NUM_EXPS: usize = 1;
const DEFAULT_RNG_SEED: usize = 1;
const DEFAULT_POP_SIZE: usize = 1000;
const DEFAULT_MAX_GENS: usize = 10000;
const DEFAULT_SEARCHTYPE: SearchType = SearchType::Phased;
const DEFAULT_FORCE_DELETE: bool = false;

impl Default for AccNeatArgs {
    fn default() -> Self {
        Self {
            num_experiments: DEFAULT_NUM_EXPS,
            force_delete: DEFAULT_FORCE_DELETE,
            rng_seed: DEFAULT_RNG_SEED,
            pop_size: DEFAULT_POP_SIZE,
            maxgens: DEFAULT_MAX_GENS,
            search_type: DEFAULT_SEARCHTYPE,
            experiment: "xor".to_string(),
        }
    }
}

pub fn execute(args: AccNeatArgs) -> Result<(String, String), std::io::Error> {
    let mut a = vec![];
    if args.force_delete {
        a.push("-f".to_string());
    }
    a.push("-c".to_string());
    a.push(args.num_experiments.to_string());
    a.push("-r".to_string());
    a.push(args.rng_seed.to_string());
    a.push("-n".to_string());
    a.push(args.pop_size.to_string());
    a.push("-x".to_string());
    a.push(args.maxgens.to_string());
    a.push("-s".to_string());
    a.push(
        match args.search_type {
            SearchType::Phased => "phased",
            SearchType::Blended => "blended",
            SearchType::Complexify => "complexify",
        }
        .to_string(),
    );
    a.push(args.experiment);

    execute_cmd_line(a.as_slice())
}

#[derive(Debug)]
enum Error {
    IoError(std::io::Error),
    ParseFloatError(std::num::ParseFloatError),
    ParseIntError(std::num::ParseIntError),
}

#[derive(Debug)]
struct OrganismInfo {
    id: usize,
    fitness: f32,
    error: f32,
}

impl Default for OrganismInfo {
    fn default() -> Self {
        Self {
            id: usize::MAX,
            fitness: -1.0,
            error: 100.0,
        }
    }
}

#[derive(Debug)]
struct TraitInfo {

}

#[derive(Debug)]
struct ParsedOrganism {
    info: OrganismInfo,
    traits: Vec<TraitInfo>,
}

fn parse_fittest_file(path: String) -> Result<(OrganismInfo, Vec<Vec<f32>>, Vec<Vec<usize>>, Vec<Vec<f32>>), Error> {
    let raws = std::fs::read_to_string(&path).map_err(Error::IoError)?;
    let reader = BufReader::new(File::open(path).map_err(Error::IoError)?);
    let mut reader = whiteread::Reader::new(reader);
    let mut org = OrganismInfo::default();

    let mut traits = vec![];
    let mut nodes = vec![];
    let mut genes = vec![];
    while let Ok(v) = reader.line::<Vec<String>>() {
        match v.len() {
            8 => {
                org.id = usize::from_str(&v[2][1..]).map_err(Error::ParseIntError)?;
                org.error = f32::from_str(&v[6]).map_err(Error::ParseFloatError)?;
                org.fitness = f32::from_str(&v[4]).map_err(Error::ParseFloatError)?;
                println!("Found Organism (# {}) with fitness, {} and error, {}", org.id, org.fitness, org.error);
            },
            10 => {
                assert_eq!("trait", &v[0]);
                let t = &v[1..];
                let data = t.iter().filter_map(|x| {
                    match f32::from_str(x).map_err(Error::ParseFloatError) {
                        Ok(x) => Some(x),
                        Err(_) => None,
                    }
                }).collect();
                traits.push(data);
            },
            4 => {
                assert_eq!("node", &v[0]);
                let data = (&v[1..]).iter().filter_map(|x|{
                    match usize::from_str(x).map_err(Error::ParseIntError) {
                        Ok(x) => Some(x),
                        Err(_) => None,
                    }
                }).collect();
                nodes.push(data);
            },
            9 => {
                assert_eq!("gene", &v[0]);
                let g = &v[1..];
                let data = g.iter().filter_map(|x| {
                    match f32::from_str(x).map_err(Error::ParseFloatError) {
                        Ok(x) => Some(x),
                        Err(_) => None,
                    }
                }).collect();
                genes.push(data);
            },
            2 => {
                let cmd = &v[0];
                let id = usize::from_str(&v[1]).map_err(Error::ParseIntError)?;
                assert_eq!(id, org.id);
                match cmd.as_str() {
                    "genomestart" => {
                        println!("Found start of Genome");
                    },
                    "genomeend" => {
                        println!("Found end of Genome");
                    },
                    _ => {}
                }
                assert!(cmd == "genomestart" || cmd == "genomeend");
            },
            _ => {unreachable!()}
        }
    }
    Ok((org, traits, nodes, genes))
    /*

    /* Organism #6 Fitness: 0.542682 Error: 1.82927 */
    genomestart 6
    trait 1 0.568373 0.112194 0.400981 0.918777 0.59396 0.239687 0.331848 0
    node 1 1 0
    node 2 1 1
    node 3 1 1
    node 4 1 2
    node 5 1 3
    node 6 1 3
    gene 1 1 5 0.595938 0 1 0.595938 1
    gene 1 1 6 0.442134 0 2 0.442134 1
    gene 1 2 5 -0.440328 0 3 -0.440328 1
    gene 1 2 6 -0.427128 0 4 -0.427128 1
    gene 1 3 5 0.878225 0 5 0.878225 1
    gene 1 3 6 0.60156 0 6 0.60156 1
    gene 1 5 4 0.107134 0 7 0.107134 1
    gene 1 6 4 -0.840043 0 8 -0.840043 1
    genomeend 6

         */
    // todo!();
}

fn find_fittest_files() -> Result<Vec<String>, std::io::Error> {
    let resdirs = find_experiment_result_dirs()?;
    let mut ff = vec![];

    for d in resdirs.iter() {
        // println!("processing {}", d);
        if let Ok(dir) = std::fs::read_dir(d) {
            ff = dir.map(|x| x.unwrap().path().to_str().unwrap().to_string()).collect();
        }
    }
    Ok(ff)
}

fn find_experiment_result_dirs() -> Result<Vec<String>, std::io::Error> {
    let dir = std::fs::read_dir("experiments");
    if let Ok(dir) = dir {
        Ok(dir
            .map(|x| x.unwrap().path().to_str().unwrap().to_string())
            .collect())
    } else {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use crate::{execute, find_experiment_result_dirs, find_fittest_files, AccNeatArgs, parse_fittest_file};
    use serial_test::serial;

    fn exec_default() -> (String, String) {
        let mut a = AccNeatArgs::default();
        a.force_delete = true;
        a.pop_size = 10;
        a.maxgens = 10;
        let r = execute(a);
        assert!(r.is_ok());
        let (r, errstr) = r.unwrap();
        // println!("result: {}", &r);
        // println!("errors: {}", &errstr);
        assert!(!r.is_empty());
        assert!(errstr.is_empty());
        (r, errstr)
    }

    #[test]
    #[serial]
    fn cannot_find_experiment_result_dirs() {
        std::fs::remove_dir_all("experiments").unwrap_or(());
        let f = find_experiment_result_dirs().unwrap();
        assert!(f.is_empty());
    }

    #[test]
    #[serial]
    fn can_execute() {
        let r = exec_default();
        assert!(!r.0.is_empty());
        assert!(r.1.is_empty());
        std::fs::remove_dir_all("experiments").unwrap();
    }

    #[test]
    #[serial]
    fn can_find_fittest_file() {
        let (r, errstr) = exec_default();
        // println!("result: {}", &r);
        // println!("errors: {}", &errstr);
        assert!(!r.is_empty());
        assert!(errstr.is_empty());
        let f = find_fittest_files().unwrap();
        assert!(!f.is_empty());
        println!("{:?}", f);
        std::fs::remove_dir_all("experiments").unwrap();
    }

    #[test]
    #[serial]
    fn can_parse_fittest_file() {
        let (r, errstr) = exec_default();
        let f = find_fittest_files().unwrap();
        println!("{:?}", f);
        assert!(!f.is_empty());
        assert!(f.len() == 1);
        let mut f = f;
        for path in f.drain(..) {
            println!("{}:", &path);
            let f = parse_fittest_file(path);
            println!("{:?}", f.unwrap());
        }

        std::fs::remove_dir_all("experiments").unwrap();
        todo!();
    }
}

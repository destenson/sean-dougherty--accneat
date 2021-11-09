use std::collections::HashSet;
use std::fmt::Formatter;
use std::fs::File;
use std::hash::{Hash, Hasher};
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
    // println!("build_cmd({:?})", &args);
    if cfg!(target_os = "windows") {
        Command::new(".\\vendor\\accneat\\cmake-build-debug\\accneat.exe")
        // Command::new("..\\..\\cmake-build-debug\\accneat.exe")
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

#[derive(Clone, Copy)]
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

impl std::fmt::Display for OrganismInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Organism #{}    Fitness: {:.2}%    Error: {:.4}", self.id, self.fitness*100.0, self.error)
    }
}

#[derive(Clone)]
struct TraitInfo {
    id: usize,
    params: Vec<f32>,
}

impl Eq for TraitInfo {}
impl PartialEq for TraitInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
impl Hash for TraitInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl TraitInfo {
    fn new(a: Vec<f32>) -> Self {
        assert_eq!(a.len(), 9);
        let id = a[0] as usize;
        let params = a[1..].iter().cloned().collect();
        Self { id, params }
    }
}

impl std::fmt::Display for TraitInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let p = &self.params;
        assert_eq!(p.len(), 8);
        writeln!(f, "Trait {}: [{:.4}, {:.4}, {:.4}, {:.4}, {:.4}, {:.4}, {:.4}, {:.4}]",
                 self.id, p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7])
    }
}

#[derive(Debug, Clone, Hash)]
enum NodeType {
    Bias = 0,
    Sensor = 1,
    Output = 2,
    Hidden = 3,
}

#[derive(Clone, Hash)]
struct NodeInfo {
    id: usize,
    trait_id: usize,
    type_: NodeType,
}

impl Eq for NodeInfo {}
impl PartialEq for NodeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id) && self.trait_id.eq(&other.trait_id)
    }
}

impl NodeInfo {
    fn new(a: Vec<usize>) -> Self {
        assert_eq!(a.len(), 3);
        let id = a[0] as usize;
        let trait_id = a[1] as usize;
        let type_ = match a[2] {
            0 => NodeType::Bias,
            1 => NodeType::Sensor,
            2 => NodeType::Output,
            3 => NodeType::Hidden,
            _ => unreachable!(),
        };
        Self { id, trait_id, type_ }
    }
}

impl std::fmt::Display for NodeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}: {2:?}] => {1}", self.id, self.trait_id, self.type_)
    }
}

#[derive(Clone, Copy)]
struct GeneInfo {
    trait_id: usize,
    in_node_id: usize,
    out_node_id: usize,
    weight: f32,
    is_recurrent: bool,
    innovation_num: usize,
    mutation_num: f32,
    enable: bool,
}

impl std::fmt::Display for GeneInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{5}<-[{1}=>{2}] {6} we: {3:.4} mu: {4:.4} {7} ({0})", self.innovation_num,
               self.in_node_id, self.out_node_id, self.weight, self.mutation_num, self.trait_id,
               if self.enable {"e"} else {"d"}, if self.is_recurrent {"rc"} else {""})
    }
}

impl GeneInfo {
    fn new(a: Vec<f32>) -> Self {
        assert_eq!(a.len(), 8);
        Self {
            trait_id: a[0] as usize,
            in_node_id: a[1] as usize,
            out_node_id: a[2] as usize,
            weight: a[3],
            is_recurrent: a[4] > 0.5,
            innovation_num: a[5] as usize,
            mutation_num: a[6],
            enable: a[7] > 0.5,
        }
    }
}

#[derive(Clone)]
struct Nodes(pub HashSet<NodeInfo>);

impl std::fmt::Display for Nodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} nodes: ", self.0.len() )?;
        for n in &self.0 {
            write!(f, "{}; ", n)?;
        }
        writeln!(f, "")
    }
}

#[derive(Clone)]
struct Traits(pub HashSet<TraitInfo>);

impl std::fmt::Display for Traits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} traits:", self.0.len() )?;
        for n in &self.0 {
            write!(f, "  {}", n)?;
        }
        // writeln!(f, "")
        Ok(())
    }
}

#[derive(Clone)]
struct Genes(pub Vec<GeneInfo>);

impl std::fmt::Display for Genes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} genes:", self.0.len() )?;
        for n in &self.0 {
            writeln!(f, "  {}", n)?;
        }
        // writeln!(f, "")
        Ok(())
    }
}

#[derive(Clone)]
struct ParsedOrganism {
    info: OrganismInfo,
    traits: Traits,
    nodes: Nodes,
    genes: Genes,
}

impl ParsedOrganism {
    fn new(info: OrganismInfo, traits: Vec<TraitInfo>, nodes: Vec<NodeInfo>, genes: Vec<GeneInfo>) -> Self {
        let traits = Traits(traits.iter().cloned().collect());
        let nodes = Nodes(nodes.iter().cloned().collect());
        let genes = Genes(genes.iter().cloned().collect());
        Self { info, traits, nodes, genes }
    }
}

impl std::fmt::Display for ParsedOrganism {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}{}", self.info, self.traits)?;
        writeln!(f, "{}{}", self.nodes, self.genes)
    }
}

fn parse_fittest() -> Option<ParsedOrganism> {
    let f = find_fittest_files().unwrap();
    // println!("{:?}", f);
    assert!(!f.is_empty());
    let mut f = f;
    let mut fittest: Option<ParsedOrganism> = None;
    for path in f.drain(..) {
        println!("{}:", &path);
        if let Ok(f) = parse_fittest_file(path) {
            println!("{}", f);
            if (&fittest).is_none() {
                fittest = Some(f);
            } else {
                let replace = if let Some(fittest) = &fittest {
                    fittest.info.fitness < f.info.fitness
                } else {false};
                if replace {
                    fittest = Some(f);
                }
            }
        }
    }
    fittest
}

fn parse_fittest_file(path: String) -> Result<ParsedOrganism, Error> {
    // let raws = std::fs::read_to_string(&path).map_err(Error::IoError)?;
    // println!("{}", raws);
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
                // println!("Found Organism (# {}) with fitness, {} and error, {}", org.id, org.fitness, org.error);
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
                traits.push(TraitInfo::new(data));
            },
            4 => {
                assert_eq!("node", &v[0]);
                let data = (&v[1..]).iter().filter_map(|x|{
                    match usize::from_str(x).map_err(Error::ParseIntError) {
                        Ok(x) => Some(x),
                        Err(_) => None,
                    }
                }).collect();
                nodes.push(NodeInfo::new(data));
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
                genes.push(GeneInfo::new(data));
            },
            2 => {
                let cmd = &v[0];
                let id = usize::from_str(&v[1]).map_err(Error::ParseIntError)?;
                assert_eq!(id, org.id);
                match cmd.as_str() {
                    "genomestart" => {
                        // println!("Found start of Genome");
                    },
                    "genomeend" => {
                        // println!("Found end of Genome");
                    },
                    _ => unreachable!(),
                }
                assert!(cmd == "genomestart" || cmd == "genomeend");
            },
            _ => unreachable!(),
        }
    }
    Ok(ParsedOrganism::new(org, traits, nodes, genes))
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
    use crate::{execute, find_experiment_result_dirs, find_fittest_files, AccNeatArgs, parse_fittest_file, parse_fittest};
    use serial_test::serial;

    fn exec_default() -> (String, String) {
        let mut a = AccNeatArgs::default();
        a.force_delete = true;
        a.pop_size = 1000;
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

    fn exec(pop_size: usize) -> (String, String) {
        let mut a = AccNeatArgs::default();
        a.force_delete = true;
        a.pop_size = pop_size;
        a.rng_seed = (std::time::Instant::now().elapsed().as_nanos() % 0xffffffff) as usize;
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
    fn test_cannot_find_experiment_result_dirs() {
        std::fs::remove_dir_all("experiments").unwrap_or(());
        let f = find_experiment_result_dirs().unwrap();
        assert!(f.is_empty());
    }

    #[test]
    #[serial]
    fn test_execute() {
        let r = exec_default();
        assert!(!r.0.is_empty());
        assert!(r.1.is_empty());
        std::fs::remove_dir_all("experiments").unwrap();
    }

    #[test]
    #[serial]
    fn test_find_fittest_file() {
        let (r, errstr) = exec_default();
        // println!("result: {}", &r);
        // println!("errors: {}", &errstr);
        assert!(!r.is_empty());
        assert!(errstr.is_empty());
        let f = find_fittest_files().unwrap();
        assert!(!f.is_empty());
        // println!("{:?}", f);
        std::fs::remove_dir_all("experiments").unwrap();
    }

    #[test]
    #[serial]
    fn test_parse_fittest_file() {
        let (r, errstr) = exec_default();
        println!("{}", r);
        let f = find_fittest_files().unwrap();
        // println!("{:?}", f);
        assert!(!f.is_empty());
        assert_eq!(f.len(), 1);
        let mut f = f;
        for path in f.drain(..) {
            println!("{}:", &path);
            let f = parse_fittest_file(path);
            println!("{}", f.unwrap());
        }

        std::fs::remove_dir_all("experiments").unwrap();
    }

    #[test]
    #[serial]
    fn test_optimize() {

        let mut fittest = None;
        for pop_size in [12,25,36,50,60,75,90,100,120,180,360,420,540,680,800,1200,1600,3200] {
            let (r, errstr) = exec(pop_size);
            let f = parse_fittest();
            if let Some(o) = f {
                if o.info.fitness > 0.9999 {
                    fittest = Some((pop_size,o));
                    break;
                }
            }
        }
        if let Some((pop_size, fittest)) = fittest {
            println!("With population size: {}, the winner is {}", pop_size, fittest);
        }
    }


}

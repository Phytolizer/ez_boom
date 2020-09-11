use crate::ARGS;

const PARM_RECORD: &str = "-record";
const PARM_RECORDFROM: &str = "-recordfrom";
const PARM_RECORDFROMTO: &str = "-recordfromto";

const PARM_PLAYDEMO: &str = "-playdemo";
const PARM_TIMEDEMO: &str = "-timedemo";
const PARM_FASTDEMO: &str = "-fastdemo";

pub fn check_parm(check: &str) -> Option<usize> {
    let args = crate::ARGS.read();
    let mut i = args.len() - 1;
    loop {
        if args[i] == check {
            return Some(i);
        }
        if i == 0 {
            return None;
        }
        i -= 1;
    }
}

pub(crate) fn check_arg_conflicts() {
    let recording_attempt = check_parm(PARM_RECORD).is_some()
        || check_parm(PARM_RECORDFROM).is_some()
        || check_parm(PARM_RECORDFROMTO).is_some();

    let playback_attempt = check_parm(PARM_PLAYDEMO).is_some()
        || check_parm(PARM_TIMEDEMO).is_some()
        || check_parm(PARM_FASTDEMO).is_some();

    if recording_attempt && playback_attempt {
        crate::error("Parameter conflict: cannot play back and record a demo at the same time.");
    }
}
struct LooseArg {
    exts: Vec<&'static str>,
    list: Vec<String>,
}
impl LooseArg {
    fn new(exts: Vec<&'static str>) -> Self {
        Self { exts, list: vec![] }
    }
    fn empty(&self) -> bool {
        self.list.is_empty()
    }
}

/**
Handle loose file arguments by prepending an appropriate flag.

For example, `ez_boom sunlust.wad sl01.lmp` will be changed to
`ez_boom -file sunlust.wad -playdemo sl01.lmp` by this function.

Importantly, this function skips any loose file args that occur
after switches (args starting with '-').
**/
pub(crate) fn handle_loose_files() {
    let mut loose_lmps = LooseArg::new(vec![".lmp"]);
    let mut loose_dehs = LooseArg::new(vec![".deh", ".bex"]);
    let mut loose_wads = LooseArg::new(vec![".wad", ""]);
    let mut loose_args = [&mut loose_lmps, &mut loose_dehs, &mut loose_wads];

    let mut skip = vec![false; ARGS.read().len()];
    for (i, arg) in ARGS.read().iter().enumerate().skip(1) {
        if arg.starts_with('-') {
            // quit at first switch
            break;
        }
        if let Some(loose) = loose_args
            .iter_mut()
            .find(|loose| loose.exts.iter().any(|ext| arg.ends_with(ext)))
        {
            loose.list.push(arg.clone());
        }
        skip[i] = true;
    }

    if loose_args.iter().all(|loose| loose.empty()) {
        // no loose files, yay!
        return;
    }

    let mut temp_args = Vec::<String>::new();
    temp_args.push(ARGS.read()[0].clone());

    let params = [
        ("-file", &loose_wads),
        ("-deh", &loose_dehs),
        ("-playdemo", &loose_lmps),
    ];
    for param in &params {
        if !param.1.list.is_empty() {
            temp_args.push(param.0.to_string());
            for arg in &param.1.list {
                temp_args.push(arg.to_string());
            }
        }
    }

    for (i, arg) in ARGS.read().iter().enumerate().skip(1) {
        if !skip[i] {
            temp_args.push(arg.to_string());
        }
    }

    *ARGS.write() = temp_args;
}

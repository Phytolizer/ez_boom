const PARM_RECORD: &str = "-record";
const PARM_RECORDFROM: &str = "-recordfrom";
const PARM_RECORDFROMTO: &str = "-recordfromto";

const PARM_PLAYDEMO: &str = "-playdemo";
const PARM_TIMEDEMO: &str = "-timedemo";
const PARM_FASTDEMO: &str = "-fastdemo";

/**
Information about which args were passed on the command line.
Related to but not identical to RuntimeConfig.
**/
#[derive(Debug)]
pub(crate) struct ArgMeta {
    pub(crate) nomonsters: bool,
    pub(crate) respawnparm: bool,
    pub(crate) fastparm: bool,
}

impl Default for ArgMeta {
    fn default() -> Self {
        Self {
            nomonsters: false,
            respawnparm: false,
            fastparm: false,
        }
    }
}

pub(crate) type Args = Vec<String>;

pub(crate) trait ArgList {
    /// Check for the existence of `check`. If it exists, this returns
    /// `Some(i)`, where `i` is the index into `self` where the arg is
    /// located. If the arg is not found, it returns `None`.
    fn check_parm(&self, check: &str) -> Option<usize>;
    /// Check for the existence of any parameter in the `checks` slice.
    /// This is used for parameter aliases.
    ///
    /// The return value is identical to `check_parm`'s.
    fn check_parms(&self, checks: &[&str]) -> Option<usize>;
    /// Check for conflicting parameters and error out if some are found.
    ///
    /// Right now, this only applies to `-playdemo`, `-timedemo`, `-fastdemo`,
    /// `-record`, `-recordfrom`, and `-recordfromto`.
    fn check_arg_conflicts(&self);
    /// Handle loose file arguments by prepending an appropriate flag.
    ///
    /// For example, `ez_boom sunlust.wad sl01.lmp` will be changed to
    /// `ez_boom -file sunlust.wad -playdemo sl01.lmp` by this function.
    ///
    /// Importantly, this function skips any loose file args that occur
    /// after switches (args starting with '-').
    fn handle_loose_files(&mut self);
}

impl ArgList for Args {
    fn check_parm(&self, check: &str) -> Option<usize> {
        for (i, arg) in self.iter().enumerate().rev() {
            if arg == check {
                return Some(i);
            }
        }
        // let mut i = self.len() - 1;
        // loop {
        //     if self[i] == check {
        //         return Some(i);
        //     }
        //     if i == 0 {
        //         return None;
        //     }
        //     i -= 1;
        // }
        None
    }
    fn check_parms(&self, checks: &[&str]) -> Option<usize> {
        for check in checks {
            if let Some(i) = self.check_parm(check) {
                return Some(i);
            }
        }
        None
    }
    fn check_arg_conflicts(&self) {
        let recording_attempt = self.check_parm(PARM_RECORD).is_some()
            || self.check_parm(PARM_RECORDFROM).is_some()
            || self.check_parm(PARM_RECORDFROMTO).is_some();

        let playback_attempt = self.check_parm(PARM_PLAYDEMO).is_some()
            || self.check_parm(PARM_TIMEDEMO).is_some()
            || self.check_parm(PARM_FASTDEMO).is_some();

        if recording_attempt && playback_attempt {
            crate::error(
                "Parameter conflict: cannot play back and record a demo at the same time.",
            );
        }
    }
    fn handle_loose_files(&mut self) {
        let mut loose_lmps = LooseArg::new(vec![".lmp"]);
        let mut loose_dehs = LooseArg::new(vec![".deh", ".bex"]);
        let mut loose_wads = LooseArg::new(vec![".wad", ""]);
        let mut loose_args = [&mut loose_lmps, &mut loose_dehs, &mut loose_wads];

        let mut skip = vec![false; self.len()];
        for (i, arg) in self.iter().enumerate().skip(1) {
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
        temp_args.push(self[0].clone());

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

        for (i, arg) in self.iter().enumerate().skip(1) {
            if !skip[i] {
                temp_args.push(arg.to_string());
            }
        }

        self.clear();
        self.append(&mut temp_args);
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

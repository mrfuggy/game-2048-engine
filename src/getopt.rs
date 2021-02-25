/* getopt.rs -- parse command line.
Copyright (C) 2020-2021 fuggy

This file is part of game-2048-engine.

game-2048-engine is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

game-2048-engine is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with game-2048-engine.  If not, see <https://www.gnu.org/licenses/>.
*/

#![allow(dead_code)]
use std::collections::HashMap;

struct LongOption<'a> {
    short: char,
    long: &'a str,
    desc: &'a str,
    hint: &'a str,
    has_arg: HasArg,
    occur: Occur,
}

enum HasArg {
    No,
    Required,
    Optional,
}

enum Occur {
    Rec,
    Option,
    Multi,
}

impl<'a> LongOption<'a> {
    fn opt(
        short: char,
        long: &'a str,
        desc: &'a str,
        hint: &'a str,
        has_arg: HasArg,
        occur: Occur,
    ) -> LongOption<'a> {
        LongOption {
            short,
            long,
            desc,
            hint,
            has_arg,
            occur,
        }
    }

    fn opt_flag(short: char, long: &'a str, desc: &'a str) -> LongOption<'a> {
        LongOption::opt(short, long, desc, "", HasArg::No, Occur::Option)
    }

    fn opt_flagreq(short: char, long: &'a str, desc: &'a str) -> LongOption<'a> {
        LongOption::opt(short, long, desc, "", HasArg::No, Occur::Rec)
    }

    fn opt_flagmulti(short: char, long: &'a str, desc: &'a str) -> LongOption<'a> {
        LongOption::opt(short, long, desc, "", HasArg::No, Occur::Multi)
    }

    fn opt_long(short: char, long: &'a str, desc: &'a str, hint: &'a str) -> LongOption<'a> {
        LongOption::opt(short, long, desc, hint, HasArg::Required, Occur::Option)
    }

    fn opt_longopt(short: char, long: &'a str, desc: &'a str, hint: &'a str) -> LongOption<'a> {
        LongOption::opt(short, long, desc, hint, HasArg::Optional, Occur::Option)
    }

    fn opt_only_long(long: &'a str, desc: &'a str, hint: &'a str) -> LongOption<'a> {
        LongOption::opt('\0', long, desc, hint, HasArg::Required, Occur::Option)
    }

    fn opt_only_longopt(long: &'a str, desc: &'a str, hint: &'a str) -> LongOption<'a> {
        LongOption::opt('\0', long, desc, hint, HasArg::Optional, Occur::Option)
    }
}

#[derive(PartialEq, Debug)]
pub enum Match {
    /// valid option optional or required argument
    Opt { short: char, optarg: Option<String> },
    /// gnu-getopt ':' result if option string start with :
    MissingArg { short: char },
    /// gnu-getopt '?' not listed or missing short=optopt
    Unknown { short: char },
    /// rest non options of args string
    NonOption { nonopt: String },
}

/// fsm parser state
enum OptFsmState {
    Start,
    FindOpt,
    Opt,
    Arg,
    NonOpt,
    Rest,
    End,
}

pub struct OptionParser {
    state: OptFsmState,
    opterr: bool,
    args: Vec<Vec<char>>,
    opt_array: Vec<char>,
    opt_map: HashMap<char, usize>,
    args_index: usize,
    str_index: usize,
    optional_arg: bool,
    optopt: char,
}

impl OptionParser {
    fn is_required_arg(&self, opt_i: &usize) -> bool {
        self.opt_array.get(opt_i + 1) == Some(&':')
    }

    fn is_optional_arg(&self, opt_i: &usize) -> bool {
        self.opt_array.get(opt_i + 2) == Some(&':')
    }

    fn print_err_req(&self) {
        eprintln!(
            "{}: option requires an argument -- '{}'",
            self.args[0].iter().collect::<String>(),
            self.optopt
        );
    }
}

/// parser getopt gnu-like
impl Iterator for OptionParser {
    type Item = Match;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &self.state {
                OptFsmState::Start => {
                    self.args_index += 1;
                    self.str_index = 0;
                    let str = self.args.get(self.args_index);
                    self.state = if str == None {
                        OptFsmState::End
                    } else {
                        OptFsmState::FindOpt
                    };
                }

                OptFsmState::FindOpt => {
                    let ch = self.args[self.args_index].get(self.str_index);
                    self.state = match ch {
                        Some('-') if self.args[self.args_index].len() == 1 => OptFsmState::NonOpt,
                        Some('-') => {
                            self.str_index += 1;
                            OptFsmState::Opt
                        }
                        _ => OptFsmState::NonOpt,
                    };
                }

                OptFsmState::Opt => {
                    let ch = self.args[self.args_index].get(self.str_index);
                    match ch {
                        // just "--" opt
                        Some('-') if self.args[self.args_index].len() == 2 => {
                            self.state = OptFsmState::Rest;
                        }

                        // special case "--x"
                        Some('-') if self.str_index == 1 => {
                            self.str_index += 1;
                            return Some(Match::Unknown { short: '-' });
                        }

                        Some(x) => {
                            self.str_index += 1;
                            let contains = self.opt_map.get(&x);
                            match contains {
                                Some(opt_i) if self.is_required_arg(opt_i) => {
                                    self.optopt = *x;
                                    self.optional_arg = self.is_optional_arg(opt_i); //gnu extension '::'
                                    self.state = OptFsmState::Arg;
                                }
                                Some(_) => {
                                    return Some(Match::Opt {
                                        short: *x,
                                        optarg: None,
                                    });
                                }
                                None => {
                                    self.state = OptFsmState::Start;
                                    return Some(Match::Unknown { short: *x });
                                }
                            }
                        }

                        None => {
                            self.state = OptFsmState::Start;
                        }
                    }
                }

                OptFsmState::Arg => {
                    let try_str = self.args.get(self.args_index);
                    match try_str {
                        None => {
                            self.state = OptFsmState::Start;
                            let res = if self.optional_arg {
                                Match::Opt {
                                    short: self.optopt,
                                    optarg: None,
                                }
                            } else {
                                if self.opterr {
                                    self.print_err_req();
                                }

                                if self.opt_array[0] == ':' {
                                    Match::MissingArg { short: self.optopt }
                                } else {
                                    Match::Opt {
                                        short: self.optopt,
                                        optarg: None,
                                    }
                                }
                            };
                            return Some(res);
                        }

                        Some(str) => {
                            let ch = str.get(self.str_index);
                            match ch {
                                Some(_) => {
                                    self.state = OptFsmState::Start;
                                    return Some(Match::Opt {
                                        short: self.optopt,
                                        optarg: Some(
                                            str[self.str_index..str.len()].iter().collect(),
                                        ),
                                    });
                                }
                                None => {
                                    self.args_index += 1;
                                    self.str_index = 0;
                                }
                            }
                        }
                    }
                }

                OptFsmState::NonOpt => {
                    self.state = OptFsmState::Start;
                    return Some(Match::NonOption {
                        nonopt: self.args[self.args_index].iter().collect(),
                    });
                }

                OptFsmState::Rest => {
                    self.args_index += 1;
                    self.str_index = 0;
                    let has_next = self.args.get(self.args_index);
                    match has_next {
                        Some(str) => {
                            return Some(Match::NonOption {
                                nonopt: str.iter().collect(),
                            });
                        }
                        None => self.state = OptFsmState::End,
                    }
                }

                OptFsmState::End => return None,
            };
        }
    }
}

/// gnu getopt for rust
/// The options argument is a string that specifies the option characters that are valid for this program.
/// If the first character of options is a colon (‘:’), then getopt returns ‘Matchresult::MissingArg’ instead of ‘Matchresult::Opt’ to indicate a missing option argument.
/// In addition, if the variable opterr is true, getopt prints an error message.
/// An option character in this string can be followed by a colon (‘:’) to indicate that it takes a required argument.
/// If an option character is followed by two colons (‘::’), its argument is optional; this is a GNU extension.
pub fn getopt(args: Vec<String>, options: &str, opterr: bool) -> OptionParser {
    let args_array: Vec<Vec<char>> = args.into_iter().map(|x| x.chars().collect()).collect();
    let opt_array: Vec<char> = options.chars().collect();
    let mut opt_map: HashMap<char, usize> = HashMap::new();
    for (i, ch) in options.char_indices() {
        if ch.is_alphanumeric() {
            opt_map.insert(ch, i);
        }
    }
    OptionParser {
        state: OptFsmState::Start,
        opterr,
        args: args_array,
        opt_array,
        opt_map,
        args_index: 0,
        str_index: 0,
        optopt: '\0',
        optional_arg: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_getopt1() {
        let args: Vec<String> = vec!["./main", "-abcd"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let res: Vec<Match> = getopt(args, "abc:", false).collect();
        assert_eq!(
            res[0],
            Match::Opt {
                short: 'a',
                optarg: None
            }
        );
        assert_eq!(
            res[1],
            Match::Opt {
                short: 'b',
                optarg: None
            }
        );
        assert_eq!(
            res[2],
            Match::Opt {
                short: 'c',
                optarg: Some("d".to_string())
            }
        );
    }

    #[test]
    fn should_getopt2() {
        let args: Vec<String> = vec!["./main", "-c"].iter().map(|x| x.to_string()).collect();
        let res: Vec<Match> = getopt(args, ":abc:", true).collect();
        assert_eq!(res[0], Match::MissingArg { short: 'c' });
    }

    #[test]
    fn should_getopt3() {
        let args: Vec<String> = vec!["./main", "-c"].iter().map(|x| x.to_string()).collect();
        let res: Vec<Match> = getopt(args, ":abc::", true).collect();
        assert_eq!(
            res[0],
            Match::Opt {
                short: 'c',
                optarg: None
            }
        );
    }

    #[test]
    fn should_getopt4() {
        let args: Vec<String> = vec!["./main", "-a", "--", "-b"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let res: Vec<Match> = getopt(args, ":abc::", true).collect();
        assert_eq!(
            res[0],
            Match::Opt {
                short: 'a',
                optarg: None
            }
        );
        assert_eq!(
            res[1],
            Match::NonOption {
                nonopt: "-b".to_string()
            }
        );
    }
}

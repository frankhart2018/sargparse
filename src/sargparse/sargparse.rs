use std::collections::HashMap;

#[derive(Debug)]
pub enum ArgumentType {
    INT,
    FLOAT,
    STR,
    BOOL,
}

#[derive(Debug)]
pub enum InnerData {
    INT(i32),
    FLOAT(f32),
    STR(String),
    BOOL(bool),
}

impl InnerData {
    pub fn get_int(&self) -> i32 {
        match self {
            &InnerData::INT(ref i) => *i,
            _ => panic!("InnerData::get_int() called on non-INT data"),
        }
    }

    pub fn get_float(&self) -> f32 {
        match self {
            &InnerData::FLOAT(ref f) => *f,
            _ => panic!("InnerData::get_float() called on non-FLOAT data"),
        }
    }

    pub fn get_str(&self) -> String {
        match self {
            &InnerData::STR(ref s) => s.clone(),
            _ => panic!("InnerData::get_str() called on non-STR data"),
        }
    }

    pub fn get_bool(&self) -> bool {
        match self {
            &InnerData::BOOL(ref b) => *b,
            _ => panic!("InnerData::get_bool() called on non-BOOL data"),
        }
    }

    fn from_int(i: i32) -> InnerData {
        InnerData::INT(i)
    }

    fn from_float(f: f32) -> InnerData {
        InnerData::FLOAT(f)
    }

    fn from_str(s: String) -> InnerData {
        InnerData::STR(s)
    }

    fn from_bool(b: bool) -> InnerData {
        InnerData::BOOL(b)
    }
}

#[derive(Debug)]
struct Argument {
    short_name: String,
    long_name: String,
    help: String,
    default: Option<InnerData>,
    dtype: ArgumentType,
}

#[derive(Debug)]
pub struct ArgumentParser {
    description: String,
    required_args: Vec<Argument>,
    optional_args: Vec<Argument>,
}

impl ArgumentParser {
    pub fn new(description: Option<&str>) -> ArgumentParser {
        let description = match description {
            Some(d) => d.to_string(),
            None => String::new(),
        };

        ArgumentParser {
            description,
            required_args: Vec::new(),
            optional_args: Vec::new(),
        }
    }

    pub fn add_argument(&mut self, short_name: &str, long_name: &str, help: &str, 
                        required: bool, default: Option<InnerData>, dtype: ArgumentType) {
        let argument = Argument {
            short_name: short_name.to_string(),
            long_name: long_name.to_string(),
            help: help.to_string(),
            default: default,
            dtype: dtype,
        };

        if required {
            self.required_args.push(argument);
        } else {
            self.optional_args.push(argument);
        }
    }

    fn push_parsed_args(&self, arg: &Argument, intermediate_parsed_args: &HashMap<String, String>,
                        parsed_args: &mut HashMap<String, InnerData>, search_param: &str) {
        match arg.dtype {
            ArgumentType::INT => {
                let value = intermediate_parsed_args.get(search_param);

                match value {
                    Some(v) => {
                        let v = v.parse::<i32>().unwrap();
                        parsed_args.insert(arg.long_name.clone(), InnerData::from_int(v));
                    },
                    None => {
                        match &arg.default {
                            Some(d) => {
                                let d = d.get_int();
                                parsed_args.insert(arg.long_name.clone(), InnerData::from_int(d));
                            },
                            None => {
                                panic!("Missing required argument: {}", arg.long_name);
                            },
                        }
                    },
                }
            },
            ArgumentType::FLOAT => {
                let value = intermediate_parsed_args.get(search_param);

                match value {
                    Some(v) => {
                        let v = v.parse::<f32>().unwrap();
                        parsed_args.insert(arg.long_name.clone(), InnerData::from_float(v));
                    },
                    None => {
                        match &arg.default {
                            Some(d) => {
                                let d = d.get_float();
                                parsed_args.insert(arg.long_name.clone(), InnerData::from_float(d));
                            },
                            None => {
                                panic!("Missing required argument: {}", arg.long_name);
                            },
                        }
                    },
                }
            },
            ArgumentType::STR => {
                let value = intermediate_parsed_args.get(search_param);

                match value {
                    Some(v) => {
                        let v = v.clone();
                        parsed_args.insert(arg.long_name.clone(), InnerData::from_str(v));
                    },
                    None => {
                        match &arg.default {
                            Some(d) => {
                                let d = d.get_str();
                                parsed_args.insert(arg.long_name.clone(), InnerData::from_str(d));
                            },
                            None => {
                                panic!("Missing required argument: {}", arg.long_name);
                            },
                        }
                    },
                }
            },
            ArgumentType::BOOL => {
                let value = intermediate_parsed_args.get(search_param);

                match value {
                    Some(v) => {
                        let v = v.parse::<bool>().unwrap();
                        parsed_args.insert(arg.long_name.clone(), InnerData::from_bool(v));
                    },
                    None => {
                        match &arg.default {
                            Some(d) => {
                                let d = d.get_bool();
                                parsed_args.insert(arg.long_name.clone(), InnerData::from_bool(d));
                            },
                            None => {
                                panic!("Missing required argument: {}", arg.long_name);
                            },
                        }
                    },
                }
            },
        }
    }

    fn help(&self) {
        println!("--------------------------------------------------");
        println!("{}", self.description);
        println!("\n");
        println!("Required arguments:");
        for arg in &self.required_args {
            println!("\t--{} (-{}, {:?}): {}", arg.long_name, arg.short_name, arg.dtype, arg.help);
        }
        println!("\n");
        println!("Optional arguments:");
        for arg in &self.optional_args {
            println!("\t--{} (-{}, {:?}): {}", arg.long_name, arg.short_name, arg.dtype, arg.help);
        }
    }

    pub fn parse_args(&self) -> Option<HashMap<String, InnerData>> {
        let args = std::env::args().collect::<Vec<String>>();

        let mut parsed_args: HashMap<String, InnerData> = HashMap::new();
        let mut intermediate_parsed_args: HashMap<String, String> = HashMap::new();

        let mut i = 1;
        while i < args.len() {
            if i < args.len() && args[i].starts_with("-") || args[i].starts_with("--") {
                let arg_option = args[i].clone().replace("-", "");
                i += 1;

                if i < args.len() && (args[i].starts_with("-") || args[i].starts_with("--")) {
                    intermediate_parsed_args.insert(arg_option, "true".to_string());
                    continue;
                } else if i >= args.len() {
                    intermediate_parsed_args.insert(arg_option, "true".to_string());
                    break;
                }


                let arg_value = args[i].clone();

                intermediate_parsed_args.insert(arg_option, arg_value);
                i += 1;
            } else {
                panic!("Error parsing arguments!");
            }
        }

        if intermediate_parsed_args.contains_key("h") || intermediate_parsed_args.contains_key("help") {
            self.help();
            return None;
        }

        for arg in self.required_args.iter() {
            if intermediate_parsed_args.contains_key(&arg.short_name) {
                self.push_parsed_args(arg, &intermediate_parsed_args, &mut parsed_args, &arg.short_name);
            } else if intermediate_parsed_args.contains_key(&arg.long_name) {
                self.push_parsed_args(arg, &intermediate_parsed_args, &mut parsed_args, &arg.long_name);
            } else {
                panic!("Missing required argument: {}", arg.short_name);
            }
        }

        for arg in self.optional_args.iter() {
            if intermediate_parsed_args.contains_key(&arg.short_name) {
                self.push_parsed_args(arg, &intermediate_parsed_args, &mut parsed_args, &arg.short_name);
            } else if intermediate_parsed_args.contains_key(&arg.long_name) {
                self.push_parsed_args(arg, &intermediate_parsed_args, &mut parsed_args, &arg.long_name);
            }
        }

        Some(parsed_args)
    }
}
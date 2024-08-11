use std::fs;

#[derive(knuffel::Decode, Debug)]
pub struct FlagsConfig {
    #[knuffel(child, unwrap(argument))]
    pub rotation_delay_seconds: i64,
    #[knuffel(child, unwrap(argument))]
    pub show_flag_name: bool,
    #[knuffel(child, unwrap(argument))]
    pub show_color_names: bool,
    #[knuffel(children(name="flag"))]
    pub flags: Vec<PrideFlag>,
}

#[derive(knuffel::Decode, Clone, Debug)]
pub struct FlagColor {
    #[knuffel(argument)]
    pub name: String,
    #[knuffel(argument)]
    pub r: u8,
    #[knuffel(argument)]
    pub g: u8,
    #[knuffel(argument)]
    pub b: u8,
}

#[derive(knuffel::Decode, Clone, Debug)]
pub struct PrideFlag {
    #[knuffel(argument)]
    pub name: String,
    #[knuffel(children)]
    pub colors: Vec<FlagColor>,
}

pub fn parse_flags() -> FlagsConfig {
    let filepath = "flags.kdl";

    let contents = fs::read_to_string(filepath).unwrap();

    knuffel::parse(filepath, &contents).unwrap()
}


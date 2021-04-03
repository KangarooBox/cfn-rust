#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

#[macro_use]
extern crate env_logger;
extern crate handlebars;

extern crate log;
extern crate serde_derive;
extern crate serde_json;

mod helpers;

use serde_json::json;
use serde_json::value::{Map, Value as Json};
use std::error::Error;

use log::{debug, info, warn};
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};

use handlebars::{
    to_json, Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext,
    RenderError,
};

// Process a directory of partials and load them into the Handlebars context
fn register_partials(hbs: &mut Handlebars<'_>, filepath: &str) -> Result<(), Box<dyn Error>> {
    debug!("\tfrom '{}'", filepath);

    for entry in fs::read_dir(filepath)? {
        let dir = entry?;
        let contents = fs::read_to_string(dir.path()).unwrap();
        let name = dir.file_name().into_string().unwrap();

        hbs.register_partial(&name, &contents)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut hbs = Handlebars::new();
    hbs.set_dev_mode(true);

    helpers::register(&mut hbs);

    debug!("loading partials...");
    // register_partials(&hbs, "../blueprints/common/partials")?;
    register_partials(&mut hbs, "./blueprints/partials")?;

    // registerPartials(__dirname + "/../blueprints/common/partials");
    // registerPartials(process.cwd() + "/../partials");
    // registerPartials(process.cwd() + "/blueprints/partials");

    info!("loading globals");
    hbs.register_template_file("globals", "./blueprints/common/globals.hbs")?;
    hbs.register_template_file("test", "./blueprints/stacks/test.hbs")?;

    let mut blah = hbs.get_template("test");
    info!("{:?}", blah);

    let mut output_file = File::create("artifacts/cloudformation/test.template")?;
    let globals_str = hbs.render("globals", &Json::Null)?;
    let globals: Json = serde_json::from_str(&globals_str)?;

    hbs.render_to_write("test", &globals, &mut output_file)?;

    Ok(())
}

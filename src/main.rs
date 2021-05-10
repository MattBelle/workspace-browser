mod event;
mod tui;
mod workspace;
use crate::{
    tui::run,
    workspace::{Workspace, WorkspaceFactory},
};
use clap::{App, Arg};
use std::{env, error::Error, fs::File, io::Write};

fn generate_script(workspace: Workspace, mut file: File) {
    write!(
        file,
        r#"
    #!/usr/bin/env bash

    export WORKSPACE={}
    cd $WORKSPACE
    if [[ -f $WORKSPACE/workspace_env ]]; then
        source $WORKSPACE/workspace_env
    fi
    "#,
        workspace
            .path()
            .clone()
            .into_os_string()
            .into_string()
            .expect("workspace path")
    )
    .expect("generated script");
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Load Workspace")
        .version("1.0")
        .author("Matt B. mattbelle17@gmail.com")
        .about("Generates a script to load the selected workspace.")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Specify the output file location")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    let file_name = matches.value_of("output").expect("output option");

    match env::var("WORKSPACES") {
        Ok(workspace_loc) => {
            let factory = WorkspaceFactory::new(workspace_loc);
            let workspaces = factory.get_workspaces()?;
            match run(workspaces) {
                Ok(success) => match success {
                    Some(workspace) => {
                        generate_script(workspace, File::create(file_name)?);
                    }
                    None => (),
                },
                Err(err) => {
                    println!("Error running the tui: {}", err);
                }
            }
            Ok(())
        }
        Err(err) => {
            println!(
                "Error accessing environmental variable `WORKSPACES`: {}",
                err
            );
            Err(Box::new(err))
        }
    }
}

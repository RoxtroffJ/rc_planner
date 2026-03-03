// Test definition of NACA airfoils, analysis creation and saving project.

use std::path::Path;

use crate::api::{
    Project,
    foil::{create_analysis, make_naca_foil}, globals::save_fl5_project,
};

#[test]
fn main() {
    // Project 1
    {
        println!("Creating project 1");
        let mut project = Project::new().unwrap();

        // Create foils
        println!("Creating foil1");
        let foil1_name = "NACA 0012";
        let _foil1 = make_naca_foil(&mut project, 0012, foil1_name).unwrap();

        println!("Creating foil2");
        let foil2_name = "NACA 2412";
        let _foil2 = make_naca_foil(&mut project, 2412, foil2_name).unwrap();

        // Check that another project cannot be created while one exists
        println!("Checking that another project cannot be created while one exists");
        assert!(Project::new().is_err());

        // Create analysis for foils
        println!("Creating analysis for foil1");
        let _polar1 = create_analysis(&mut project, foil1_name).unwrap();
        println!("Creating analysis for foil2");
        let _polar2 = create_analysis(&mut project, foil2_name).unwrap();

        // Save project
        println!("Saving project");
        assert!(save_fl5_project(&project, Path::new("project1_test.fl5")));
    }

    // Check that a new project can be created after the first one is dropped
    {
        println!("Checking that we can now create project 2");
        let _project2 = Project::new().unwrap();
    }
}
//! Generic functions for foil creation and polar association.

use crate::api::{Foil, Polar, Project, PtrToRef};
use cxx::let_cxx_string;
use std::path::Path;

#[cxx::bridge(namespace = "foil")]
mod ffi {
    unsafe extern "C++" {
        include!("wrapper.h");

        #[namespace = ""]
        type Foil = crate::api::ffi::Foil;
        #[namespace = ""]
        type Polar = crate::api::ffi::Polar;

        /// Loads a foil from a file and stores the pointer to the created object in the array.
        ///
        /// Returns a pointer to the foil instance if successfully loaded and stored, nullptr otherwise.
        fn loadFoil(pathname: &CxxString) -> *mut Foil;

        /// Makes and stores in the database a NACA 4 or digits airfoil.
        fn makeNacaFoil(digits: i32, name: &CxxString) -> *mut Foil;

        /// Returns a pointer to the foil object with the given name, or a nullptr if none is found.
        fn foil(name: &CxxString) -> *mut Foil;

        /// Creates a generic 2d analysis, i.e. a polar, and associates it to the foil.
        /// The analysis data can then be set by accessing the polar's class public methods.
        ///
        /// Returns a pointer to the polar if creation was successful, nullptr otherwise.
        fn createAnalysis(foilname: &CxxString) -> *mut Polar;

        /// Reads an xml file containg the description of a 2d analysis.
        /// If sucessful, creates a [Polar], stores it in the database and retuns a pointer to the object if successfully created, nullptr otherwise.
        fn importAnalysisFromXml(pathname: &CxxString) -> *mut Polar;
    }
}

/// Loads a foil from a file and adds it to the project.
///
/// Returns a reference to the foil instance if successfully loaded and stored.
pub fn load_foil<'a>(project: &'a mut Project, pathname: &Path) -> Option<&'a mut Foil> {
    let_cxx_string!(str = pathname.as_os_str().as_encoded_bytes());

    let ptr = ffi::loadFoil(&str);

    Foil::ptr_to_mut_ref(project, ptr)
}

/// Makes and stores in the database a NACA 4 or digits airfoil.
///
/// Can fail if the digits are not valid.
pub fn make_naca_foil(
    project: &mut Project,
    digits: i32,
    name: impl AsRef<[u8]>,
) -> Option<&mut Foil> {
    let_cxx_string!(str = name);

    let ptr = ffi::makeNacaFoil(digits, &str);

    Foil::ptr_to_mut_ref(project, ptr)
}

/// Returns a reference to the foil object with the given name.
pub fn foil(project: &Project, name: impl AsRef<[u8]>) -> Option<&Foil> {
    let_cxx_string!(str = name);

    let ptr = ffi::foil(&str);

    Foil::ptr_to_ref(project, ptr)
}

/// Returns a mutable reference to the foil object with the given name.
pub fn foil_mut(project: &mut Project, name: impl AsRef<[u8]>) -> Option<&mut Foil> {
    let_cxx_string!(str = name);

    let ptr = ffi::foil(&str);

    Foil::ptr_to_mut_ref(project, ptr)
}

/// Creates a generic 2d analysis, i.e. a [Polar], and associates it to the [Foil].
/// The analysis data can then be set by accessing the [Polar]'s methods.
pub fn create_analysis(project: &mut Project, foilname: impl AsRef<[u8]>) -> Option<&mut Polar> {
    let_cxx_string!(str = foilname);

    let ptr = ffi::createAnalysis(&str);

    Polar::ptr_to_mut_ref(project, ptr)
}

/// Reads an xml file containg the description of a 2d analysis.
/// If sucessful, creates a [Polar], stores it in the database and retuns a reference to the object if successfully created.
pub fn import_analysis_from_xml<'a>(project: &'a mut Project, pathname: &Path) -> Option<&'a mut Polar> {
    let_cxx_string!(str = pathname.as_os_str().as_encoded_bytes());

    let ptr = ffi::importAnalysisFromXml(&str);

    Polar::ptr_to_mut_ref(project, ptr)
}

#[cfg(test)]
mod tests;
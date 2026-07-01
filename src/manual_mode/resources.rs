use std::collections::HashSet;
use bevy::prelude::*;
use std::fmt;
use common_game::components::resource::{BasicResourceType, ComplexResourceType};
use common_game::utils::ID;

// ---------------------------------------------------------------
//      ManualModePanel
// ---------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Galaxy,
    Explorer1,
    Explorer2,
}

#[derive(Resource)]
pub struct ManualModePanel {
    /// defines if the panel has to be visible or not
    pub visible: bool,
    /// define the current selected tab
    pub active_tab: Tab,
}

impl Default for ManualModePanel {
    fn default() -> Self {
        ManualModePanel { visible:true, active_tab: Tab::Galaxy } // The application starts in manual mode -> visibile: true
    }
}

// ---------------------------------------------------------------
//      Spinners
// ---------------------------------------------------------------
#[derive(Resource, Default)]
pub struct PlanetSpinner {
    value: usize,
}

impl PlanetSpinner {
    pub fn increase(&mut self) {
        self.value = if self.value >= 6 { 0 } else { self.value + 1 };
    }
    pub fn decrease(&mut self) {
        self.value = if self.value == 0 { 6 } else { self.value - 1 };
    }
    /// Returns the actual planet index 0-6
    pub fn get_current_value(&self) -> usize {
        self.value
    }
}

impl fmt::Display for PlanetSpinner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value + 1)
    }
}

/// This Resource contains the generatable resource of each planet
/// When 'SendPlanetGenerate' message is received, the resource is updated
#[derive(Resource)]
pub struct GeneratableResourcesOnPlanet {
    planet: Vec<GenerateResourceSpinner>,
}

impl Default for GeneratableResourcesOnPlanet {
    fn default() -> Self {
        let mut spinners = Vec::new();
        for _ in 0..7 {
            let spinner = GenerateResourceSpinner::new_empty();
            spinners.push(spinner);
        }
        assert_eq!(spinners.len(), 7);
        Self {
            planet: spinners,
        }
    }
}

impl GeneratableResourcesOnPlanet {
    /// Returns the corresponding GenerateResourceSpinner of the planet indexed 0-6 
    pub fn get_generate(&self, id: ID) -> &GenerateResourceSpinner {
        match id as usize {
            0..7 => {
                &self.planet[id as usize]
            }
            _ => panic!("GeneratableResourcesOnPlanet::get_generate called with invalid id"),
        }
    }
    /// Returns a mutable reference to the corresponding GenerateResourceSpinner of the planet indexed 0-6
    pub fn get_generate_mut(&mut self, id: ID) -> &mut GenerateResourceSpinner {
        match id as usize {
            0..7 => {
                &mut self.planet[id as usize]
            }
            _ => panic!("GeneratableResourcesOnPlanet::get_generate called with invalid id"),
        }
    }
    /// Substitutes the current spinner with a new one
    pub fn set_planet_spinner(&mut self, id: ID, spinner: GenerateResourceSpinner) {
        match id as usize {
            0..7 => {
                self.planet[id as usize] = spinner;
            }
            _ => panic!("GeneratableResourcesOnPlanet::set_planet_spinner called with invalid id"),
        }
    }
}

#[derive(Resource)]
pub struct CombinableResourcesOnPlanet {
    planet: Vec<CombineResourceSpinner>,
}

impl Default for CombinableResourcesOnPlanet {
    fn default() -> Self {
        let mut spinners = Vec::new();
        for _ in 0..7 {
            let spinner = CombineResourceSpinner::new_empty();
            spinners.push(spinner);
        }
        assert_eq!(spinners.len(), 7);
        Self {
            planet: spinners,
        }
    }
}

impl CombinableResourcesOnPlanet {
    
    pub fn get_combine(&self, id: ID) -> &CombineResourceSpinner {
        match id as usize {
            0..7 => {
                &self.planet[id as usize]
            }
            _ => panic!("GeneratableResourcesOnPlanet::get_generate called with invalid id"),
        }
    }
    
    /// Returns a mutable reference to the corresponding GenerateResourceSpinner of the planet indexed 0-6
    pub fn get_combine_mut(&mut self, id: ID) -> &mut CombineResourceSpinner {
        match id as usize {
            0..7 => {
                &mut self.planet[id as usize]
            }
            _ => panic!("GeneratableResourcesOnPlanet::get_generate called with invalid id"),
        }
    }
    /// Substitutes the current spinner with a new one
    pub fn set_planet_spinner(&mut self, id: ID, spinner: CombineResourceSpinner) {
        match id as usize {
            0..7 => {
                self.planet[id as usize] = spinner;
            }
            _ => panic!("CombinableResourcesOnPlanet::set_planet_spinner called with invalid id"),
        }
    }
}

pub struct GenerateResourceSpinner {
    resources: Vec<BasicResourceType>,
    size: usize,
    index: usize,
}

impl GenerateResourceSpinner {
    /// Creates an empty Spinner
    pub fn new_empty() -> Self {
        GenerateResourceSpinner { resources: Vec::<BasicResourceType>::new(), size: 0, index: 0 }
    }
    /// Converts the HashSet of generable resource of a planet into a Vec in order to display it
    pub fn new(set: HashSet<BasicResourceType>) -> Self {
        let size = set.len();
        let resources = set.into_iter().collect::<Vec<BasicResourceType>>();
        GenerateResourceSpinner { resources, size, index: 0 }
    }
    pub fn increase(&mut self) {
        if self.size == 0 { return; }
        self.index = if self.index < self.size - 1 { self.index + 1 } else { 0 };
    }
    pub fn decrease(&mut self) {
        if self.size == 0 { return; }
        self.index = if self.index == 0 { self.size - 1 } else { self.index - 1 };
    }
    /// Returns a reference to BasicResourceType
    pub fn get_current_value(&self) -> Option<&BasicResourceType> {
        self.resources.get(self.index)
    }
}

pub struct CombineResourceSpinner {
    resources: Vec<ComplexResourceType>,
    size: usize,
    index: usize,
}

impl CombineResourceSpinner {
    /// Creates an empty Spinner
    pub fn new_empty() -> Self {
        CombineResourceSpinner { resources: Vec::<ComplexResourceType>::new(), size: 0, index: 0 }
    }
    /// Converts the HashSet of generable resource of a planet into a Vec in order to display it
    pub fn new(set: HashSet<ComplexResourceType>) -> Self {
        let size = set.len();
        let resources = set.into_iter().collect::<Vec<ComplexResourceType>>();
        CombineResourceSpinner { resources, size, index: 0 }
    }
    pub fn increase(&mut self) {
        if self.size == 0 { return; }
        self.index = if self.index < self.size - 1 { self.index + 1 } else { 0 };
    }
    pub fn decrease(&mut self) {
        if self.size == 0 { return; }
        self.index = if self.index == 0 { self.size - 1 } else { self.index - 1 };
    }
    /// Returns a reference to ComplexResourceType
    pub fn get_current_value(&self) -> Option<&ComplexResourceType> {
        self.resources.get(self.index)
    }
}

// Implement the Display trait for both CombineResourceSpinner and GenerateResourceSpinner
macro_rules! impl_spinner_display {
    ($t:ty) => {
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.resources.is_empty() {
                    write!(f, "← ASK FIRST")
                } else {
                    write!(f, "{:?}", self.resources[self.index])
                }
            }
        }
    };
}

impl_spinner_display!(CombineResourceSpinner);
impl_spinner_display!(GenerateResourceSpinner);


// ---------------------------------------------------------------
//      ExplorerPanel -> Info in Res<ExplorersData>
// ---------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use common_game::components::resource::{BasicResourceType, ComplexResourceType};
    use crate::manual_mode::resources::{CombinableResourcesOnPlanet, CombineResourceSpinner, GeneratableResourcesOnPlanet, GenerateResourceSpinner, PlanetSpinner};

    #[test]
    fn test_generate_resource_on_planet() {
        let mut generatable = GeneratableResourcesOnPlanet::default();
        for i in 0..7 {
            generatable.set_planet_spinner(i, create_gen_res_spinner());
        }
        println!("{}", generatable.get_generate(0));
        let spinner = generatable.get_generate_mut(0);
        println!("{:?}",  spinner.get_current_value());
        spinner.increase();
        println!("{:?}", spinner.get_current_value());
    }

    #[test]
    fn test_combine_resource_on_planet() {
        let mut combinable = CombinableResourcesOnPlanet::default();
        for i in 0..7 {
            combinable.set_planet_spinner(i, create_comb_res_spinner());
            println!("{}", combinable.get_combine_mut(0));
            let spinner = combinable.get_combine_mut(0);
            println!("{:?}",  spinner.get_current_value());
            spinner.increase();
            println!("{:?}", spinner.get_current_value());
        }
    }

    #[test]
    fn test_planet_spinner_display() {
        let mut planet_spinner = PlanetSpinner::default();
        assert_eq!(planet_spinner.get_current_value(), 0);
        println!("{}", planet_spinner); // 1
        planet_spinner.decrease();
        assert_eq!(planet_spinner.get_current_value(), 6);
        println!("{}", planet_spinner); // 7
        planet_spinner.increase();
        println!("{}", planet_spinner); // 1
        assert_eq!(planet_spinner.get_current_value(), 0);
    }

    #[test]
    fn test_basic_resource_spinner_display() {
        let mut basic_resource = create_gen_res_spinner();
        assert_eq!(basic_resource.size, 3);
        assert_eq!(basic_resource.index, 0);
        basic_resource.decrease();
        assert_eq!(basic_resource.index, 2);
        basic_resource.increase();
        assert_eq!(basic_resource.index, 0);
    }

    #[test]
    fn test_combine_resource_spinner_display() {
        let mut combine_resource = create_comb_res_spinner();
        assert_eq!(combine_resource.size, 3);
        assert_eq!(combine_resource.index, 0);
        combine_resource.decrease();
        assert_eq!(combine_resource.index, 2);
        combine_resource.increase();
        assert_eq!(combine_resource.index, 0);
    }

    fn create_gen_res_spinner() -> GenerateResourceSpinner {
        let mut set: HashSet<BasicResourceType> = HashSet::new();
        set.insert(BasicResourceType::Oxygen);
        set.insert(BasicResourceType::Carbon);
        set.insert(BasicResourceType::Hydrogen);
        GenerateResourceSpinner::new(set)
    }

    fn create_comb_res_spinner() -> CombineResourceSpinner {
        let mut set: HashSet<ComplexResourceType> = HashSet::new();
        set.insert(ComplexResourceType::Diamond);
        set.insert(ComplexResourceType::Life);
        set.insert(ComplexResourceType::AIPartner);
        CombineResourceSpinner::new(set)
    }
}
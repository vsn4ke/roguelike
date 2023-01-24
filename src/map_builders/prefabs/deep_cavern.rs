use super::{Description, PrefabSection, Surface, X, Y};

pub fn orc_camp_prefab() -> (PrefabSection, Vec<Description>) {
    let d: Vec<Description> = vec![
        Description::default(),
        Description::empty('≈', Surface::DeepWater),
        Description::floor('☼', "Watch Fire"),
        Description::floor('g', "Goblin"),
        Description::floor('O', "Orc Leader"),
        Description::floor('o', "Orc"),
    ];

    let section = PrefabSection {
        width: 12,
        height: 12,
        placement: (X::Center, Y::Center),
        template: "            
 ≈≈≈≈o≈≈≈≈≈ 
 ≈☼      ☼≈ 
 ≈ g      ≈ 
 ≈        ≈ 
 ≈    g   ≈ 
 o   O    o 
 ≈        ≈ 
 ≈ g      ≈ 
 ≈    g   ≈ 
 ≈☼      ☼≈ 
 ≈≈≈≈o≈≈≈≈≈ 
            ",
    };

    (section, d)
}

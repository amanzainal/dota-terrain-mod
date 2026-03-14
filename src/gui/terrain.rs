#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum TerrainCategory {
    Premium,
    Seasonal,
}

#[derive(Debug, Clone)]
pub struct TerrainInfo {
    pub id: u32,
    pub name: &'static str,
    pub vpk_file: &'static str,
    pub image_url: &'static str,
    #[allow(dead_code)]
    pub category: TerrainCategory,
    pub description: &'static str,
}

/// All available terrain skins.
///
/// Image URLs point to publicly hosted terrain preview images.
/// If an image fails to download, the app shows a styled placeholder.
pub const TERRAINS: &[TerrainInfo] = &[
    TerrainInfo {
        id: 1,
        name: "Desert Terrain",
        vpk_file: "dota_desert.vpk",
        image_url: "https://dota2.fandom.com/wiki/Special:Filepath/Desert_Terrain_Preview_1.jpg",
        category: TerrainCategory::Premium,
        description: "TI5 Battle Pass",
    },
    TerrainInfo {
        id: 2,
        name: "The King's New Journey",
        vpk_file: "dota_journey.vpk",
        image_url: "https://dota2.fandom.com/wiki/Special:Filepath/The_King%27s_New_Journey_Preview_1.jpg",
        category: TerrainCategory::Premium,
        description: "New Bloom 2017",
    },
    TerrainInfo {
        id: 3,
        name: "Immortal Gardens",
        vpk_file: "dota_coloseum.vpk",
        image_url: "https://dota2.fandom.com/wiki/Special:Filepath/Immortal_Gardens_Preview_1.jpg",
        category: TerrainCategory::Premium,
        description: "TI6 Battle Pass",
    },
    TerrainInfo {
        id: 4,
        name: "Overgrown Empire",
        vpk_file: "dota_jungle.vpk",
        image_url: "https://dota2.fandom.com/wiki/Special:Filepath/Overgrown_Empire_Preview_1.jpg",
        category: TerrainCategory::Premium,
        description: "TI9 Battle Pass",
    },
    TerrainInfo {
        id: 5,
        name: "Reef's Edge",
        vpk_file: "dota_reef.vpk",
        image_url: "https://dota2.fandom.com/wiki/Special:Filepath/Reef%27s_Edge_Preview_1.jpg",
        category: TerrainCategory::Premium,
        description: "TI7 Battle Pass",
    },
    TerrainInfo {
        id: 6,
        name: "Sanctums of the Divine",
        vpk_file: "dota_ti10.vpk",
        image_url: "https://dota2.fandom.com/wiki/Special:Filepath/Sanctums_of_the_Divine_Preview_1.jpg",
        category: TerrainCategory::Premium,
        description: "TI10 Battle Pass",
    },
    TerrainInfo {
        id: 7,
        name: "The Emerald Abyss",
        vpk_file: "dota_cavern.vpk",
        image_url: "https://dota2.fandom.com/wiki/Special:Filepath/The_Emerald_Abyss_Preview_1.jpg",
        category: TerrainCategory::Premium,
        description: "TI8 Battle Pass",
    },
    TerrainInfo {
        id: 8,
        name: "Autumn Terrain",
        vpk_file: "dota_autumn.vpk",
        image_url: "https://dota2.fandom.com/wiki/Special:Filepath/Seasonal_Terrain_-_Autumn_Preview_1.jpg",
        category: TerrainCategory::Seasonal,
        description: "Dota Plus Seasonal",
    },
    TerrainInfo {
        id: 9,
        name: "Winter Terrain",
        vpk_file: "dota_winter.vpk",
        image_url: "https://dota2.fandom.com/wiki/Special:Filepath/Seasonal_Terrain_-_Winter_Preview_1.jpg",
        category: TerrainCategory::Seasonal,
        description: "Dota Plus Seasonal",
    },
    TerrainInfo {
        id: 10,
        name: "Spring Terrain",
        vpk_file: "dota_spring.vpk",
        image_url: "https://dota2.fandom.com/wiki/Special:Filepath/Seasonal_Terrain_-_Spring_Preview_1.jpg",
        category: TerrainCategory::Seasonal,
        description: "Dota Plus Seasonal",
    },
    TerrainInfo {
        id: 11,
        name: "Summer Terrain",
        vpk_file: "dota_summer.vpk",
        image_url: "https://dota2.fandom.com/wiki/Special:Filepath/Seasonal_Terrain_-_Summer_Preview_1.jpg",
        category: TerrainCategory::Seasonal,
        description: "Dota Plus Seasonal",
    },
];

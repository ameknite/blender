use std::{
    fs::{self, File},
    io::Write,
    process::Command,
};

fn main() {
    create_master_themes();

    // Normal themes
    let normal_palettes = [
        Palette::new("latte", COLORS, LATTE_COLORS, None),
        Palette::new("frappe", COLORS, FRAPPE_COLORS, None),
        Palette::new("macchiato", COLORS, MACCHIATO_COLORS, None),
        Palette::new("mocha", COLORS, MOCHA_COLORS, None),
    ];
    for palette in normal_palettes {
        palette.create_themes("normal");
    }

    // Medium themes
    let normal_palettes = [
        Palette::new("latte", COLORS, LATTE_COLORS, None),
        Palette::new("frappe", COLORS, LATTE_COLORS, None),
        Palette::new("macchiato", COLORS, LATTE_COLORS, None),
        Palette::new("mocha", COLORS, LATTE_COLORS, None),
    ];
    for palette in normal_palettes {
        palette.create_themes("medium");
    }

    // Contrast themes
    let contrast_palettes = [
        Palette::new("latte", COLORS, LATTE_COLORS, Some(LATTE_COLORS)),
        Palette::new("frappe", COLORS, FRAPPE_COLORS, Some(LATTE_COLORS)),
        Palette::new("macchiato", COLORS, MACCHIATO_COLORS, Some(LATTE_COLORS)),
        Palette::new("mocha", COLORS, MOCHA_COLORS, Some(LATTE_COLORS)),
    ];
    for palette in contrast_palettes {
        palette.create_themes("contrast");
    }

    delete_master_themes();
}
static MASTER_PALLET: &str = "mocha_master.xml";
static ACCENT_COLOR: &str = "#bada55";

static COLORS: [&str; 14] = [
    "rosewater",
    "flamingo",
    "pink",
    "mauve",
    "red",
    "maroon",
    "peach",
    "yellow",
    "green",
    "teal",
    "sky",
    "sapphire",
    "blue",
    "lavender",
];
static LATTE_COLORS: [&str; 14] = [
    "#dc8a78", "#dd7878", "#ea76cb", "#8839ef", "#d20f39", "#e64553", "#fe640b", "#df8e1d",
    "#40a02b", "#179299", "#04a5e5", "#209fb5", "#1e66f5", "#7287fd",
];
static FRAPPE_COLORS: [&str; 14] = [
    "#f2d5cf", "#eebebe", "#f4b8e4", "#ca9ee6", "#e78284", "#ea999c", "#ef9f76", "#e5c890",
    "#a6d189", "#81c8be", "#99d1db", "#85c1dc", "#8caaee", "#babbf1",
];
static MACCHIATO_COLORS: [&str; 14] = [
    "#f4dbd6", "#f0c6c6", "#f5bde6", "#c6a0f6", "#ed8796", "#ee99a0", "#f5a97f", "#eed49f",
    "#a6da95", "#8bd5ca", "#91d7e3", "#7dc4e4", "#8aadf4", "#b7bdf8",
];
static MOCHA_COLORS: [&str; 14] = [
    "#f5e0dc", "#f2cdcd", "#f5c2e7", "#cba6f7", "#f38ba8", "#eba0ac", "#fab387", "#f9e2af",
    "#a6e3a1", "#94e2d5", "#89dceb", "#74c7ec", "#89b4fa", "#b4befe",
];

#[derive(Default, Debug)]
struct Color {
    name: String,
    hex: String,
    contrast: Option<String>,
}

#[derive(Default, Debug)]
struct Palette {
    name: String,
    colors: Vec<Color>,
}

impl Palette {
    fn new(
        name: &str,
        colors: [&str; 14],
        hexes: [&str; 14],
        contrast_colors: Option<[&str; 14]>,
    ) -> Self {
        let mut colors: Vec<Color> = colors
            .iter()
            .zip(hexes.iter())
            .map(|(&color, &hex)| Color {
                name: color.into(),
                hex: hex.into(),
                contrast: None,
            })
            .collect();

        if let Some(contrast_colors) = contrast_colors {
            colors
                .iter_mut()
                .zip(contrast_colors.iter())
                .for_each(|(color, &contrast)| color.contrast = Some(contrast.into()))
        }
        Self {
            name: name.into(),
            colors,
        }
    }

    fn create_themes(&self, folder_name: &str) {
        let file_name = format!("{}.xml", self.name);
        let file_content = fs::read_to_string(file_name).expect("Filed to read file.xml");

        for color in &self.colors {
            // file modifications

            // change accent_color
            let mut temp_file = file_content.replace(ACCENT_COLOR, &color.hex);

            // change colors to contrast colors
            for cycle in &self.colors {
                if let Some(contrast) = &cycle.contrast {
                    temp_file = temp_file.replace(&cycle.hex, contrast);
                }
            }

            // create folders
            let path_folders = format!("../themes/{}/{}", folder_name, self.name);
            fs::create_dir_all(&path_folders).expect("failed to create folder");

            // create files
            let path_file = format!("{}/{}_{}.xml", path_folders, self.name, color.name);
            let mut file = File::create(path_file).expect("Filed to create File");
            file.write_all(temp_file.as_bytes())
                .expect("Filed to  write to File");
        }
    }
}

fn create_master_themes() {
    // delete themes folder
    let dir_path = "../themes";
    if fs::metadata(dir_path).is_ok() {
        fs::remove_dir_all(dir_path).expect("Failed to removed themes");
    }
    // create themes
    let mut child = Command::new("puccinier")
        .args(["-s", MASTER_PALLET])
        .args(["-o", "latte", "frappe", "macchiato", "mocha"])
        .spawn()
        .expect("Failed to generated master themes");
    child.wait().expect("failed to wait on child");
}

fn delete_master_themes() {
    fs::remove_file("latte.xml").expect("Error deleting generated latte theme");
    fs::remove_file("frappe.xml").expect("Error deleting generated frappe theme");
    fs::remove_file("macchiato.xml").expect("Error deleting generated macchiato theme");
    fs::remove_file("mocha.xml").expect("Error deleting generated mocha theme");
}

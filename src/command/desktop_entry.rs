//     DesktopEntry {
//     exe: PathBuf,
//     name: String,
//     save_name: Option<String>,
// },

use std::path::PathBuf;

use exe::{VecPE, ResourceDirectory, Buffer};

fn get_icon(exe: PathBuf) {
    let image = VecPE::from_disk_file(exe).unwrap();
    let res = ResourceDirectory::parse(&image).unwrap();
    let groups = res.icon_groups(&image).unwrap();
    let v = groups.values().into_iter().next().unwrap();
    let buf = v.to_icon_buffer(&image).unwrap();
    let img = image::load_from_memory(buf.as_slice()).unwrap();
    let img = img.resize(256, 256, image::imageops::FilterType::Lanczos3);
    img.save("icon.png").unwrap();
}
// {
//     exe,
//     save_name,
//     name,
// } => {
//     let save_name = save_name
//         .as_ref()
//         .map(|s| s.as_str())
//         .unwrap_or_else(|| exe.file_stem().unwrap().to_str().unwrap());
//     // let mut de = DesktopEntry::default();
//     // de.exec = Some(format!("proton-launch run {}", exe.display()));
//     // de.name = Some(name.clone());
//     // de.comment = Some(format!("Run {} with Proton", name));
//     let mut de = DesktopEntry::new(name.clone());
//     de.comment = format!("Run {} with Proton", name);
//     de.exec = format!("proton-launch run {}", exe.display());
//     let de = de_to_string(&de);
//     println!("{}", de);

// }

// fn de_to_string(de: &DesktopEntry) -> String {
//     serde_ini::ser::to_string(de).unwrap()
// }

// #[derive(Default, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct DesktopEntry {
//     #[serde(rename = "Type")]
//     xdg_type: String,
//     version: String,
//     name: String,
//     comment: String,
//     exec: String,
//     path: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     terminal: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     icon: Option<String>,
// }

// impl DesktopEntry {
//     fn new(name: impl ToString) -> Self {
//         Self {
//             xdg_type: "Application".to_string(),
//             version: "1.0".to_string(),
//             name: name.to_string(),
//             comment: "".to_string(),
//             icon: None,
//             exec: "".to_string(),
//             path: "".to_string(),
//             terminal: None,
//         }
use field_count::FieldCount;

#[derive(Debug, FieldCount)]
pub struct ProgramData {
    pub original_image_path: String,
    pub new_imagepath: String,
    pub heatmap_path: String,
    pub heatmap_intensity: u8
}

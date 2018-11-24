#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Time {
    One(usize),
    Two(usize, usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    pub code: u16,
    pub operator: String,
    pub operands: Vec<String>,
    pub bits: usize,
    pub size: usize,
    pub time: Time,
    pub z: String,
    pub n: String,
    pub h: String,
    pub c: String,
}

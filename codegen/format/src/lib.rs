#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Time {
    One(usize),
    Two(usize, usize),
}

#[derive(Debug, Serialize, Deserialize)]
struct Instruction {
    code: u16,
    operator: String,
    operands: Vec<String>,
    bits: usize,
    size: usize,
    time: Time,
    z: String,
    n: String,
    h: String,
    c: String,
}


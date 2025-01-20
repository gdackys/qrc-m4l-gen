mod calculate_error_correction;
mod encode_alphanumeric;
mod gf_256;

fn main() {
    let data = "AC-42";
    let result = encode_alphanumeric::encode_alphanumeric(data);

    println!("{:?}", result);
}

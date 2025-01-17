mod encode_alphanumeric;

fn main() {
    let data = "AC-42";
    let result = encode_alphanumeric::encode_alphanumeric(data);

    println!("{:?}", result);
}

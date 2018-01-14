extern crate rust_sodium;
extern crate hex;

mod wallet;

fn main() {
    let address = wallet::create_address();

    println!("Public ID: {}", address.public_id.to_hex());
    println!("Public Key: {:?}", address.public_key);
    println!("Private Key: {:?}", address.private_key);
    println!("Amount: {:?}", address.amount);
}

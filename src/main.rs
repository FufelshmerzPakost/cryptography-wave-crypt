use rand::Rng;
use hex;

fn hex_to_vec(message: &str) -> Vec<u8> {
    let mut res:Vec<u8> = Vec::new();
    for i in 0..message.len()/2 {
        res.push(hex::decode(&message[0+i*2..2 + i*2]).unwrap()[0]); 
    }
    res
}

fn f_to(x: u8, z: f32, n: f32, dx: f32) -> char {
    let mut res = x as f32 + 255f32 * (z + n * dx).cos();
    if res < 0f32{
        res += 255f32;
    } else if res > 255f32 {
        res -= 255f32;
    }
    (res.ceil()as u8) as char
}

fn f_of(x: u8, z: f32, n: f32, dx: f32) -> char {
    let mut res = x as f32 - 255f32 * (z + n * dx).cos();
    if res < 0f32{
        res += 255f32;
    } else if res > 255f32 {
        res -= 255f32;
    }
    (res.floor()as u8) as char
}

fn encrypt(message: &str, z: f32, dx: f32) -> String {
    let mut res: String = String::new();
    for (idx, num) in message.chars().enumerate() {
        let val = f_to(num as u8, z, idx as f32, dx);
        res.push_str(&format!("{:02x}", val as u8));
    }
    res
}

fn decrypt(enc_message: &str, z: f32, dx: f32) -> String {
    let mut res: String = String::new();
    let temp: Vec <u8> = hex_to_vec(enc_message);
    for (idx, num) in temp.iter().enumerate() {
        res.push(f_of(*num, z, idx as f32, dx));
    }
    res
}

fn main() {

    let message: &str = "A bear was walking through the forest and saw a car on fire. He got in the car and burned to death.";
    let dx: f32 = rand::thread_rng().gen_range(-500..500) as f32;
    let z: f32 = rand::thread_rng().gen_range(-500..500) as f32;

    let enc_message = encrypt(message, z, dx);
    let dec_message = decrypt(&enc_message[..], z, dx);

    println!("{:?}", hex::decode(enc_message).unwrap().iter().map(|x| *x as char).collect::<String>());
    println!("{:?}", dec_message);
}

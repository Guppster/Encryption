extern crate rand;

use std::io;
use std::cmp::Ordering; 
use std::ascii::AsciiExt;
use rand::Rng;

struct ciPolyalphabetic
{
	key: String,
	message: String,
	output: String,
}

impl ciPolyalphabetic 
{
	fn Encode(self)
	{
		if self.key == "" 
		{
			self.output = "ERROR: Key or Message was not set!".to_string();
		}

		let charsKey: Vec<char> = self.key.chars().collect();
		let charsMessage: Vec<char> = self.message.chars().collect();
		let mut intsEncrypted: Vec<u32> = Vec::new();
		let mut counter: u32 = 0;

		for x in charsMessage
		{
			intsEncrypted.push(x as u32 + charsKey[counter%charsKey.len()] as u32);
			
			while intsEncrypted[counter] > 126
			{
				intsEncrypted[counter] -= 95;
			}

			counter = counter + 1;
		}

		for y in intsEncrypted
		{
			self.output.push(char::from_u32(y));
		}
	}

	fn Decode()
	{
		
	}
}

fn main() 
{
    println!("Hello, world!\n");

    println!("Please select a cipher type:");
    println!("[P]olyalphabetic:");
    println!("[X]OR");
    println!("[O]ne Time Pad");

    let mut cipher = String::new();

    io::stdin().read_line(&mut cipher)
    	.ok()
    	.expect("Failed to read line");

    if cipher.eq_ignore_ascii_case("P") || cipher.eq_ignore_ascii_case("Polyalphabetic")
    {

    }
    else if cipher.eq_ignore_ascii_case("x") || cipher.eq_ignore_ascii_case("XOR")
    {

    }
    else if cipher.eq_ignore_ascii_case("O") || cipher.eq_ignore_ascii_case("One Time Pad")
    {

    }

}

/*
Base64 is mainly used as an encoding cipher to input binary data as printable text.
Usually it requires an indexing map of 64 characters in where the input data will be
split into 6 bit streams which are then interperted by as the base64 character map.
These strings usually need to be of divisible by 4 which is why when the output is not
satisfying those requirements it's padded by the '=' character at the end of the string
*/

//Create the base64 index table
#[allow(non_upper_case_globals)]
const table : &'static [u8] =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();

/*
Base64 encoding process
For the time being the input will be of string type (when it should be byte array &[u8])
Great and short encoding implementation from Ahmed Elzoughby originally in C
https://github.com/elzoughby/Base64/blob/master/base64.c
*/
pub fn encode<T : Into<String> + Copy>(_s : T) -> String {
    //Collect the chars as bytes to interpret
    let input : Vec<u8> = _s.into().as_bytes().to_vec();

    //Result cipher
    let mut result_cipher : String = String::new();

    //Store index
    let mut index = 0;

    //Store buffer
    let mut buffer : [u8; 3] = [0u8; 3];

    //Convert to the 6 bit stream cipher which will then gain the result character from the index table
    for i in &input {
        buffer[index] = *i;
        index += 1;
        if index == 3 {
            result_cipher.push(table[(buffer[0] >> 2) as usize] as char);
            result_cipher.push(table[(((buffer[0] & 0x03) << 4) + (buffer[1] >> 4)) as usize] as char);
            result_cipher.push(table[(((buffer[1] & 0x0f) << 2) + (buffer[2] >> 6)) as usize] as char);
            result_cipher.push(table[(buffer[2] & 0x3f) as usize] as char);
            index = 0;
        }
    }

    if index > 0 {
        result_cipher.push(table[(buffer[0] >> 2) as usize] as char);
        if index == 1 {
            result_cipher.push(table[(((buffer[0] & 0x03) << 4)) as usize] as char);
            result_cipher.push('=');
        }
        //Index at 2
        else { 
            result_cipher.push(table[(((buffer[0] & 0x03) << 4) + (buffer[1] >> 4)) as usize] as char);
            result_cipher.push(table[(((buffer[1] & 0x0f) << 2)) as usize] as char);
        }
        result_cipher.push('=');
    }

    //Get the resulting encoded cipher
    result_cipher
}

/*
Get index from the indexing table (returns 0 for zero padding for char '=')
*/
fn get_index_table(inp : u8) -> usize {
    if inp as char == '=' { 0 }
    else { 
        table.iter().position(|&a| a == inp).unwrap()
    }
}

/*
Base64 decoding process
*/
pub fn decode<T : Into<String> + Copy>(_s : T) -> String {
    if _s.into().len() % 4 != 0 {
        panic!("Base64 strings require length to be divisible by 4");
    }
    
    //Collect the chars as bytes to interpret
    let input : Vec<u8> = _s.into().as_bytes().to_vec();

    //Return heap allocated buffer
    let mut output_buffer : String = String::new();

    for i in (0..input.len()).step_by(4) {
        //Get the 6 bit stream from four of the chars
        //This'll get the 24 bit number stream which will then
        //Be divided to the 8 bit stream for decoding process
        let _p = (get_index_table(input[i]) << 18) +
                 (get_index_table(input[i + 1]) << 12) + 
                 (get_index_table(input[i + 2]) << 6) +
                 (get_index_table(input[i + 3]));

        //Push the characters to the string buffer
        //This will take the 24 bit number stream and get the
        //8-bit values interpreted to the original ASCII 
        output_buffer.push((((_p >> 16) & 0xFF) as u8) as char);
        output_buffer.push((((_p >> 8) & 0xFF) as u8) as char);
        output_buffer.push(((_p & 0xFF) as u8) as char);
    }

    //Return resulting buffer
    output_buffer
}
//Create the base64 index table
#[allow(non_upper_case_globals)]
const table : &'static [u8] =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();

//Great and short encoding implementation from Ahmed Elzoughby originally in C
//https://github.com/elzoughby/Base64/blob/master/base64.c
/*
Base64 encoding process
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
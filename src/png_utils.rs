use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
// The Png Encoding algorithm has seven steps:
// Path Extraction
// Scanline Serialization
// Filtering
// Compression
// Chunking
// -> Datastream Consutrction.
// That eans, in order to decode a png, we need to do the reverse of these steps.
// So we start with a data stream
// prase it out into chunks
// decompress the Dchunks
// unfilter the scanlines
// deserialize the scanlines
// and then we have the image data.
//

struct Chunk {
    length: u32,
    chunk_type: [u8; 4],
    chunk_data: Box<[u8]>,
    crc: [u8; 4],
}

fn itoh(x: u8) -> char {
    assert!(x < 16);
    let hex_values = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    return hex_values[x as usize];
}

fn convert_to_hex(num: u8) -> String {
    let mut word = String::new();
    let mut running_value = num;
    loop {
        let quotient: u8 = running_value / 16;
        let remainder: u8 = running_value % 16;

        if quotient > 16 {
            word.push(itoh(remainder));
        } else {
            word.push(itoh(quotient));
            word.push(itoh(remainder));
            break;
        }
        running_value = quotient;
    }
    return word;
}

fn chunk_reader(chunk: &[u8]) {
    for &i in chunk {
        println!("{}, {}, {}", convert_to_hex(i), i, i as char);
    }
}
fn verify_signature(signature: &[u8; 8]) -> bool {
    let valid_png: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
    return signature == &valid_png;
}
fn parse_stream_into_chunks<R>(stream: &mut BufReader<R>) -> Option<Chunk>
where
    R: std::io::Read,
{
    let mut chunk_length: [u8; 4] = [0; 4];
    let mut chunk_type: [u8; 4] = [0; 4];
    let mut crc: [u8; 4] = [0; 4];

    stream.read_exact(&mut chunk_length).unwrap();
    let len_as_u32 = u32::from_be_bytes(chunk_length);
    let mut chunk_data = vec![0; len_as_u32 as usize];

    stream.read_exact(&mut chunk_type).unwrap();
    stream.read_exact(&mut chunk_data).unwrap();
    stream.read_exact(&mut crc).unwrap();

    let chunk = Some(Chunk {
        length: len_as_u32,
        chunk_type,
        chunk_data: chunk_data.into_boxed_slice(),
        crc,
    });

    match chunk {
        Some(chunk) => {
            println!("Length: {}", chunk.length);
            println!("Type: {:?}", chunk.chunk_type);
            println!("Type as Ascii: {:?}", std::str::from_utf8(&chunk.chunk_type).unwrap());
            println!("Data: {:?}", chunk.chunk_data);
            println!("CRC: {:?}", chunk.crc);
            return Some(chunk);
        }
        _ => {
            println!("No chunk!");
            return None;
        }
    }
}

fn main() -> io::Result<()> {
    let png_file = File::open("../monsters.png")?;
    let mut reader = BufReader::new(png_file);
    let mut signature: [u8; 8] = [0; 8];
    reader.read_exact(&mut signature).unwrap();
    if !verify_signature(&signature) {
        panic!("Not a valid signature");
    };

    loop {
        let chunk = parse_stream_into_chunks(&mut reader).unwrap();
        if chunk.chunk_type == ([73, 69, 78, 68]) {
            break;
        }

    }
    //    let mut idhr_length_bytes: [u8;4] = [0;4];
    //    reader.read_exact(&mut idhr_length_bytes).unwrap();
    //    let idhr_length = u32::from_be_bytes(idhr_length_bytes);
    //
    //    let mut idhr_chunk_type : [u8;4] = [0;4];
    //    reader.read_exact(&mut idhr_chunk_type).unwrap();
    //
    //    let mut idhr_chunk_data  = vec![0; idhr_length as usize];
    //    reader.read_exact(&mut idhr_chunk_data).unwrap();
    //
    //    let mut idhr_crc: [u8;4] = [0;4];
    //    reader.read_exact(&mut idhr_crc);
    //
    //    let idhr_chunk = Chunk{
    //        length:idhr_length,
    //        chunk_type: idhr_chunk_type,
    //        chunk_data: idhr_chunk_data.into_boxed_slice(),
    //        crc: idhr_crc
    //    };
    //    // Debug print
    //    println!("IHDR Length: {}", idhr_chunk.length);
    //    println!("IHDR Type: {:?}", idhr_chunk.chunk_type);
    //    println!("IHDR Data: {:?}", idhr_chunk.chunk_data);
    //    println!("IHDR CRC: {:?}", idhr_chunk.crc);
    Ok(())
}

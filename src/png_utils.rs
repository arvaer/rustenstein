use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;

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
#[derive(Debug)]
struct ImageMetaData {
    width: u32,
    height: u32,
    bit_depth: u8,
    colour_type: u8,
    compression: u8,
    filter_method: u8,
    interlace_method: u8,
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

fn print_chunk(chunk: &Chunk) {
    println!("Length: {}", chunk.length);
    println!("Type: {:?}", chunk.chunk_type);
    println!(
        "Type as Ascii: {:?}",
        std::str::from_utf8(&chunk.chunk_type).unwrap()
    );
    println!("Data: {:?}", chunk.chunk_data);
    println!("CRC: {:?}", chunk.crc);
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
            //            print_chunk(chunk);
            return Some(chunk);
        }
        _ => {
            println!("No chunk!");
            return None;
        }
    }
}

fn parse_image_meta_data(chunk: &Chunk) -> ImageMetaData {
    let mut image_meta_data: ImageMetaData = ImageMetaData {
        width: 0,
        height: 0,
        bit_depth: 0,
        colour_type: 0,
        compression: 0,
        filter_method: 0,
        interlace_method: 0,
    };
    let mut width_bytes = [0u8; 4];
    width_bytes.copy_from_slice(&chunk.chunk_data[0..4]);
    image_meta_data.width = u32::from_be_bytes(width_bytes);

    let mut height_bytes = [0u8; 4];
    height_bytes.copy_from_slice(&chunk.chunk_data[4..8]);
    image_meta_data.height = u32::from_be_bytes(height_bytes);

    image_meta_data.bit_depth = u8::from_be_bytes([chunk.chunk_data[8]]);
    image_meta_data.colour_type = u8::from_be_bytes([chunk.chunk_data[9]]);
    image_meta_data.compression = u8::from_be_bytes([chunk.chunk_data[10]]);
    image_meta_data.filter_method = u8::from_be_bytes([chunk.chunk_data[11]]);
    image_meta_data.interlace_method = u8::from_be_bytes([chunk.chunk_data[12]]);

    return image_meta_data;
}

fn paeth_predictor(a: u8, b: u8, c: u8) -> u8 {
    let p = a.wrapping_add(b).wrapping_sub(c);
    let pa = p.abs_diff(a);
    let pb = p.abs_diff(b);
    let pc = p.abs_diff(c);

    let pr = if pa <= pb && pa <= pc {
        a
    } else if pb <= pc {
        b
    } else {
        c
    };

    return pr;
}

fn unfilter_scanline(
    filter_type: u8,
    filtered_scanline: &[u8],
    previous_scanline: Option<&[u8]>,
    scanline_length: usize,
) -> Vec<u8> {
    let mut unfiltered_scanline: Vec<u8> = vec![0; scanline_length];

    for (i, &x) in filtered_scanline.iter().enumerate() {
        let recon_a: u8 = if i > 0 { filtered_scanline[i - 1] } else { 0 };

        let recon_b: u8 = if previous_scanline.is_some() { previous_scanline.unwrap()[i] } else { 0 };

        let recon_c = if previous_scanline.is_some() {
            if i > 0 { previous_scanline.unwrap()[i - 1] } else { 0 }
        } else {
            0
        };

        let recon_x = match filter_type {
            0 => x,
            1 => x.wrapping_add(recon_a),
            2 => x.wrapping_add(recon_b),
            3 => {
                let apb = recon_a.wrapping_add(recon_b) / 2;
                x.wrapping_add(apb)
            }
            4 => x.wrapping_add(paeth_predictor(recon_a, recon_b, recon_c)),
            _ => {
                panic!("Unsupported filter type: {}", filter_type);
            }
        };

        unfiltered_scanline[i] = recon_x;
    }

    return unfiltered_scanline;
}

fn main() -> io::Result<()> {
    let png_file = File::open("./monsters.png")?;
    let mut reader = BufReader::new(png_file);
    let mut signature: [u8; 8] = [0; 8];
    reader.read_exact(&mut signature).unwrap();
    if !verify_signature(&signature) {
        panic!("Not a valid signature");
    };

    let mut idat_data_stream: Vec<u8> = Vec::new();
    let mut scanline_length: usize = 0;
    let mut image_meta_data: Option<ImageMetaData> = None;

    loop {
        let chunk = parse_stream_into_chunks(&mut reader).unwrap();
        if chunk.chunk_type == ([73, 72, 68, 82]) {
            image_meta_data = Some(parse_image_meta_data(&chunk));
            println!("{:?}", image_meta_data);
            if let Some(meta_data) = &image_meta_data {
                scanline_length = meta_data.width as usize * 4;
            }
        }

        if chunk.chunk_type == ([73, 69, 78, 68]) {
            break;
        }
        if chunk.chunk_type == ([73, 68, 65, 84]) {
            idat_data_stream.extend_from_slice(&chunk.chunk_data);
        }
    }
    let mut decoder = ZlibDecoder::new(&idat_data_stream[..]);
    let mut decompressed_data = Vec::new();
    let mut previous_scanline: Option<&[u8]> = None;

    decoder.read_to_end(&mut decompressed_data)?;
    for (_i, scanline) in decompressed_data.chunks(scanline_length + 1).enumerate() {
        let filter_type = scanline[0];
        let filtered_scanline = &scanline[1..];

        let unfiltered_scanline = unfilter_scanline(
            filter_type,
            filtered_scanline,
            previous_scanline,
            scanline_length,
        );
        previous_scanline = Some(filtered_scanline);
        println!("Filter type: {}", filter_type);
        println!("Scanline: {:?}", unfiltered_scanline);
    }

    Ok(())
}

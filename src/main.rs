use std::env;

#[derive(Debug)]
struct Sizes {
  Bytes: u64,
  Kilobytes: u64,
  Megabytes: u64,
  Gigabytes: u64,
}

#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}

fn format_all_sizes() -> Sizes {

    let mut s: Sizes = Sizes {
         Bytes: 42,
         Kilobytes: 42,
         Megabytes: 42,
         Gigabytes: 42,
    };

    //return sizes struct
    s
}

fn main() {

    // grab the first arg after the binary name
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("too many args...");
        return;
    }

    //split the arg into size and specifer: (./fsfmt '24 mb')
    let arg_str = &args[1];
    let items: Vec<&str> = arg_str.split_whitespace().collect();

    //parse input
    let size: u32 = items[0].trim().parse().expect("Wanted a number");
    let desc: String = items[1].trim().parse().expect("Expected" );
    println!("items: {:?} {:?}",&items[0], &items[1]);

    //let result = format_size(6888837399);
    //println!("{}", result);

    let sz = format_all_sizes();
    println!("{:?}", sz);

}

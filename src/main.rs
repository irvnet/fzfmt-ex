use std::env;

#[derive(Debug)]
struct Sizes {
  Bytes: u64,
  Kilobytes: u64,
  Megabytes: u64,
  Gigabytes: u64,
}

impl Sizes {

    fn new() -> Sizes {
       let n: Sizes = Sizes {
            Bytes:     0,
            Kilobytes: 0,
            Megabytes: 0,
            Gigabytes: 0,
        };

        // return new Sizes struct
        n
    }

    fn new_sizes_from_mb(size_arg: u64, u: String) -> Sizes {
        let mut s: Sizes = Sizes::new();
    
        // match the string with a measurement
        match &u as &str {
              "b"  => { 
                s.Bytes = size_arg;
                s.Kilobytes = s.Bytes / 1024;       // cvt: b -> kb 
                s.Megabytes = s.Bytes / 1048576;    // cvt: b -> mb 
                s.Gigabytes = s.Bytes / 1073741824; // cvt: b -> gb 
              }

              "kb" => {
                s.Kilobytes = size_arg;
                s.Bytes     = s.Kilobytes * 1024;    // cvt: kb -> b
                s.Megabytes = s.Kilobytes / 1024;    // cvt: kb -> mb 
                s.Gigabytes = s.Kilobytes / 1048576; // cvt: kb -> gb 
              }

              "mb" => {
                s.Megabytes = size_arg;
                s.Bytes     = s.Megabytes * 1048576; // cvt: mb -> b
                s.Kilobytes = s.Megabytes * 1024;    // cvt: mb -> kb 
                s.Gigabytes = s.Megabytes / 1024;    // cvt: mb -> gb 
              } 

              "gb" => {
                s.Gigabytes = size_arg;
                s.Bytes     = s.Gigabytes * 1073741824; // cvt: gb -> b
                s.Kilobytes = s.Gigabytes * 1048576;    // cvt: gb -> kb 
                s.Megabytes = s.Gigabytes * 1024;       // cvt: gb -> mb 
              }
            
              _    => {
                println!("unrecognized unit...");
                std::process::exit(1);
              }
        }
    



        //return sizes struct
        s
    }

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


fn main() {

    // grab the first arg after the binary name
    let mut args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("too many args...");
        return;
    } else if args.len() < 2 {
        println!("not enough args...setting default as '70000 mb'");
        args.push("70000 mb".to_string());
    }

   //split the arg into size and specifer: (./fsfmt '24 mb')
   let arg_str = &args[1];
   let items: Vec<&str> = arg_str.split_whitespace().collect();

   //parse input
   let size_u32: u32 = items[0].trim().parse().expect("Wanted a number");
   let size_u64: u64 = size_u32 as u64;
   let desc: String = items[1].trim().parse().expect("Expected" );
   //println!("items: {:?} {:?}",&items[0], &items[1]);
    
   // create new sizes struct instance
   let file_sizes: Sizes = Sizes::new_sizes_from_mb(size_u64, desc);
   println!("updated MB as: {:?}",file_sizes);





      //let result = format_size(6888837399);
   //println!("{}", result);
}

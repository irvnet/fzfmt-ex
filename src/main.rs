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

    fn new_sizes_from_mb(s: u64, u: String) -> Sizes {

        let mut s: Sizes = Sizes::new();
        println!("new sizes struct: {:?}",s);
    
        // match the string with a measurement
        match &u as &str {
            "mb" => println!("megabytes"),
            _    => {
                // only accept megabytes as input (otherwise exit)
                println!("unrecognized unit...use 'mb' for megabytes please... ");
                std::process::exit(1);
            }
        }
    
        //return sizes struct
        s
    }

    
    fn set_megabytes(&mut self, s:u64) {
        self.Megabytes = s;
    }


    fn cvt_mb2gb(&mut self) {
        let gb = self.Megabytes as f64 / (1024.0 * 1024.0); 
        self.Gigabytes = gb as u64;
    }

}



#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

fn format_all_sizes(s: u64, u: String) -> Sizes {

    let mut s: Sizes = Sizes::new();
    println!("new sizes struct: {:?}",s);

    // match the string with a measurement
    match &u as &str {
        "mb" => println!("megabytes"),
        _    => {
            // only accept megabytes as input (otherwise exit)
            println!("unrecognized unit...use 'mb' for megabytes please... ");
            std::process::exit(1);
        }
    }

    //return sizes struct
    s
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
        println!("not enough args...setting default as '23 mb'");
        args.push("40000 mb".to_string());
    }

   //split the arg into size and specifer: (./fsfmt '24 mb')
   let arg_str = &args[1];
   let items: Vec<&str> = arg_str.split_whitespace().collect();

   //parse input
   let size_u32: u32 = items[0].trim().parse().expect("Wanted a number");
   let size_u64: u64 = size_u32 as u64;
   let desc: String = items[1].trim().parse().expect("Expected" );
   println!("items: {:?} {:?}",&items[0], &items[1]);
    
   //let result = format_size(6888837399);
   //println!("{}", result);

   // create new sizes struct instance
   let mut file_sizes: Sizes = Sizes::new();
   file_sizes.set_megabytes(size_u64);
   println!("updated MB as: {:?}",file_sizes);


   // convert from mb to "everything else"
   //todo!("add: 's.update_all_sizes();' ");

   // print results
   //todo!("add: 'println(s);' ");

   //todo!("moving this to impl for Sizes struct");
   let sz = format_all_sizes(size_u64, desc);
   println!("{:?}", sz);

}

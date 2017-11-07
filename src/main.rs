extern crate web3;

use std::thread;
use std::time::Duration;

use web3::futures::Future;
use web3::types::{Address, U256};

fn main() {
    let (eloop, transport) = web3::transports::Ipc::new("/home/tomusdrw/.local/share/io.parity.ethereum/jsonrpc.ipc").unwrap();
    let web3 = web3::Web3::new(transport);

    let accounts: Vec<Address> = vec![
        "0x3bfc20f0b9afcace800d73d2191166ff16540258".parse().unwrap(),
    ];

    let accounts = read_from_file();

    for account in accounts {
        let call = web3.eth().code(account.clone(), None);
        let call2 = web3.eth().balance(account.clone(), None);
        eloop.remote().spawn(move |_| call.and_then(move |code| {
            call2.map(move |balance| {
                if !code.0.is_empty() {
                    println!("{:?}: {:?}", account, as_eth(&balance));
                }
            })
        }).map_err(|e| println!("Error: {:?}", e)));
    }

    thread::sleep(Duration::from_secs(3600));
}

fn read_from_file() -> Vec<Address> {
    use std::{fs, io};
    use std::io::BufRead;
    let file = fs::File::open("./wallets2").unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().filter_map(|x| x.unwrap().parse().ok()).collect()
}

fn as_dec(val: &U256) -> f64 {
  let len = val.0.len();
  let mut v: f64 = 0.0;
  for i in 0..len {
    v += val.0[i] as f64;
    if i != len - 1 {
      v *= 256.0;
    }
  }
  v
}

fn as_eth(val: &U256) -> String {
  let v = as_dec(val);

  if v < 1e14 {
    format!("{} = {:?}", v, val)
  } else {
    format!("{:.5}Ξ", v / 1e18)
  }
}

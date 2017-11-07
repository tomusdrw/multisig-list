extern crate web3;

use web3::futures::Future;

fn main() {
    let (eloop, transport) = web3::transports::Ipc::new("/home/tomusdrw/.local/share/io.parity.ethereum/jsonrpc.ipc").unwrap();
    let web3 = web3::Web3::new(transport);

    let accounts = vec![
        "0x3bfc20f0b9afcace800d73d2191166ff16540258".parse().unwrap(),
    ];

    for account in accounts {
        let call = web3.eth().code(account, None);
        eloop.remote().spawn(move |_| call.map(|code| {
            println!("code: {:?}", code);
        }).map_err(|e| println!("Error: {:?}", e)));
    }
}

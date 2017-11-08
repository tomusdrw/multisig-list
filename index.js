const fs = require('fs')

const file = fs.readFileSync('./owners', 'utf8')

const { walletToOwners, ownerToWallets } = file.split('\n').reduce((x, line) => {
  if (line.startsWith('..')) {
    const owner = line.substr(2)
    x.walletToOwners[x.lastWallet] = x.walletToOwners[x.lastWallet] || []
    x.walletToOwners[x.lastWallet].push(owner)

    x.ownerToWallets[owner] = x.ownerToWallets[owner] || []
    x.ownerToWallets[owner].push(x.lastWallet)
  } else {
    const wallet = line.substr(0, 42)
    x.lastWallet = wallet
  }
  return x;
}, {
  lastWallet: null,
  walletToOwners: {},
  ownerToWallets: {}
})

fs.writeFileSync('owner2wallet.json', JSON.stringify(ownerToWallets, null, 2), 'utf8')
fs.writeFileSync('wallet2owner.json', JSON.stringify(walletToOwners, null, 2), 'utf8')

const HDWalletProvider = require("truffle-hdwallet-provider");

// add your Oasis Devnet mnemonic here. Be sure to keep it secret!
const MNEMONIC = '';

// mnemonic for Contract Kit local blockchain
const CONTRACT_KIT_MNEMONIC = 'range drive remove bleak mule satisfy mandate east lion minimum unfold ready';

module.exports = {
  networks: {
    // Oasis Devnet
    oasis: {
      provider: function () {
        return new HDWalletProvider(MNEMONIC, "https://web3.oasiscloud.io");
      },
      provider: function () {
        return new HDWalletProvider(MNEMONIC, "https://web3.oasiscloud.io", 0, 2);
      },
      network_id: "42261",
      gasPrice: "0x3b9aca00"
    },

    // Contract Kit local chain
    development: {
      provider: function () {
        return new HDWalletProvider(CONTRACT_KIT_MNEMONIC, "http://localhost:8545", 0, 10);
      },
      network_id: "*"
    },
  },
  compilers: {
    external: {
      command: "./node_modules/.bin/oasis-compile",
      targets: [{
        path: "./.oasis-build/*.json"
      }]
    }
  }
};

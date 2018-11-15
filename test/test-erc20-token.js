const TokenContract = artifacts.require("TokenContract");
const Web3 = require("web3");
const web3 = new Web3(TokenContract.web3.currentProvider);

contract("TokenContract", (accounts) => {
  const instance = new web3.eth.Contract(TokenContract.abi, TokenContract.address, {
    from: accounts[0]
  });

  it("should have a balance of 100", async () => {
    const balance = await instance.methods.balanceOf(accounts[0]).call();

    assert.equal(balance, 100);
  });

  it("should transfer balance of 16 yielding (84, 16)", async () => {
    await instance.methods.transfer(accounts[1], 16).send();
    const balance0 = await instance.methods.balanceOf(accounts[0]).call();
    const balance1= await instance.methods.balanceOf(accounts[1]).call();

    assert.equal(balance0, 84);
    assert.equal(balance1, 16);
  })
})

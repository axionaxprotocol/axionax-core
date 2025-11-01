// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
contract AXXToken {
    string public name = "Axionax"; string public symbol = "AXX"; uint8 public decimals = 18;
    uint256 public totalSupply; mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance;
    event Transfer(address indexed from,address indexed to,uint256 value); event Approval(address indexed owner,address indexed spender,uint256 value);
    constructor(uint256 initialSupply){ totalSupply=initialSupply; balanceOf[msg.sender]=initialSupply; emit Transfer(address(0), msg.sender, initialSupply); }
    function approve(address s,uint256 v) external returns(bool){ allowance[msg.sender][s]=v; emit Approval(msg.sender,s,v); return true; }
    function transfer(address to,uint256 v) external returns(bool){ require(balanceOf[msg.sender]>=v,"bal"); unchecked{balanceOf[msg.sender]-=v;} balanceOf[to]+=v; emit Transfer(msg.sender,to,v); return true; }
    function transferFrom(address f,address t,uint256 v) external returns(bool){ uint256 a=allowance[f][msg.sender]; require(a>=v,"allow"); require(balanceOf[f]>=v,"bal"); unchecked{allowance[f][msg.sender]=a-v; balanceOf[f]-=v;} balanceOf[t]+=v; emit Transfer(f,t,v); return true; }
}

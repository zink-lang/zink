// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Storage} from "../src/Storage.sol";

contract CounterTest is Test {
    Storage public storagee;

    function setUp() public {
        storagee = new Storage();

    }
    function testSetValue() public {
        uint256 value = 42;
        storagee.setValue(value);
        assertEq(storagee.getValue(), value);
    }
    function testGetValue() public {
        uint256 value = 42;
        storagee.setValue(value);
        assertEq(storagee.getValue(), value);
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Storage} from "../src/Storage.sol";

contract CounterScript is Script {
    Storage public storagee;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        storagee = new Storage();

        vm.stopBroadcast();
    }
}

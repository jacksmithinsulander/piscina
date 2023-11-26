import { Rpc } from '../modules/Rpc.js'

const demo: Rpc = new Rpc("PulseChain");
const demo2: string = demo.node;
const demo3: string = new Rpc("Bsc").node;

console.log(demo);
console.log(demo2);
console.log(demo3);
console.log(demo.network);
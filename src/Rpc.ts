class Rpc {
    public network: string;
    constructor(network: string) {
        this.network = network;
    }

    get node() {
        return "hello";
    }
}

const demo: Rpc = new Rpc("PulseChain");
const demo2: string = demo.node;
const demo3: string = new Rpc("Bsc").node;

console.log(demo);
console.log(demo2);
console.log(demo3);
console.log(demo.network);
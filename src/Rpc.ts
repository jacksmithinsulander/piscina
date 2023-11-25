class Rpc {
    public network: string;
    constructor(network: string) {
        this.network = network;
    }
}

const demo: Rpc = new Rpc("PulseChain");

console.log(demo);
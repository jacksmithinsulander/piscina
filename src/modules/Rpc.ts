class Rpc {
    public network: string;
    constructor(network: string) {
        this.network = network;
    }

    get node() {
        return "hello";
    }
}

export {Rpc};

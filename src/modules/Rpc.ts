import rpcList from '../data/rpcList';


interface RpcIndex {
    [network: string]: number;
}

class Rpc {
    public network: string;
    private rpcIndex: RpcIndex = {};

    constructor(network: string) {
        this.network = network;

        if (!this.rpcIndex[network]) {
            this.rpcIndex[network] = 0;
        }
    }
    
    get node() {
        const links: string[] = rpcList[this.network];
        const index: number = this.rpcIndex[this.network];
        this.rpcIndex[this.network] += 1;
        return links[index];
    }
}

export {Rpc};

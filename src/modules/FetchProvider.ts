import { Abi, Fixed, ZeroHexFixed, ZeroHexString } from "@hazae41/cubane"
import { RpcCounter, RpcRequestPreinit, RpcResponse } from "@hazae41/jsonrpc"

interface Callable<I extends readonly Abi.Factory[], O extends Abi.Factory> {
    readonly input: Abi.FunctionSignatureFactory<I>
    readonly output: O
}
  
  
class FetchProvider {

    readonly counter = new RpcCounter()
  
    constructor(
        readonly info: RequestInfo,
        readonly init: RequestInit = {}
    ) { }
  
    async request<T>(data: RpcRequestPreinit) {
        const { method = "POST", ...rest } = this.init
  
        const body = JSON.stringify(this.counter.prepare(data))
  
        const request = new Request(this.info, { method, body, ...rest })
        request.headers.set("Content-Type", "application/json")
  
        const response = await fetch(request)
  
        if (!response.ok)
            throw new Error(response.statusText)
  
        return RpcResponse.from<T>(await response.json())
    }
  
    async call<I extends readonly Abi.Factory[], O extends Abi.Factory>(address: ZeroHexString, callable: Callable<I, O>, ...args: Abi.Factory.Froms<I>) {
        const { input, output } = callable
  
        const response = await this.request<ZeroHexString>({
            method: "eth_call",
            params: [{
                to: address,
                data: Abi.encodeOrThrow(input.from(...args))
            }, "pending"]
        }).then(r => r.unwrap())
        return Abi.decodeOrThrow(output, response).intoOrThrow() as Abi.Factory.Into<O>
    }
    
    async callWithBlock<I extends readonly Abi.Factory[], O extends Abi.Factory>(
        address: ZeroHexString,
        callable: Callable<I, O>,
        blockNumber: number | string,
        ...args: Abi.Factory.Froms<I>
    ) {
        const { input, output } = callable;
    
        const response = await this.request<ZeroHexString>({
            method: "eth_call",
            params: [
                {
                    to: address,
                    data: Abi.encodeOrThrow(input.from(...args)),
                    blockNumber,
                },
            ],
        }).then((r) => r.unwrap());
    
        return Abi.decodeOrThrow(output, response).intoOrThrow() as Abi.Factory.Into<O>;
    }
    
    async getBlockNumber() {
        return await this.request<ZeroHexString>({
            method: "eth_blockNumber"
        }).then(r => r.mapSync(BigInt).unwrap())
    }

    async getBalance(address: ZeroHexString) {
        const response = await this.request<ZeroHexString>({
            method: "eth_getBalance",
            params: [address, "latest"]
        }).then(r => r.unwrap())
      
        return Fixed.fromJSON(new ZeroHexFixed(response, 18))
    }
}

export { FetchProvider };
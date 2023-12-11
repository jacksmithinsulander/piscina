import { Abi, Fixed, ZeroHexFixed, ZeroHexString } from "@hazae41/cubane"
import { RpcCounter, RpcRequestPreinit, RpcResponse } from "@hazae41/jsonrpc"

/**
 * Represents a callable function signature.
 * @template I - Array of input Abi factories.
 * @template O - Output Abi factory.
 */
interface Callable<I extends readonly Abi.Factory[], O extends Abi.Factory> {
    readonly input: Abi.FunctionSignatureFactory<I>
    readonly output: O
}

/**
 * Provides methods to make JSON-RPC requests to an Ethereum node.
 */
class FetchProvider {
    /**
     * Counter to manage RPC requests.
     */
    readonly counter = new RpcCounter()

    /**
     * Constructs a FetchProvider.
     * @param {RequestInfo} info - The URL or Request object.
     * @param {RequestInit} [init={}] - Optional RequestInit object.
     */
    constructor(
        readonly info: RequestInfo,
        readonly init: RequestInit = {}
    ) { }

    /**
     * Sends a JSON-RPC request.
     * @template T - Response type.
     * @param {RpcRequestPreinit} data - The RPC request data.
     * @returns {Promise<RpcResponse<T>>} The RPC response.
     */
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

    /**
     * Calls a smart contract function using eth_call.
     * @template I - Array of input Abi factories.
     * @template O - Output Abi factory.
     * @param {ZeroHexString} address - The contract address.
     * @param {Callable<I, O>} callable - The callable function.
     * @param {...Abi.Factory.Froms<I>} args - Function arguments.
     * @returns {Promise<Abi.Factory.Into<O>>} The decoded result.
     */
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

    /**
     * Calls a smart contract function with a specific block number using eth_call.
     * @template I - Array of input Abi factories.
     * @template O - Output Abi factory.
     * @param {ZeroHexString} address - The contract address.
     * @param {Callable<I, O>} callable - The callable function.
     * @param {number | string} blockNumber - The block number.
     * @param {...Abi.Factory.Froms<I>} args - Function arguments.
     * @returns {Promise<Abi.Factory.Into<O>>} The decoded result.
     */
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

    /**
     * Retrieves the current block number.
     * @returns {Promise<bigint>} The current block number.
     */
    async getBlockNumber() {
        return await this.request<ZeroHexString>({
            method: "eth_blockNumber"
        }).then(r => r.mapSync(BigInt).unwrap())
    }

    /**
     * Retrieves the balance of an Ethereum address.
     * @param {ZeroHexString} address - The Ethereum address.
     * @returns {Promise<Fixed>} The balance as a Fixed object.
     */
    async getBalance(address: ZeroHexString) {
        const response = await this.request<ZeroHexString>({
            method: "eth_getBalance",
            params: [address, "latest"]
        }).then(r => r.unwrap())
      
        return Fixed.fromJSON(new ZeroHexFixed(response, 18))
    }
}

export { FetchProvider };
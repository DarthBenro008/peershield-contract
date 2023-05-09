# Peershield

## What is PeerShield?
PeerShield is a peer-to-peer risk management marketplace for digital assets enabling users to share risks related to DeFi. The users can purchase cover products that protect against different kinds of risk or provide liquidity to the capital pool that covers the risks to earn yield on their assets.

The protocol is powered by Cosmwasm and designed to provide the infrastructure for users to buy cover, underwrite risk, assess claims, and build risk management businesses.

The protocol is managed by a DAO and DeFi users can participate in the protocol as underwriters, claim assessors or propose new risk coverage products by acquiring the DAO token.


## Inspiration
The DeFi summer of 2020 provided validation to the crypto industry beyond mere speculation. While the DeFi craze brought billions of dollars into the industry but the risk associated along with it increased significantly too. In DeFi alone, ~$6.5B worth of exploits have occured ([source: DeFiLlama](https://defillama.com/hacks)). In traditional finance, Insurance products have proved to be leaning pole for the users looking to reduce the risk and naturally making Insurance one of the most profitable verticals.

The first wave of innovation in DeFi focused mainly on two fundamental financial primitives: decentralized exchanges and lending. These two domains account for the vast bulk of the value locked in DeFi protocols, totalling USD 33 billion dollars in TVL, according to DeFi Llama. In contrast, DeFi insurance accounts for only USD 343 million dollars in TVL, despite significant advances in this segment of the industry. DeFi insurance makes up less than 1% of total TVL in DeFi. Before investing large sums of money in this market, investors may desire a sense of security, and the entire Web3 economy is currently underinsured.


## Why build within Cosmos ecosystem?

### Technological Superiority

The Cosmos ecosystem offers a wide array of advantages such as sovereignty, interoperability, rate limiting and fast finality but the most interesting aspects for PeerShield include:
- Building custom modules that can be plugged into multiple Cosmos-based chains through concepts such as outposts. This will reduce a lot of redundant efforts of deploying on multiple chains.
- One of the key things for a risk management protocol is to support the DeFi platforms with well-balanced risk profile and risk mitigation practices in Cosmos, such as rate limiting on Osmosis, present a compelling case.
- Use-case specific app-chains: Having the flexibility to migrate to an app-chain model once there is considerable traction and use-case requirement is a major advantage to build in Cosmos. The barriers to bootstrap the economic security on your own are also reducing with security sharing models such as ICS and Mesh security.

### Opportunity
Since the introduction of IBC in 2021, the Cosmos DeFi has grown at rapidly but the risks for the users have shadowed the growth constantly:

- [Osmosis exploited](https://decrypt.co/102300/cosmos-based-defi-exchange-osmosis-hit-by-5m-exploit) for USD 5M in June 2022
- Terra blow up draining majority of the liquidity in the ecosystem - Osmosis DEX lost more than 75% of it's TVL post UST collapse
- As of today, more than 10% of the liquidity on Osmosis is comprised of bridged assets (USDC, WETH, WBTC, DAI through Axelar). The case is very similar for other leading DeFi protocols in the Cosmos ecosystem such as Umee and Mars Protocol. Bridge exploits are familiar for every DeFi users and have high probability considering the complexity of bridging mechanisms.
- In absence of any concrete risk management protocol, the users have significantly less to no chance to mitigate or minimise the risks of participating in the Cosmos DeFi.

## What it does?
For the scope of v1 and effectively bootstrap the protocol, we are limiting the offering to LP coverage.

Current risks involved with liquidity provisioning on a DEX, for example Osmosis:

- The pool may be drained in an exploit such as Osmosis June 2022 exploit for USD 5M.
- Pools with the bridged assets, such as axlUSDC, are at far higher risk because of the bridge exploit.
- Impermanent loss: DEXes in Cosmos require the users to lock their assets for a certain period (Osmosis: 14 days) and in volatile markets this can result in significant impermanent loss. If the user withdraws their assets during the price fluctuations, they realise impermanent loss which can be back-breaking for smaller users.

The Peershield v1 will allow users to buy coverage for their risks involved with liquidity provisioning and in return will have to pay part of their LP rewards as premiums.

## Challenges we ran into
- Cosmos Techstack On-boarding: Most of the team has background in building solutions on EVM chains and kickstarting development on the Cosmos techstack was a little painful majorly due to lack of coherent  and comprehensive resources.
- Time Management: We wanted to build a PoC that covers the in-protocol interactions end-to-end but majority of our time went into research and mechanism flows leaving little time for PoC.
- Complex Calculations: A risk management marketplace has a lot of calculations, such as Premium Pricing, Minimum Capital Requirement for Capital Pool, that require much longer than we anticipated and additional experitise which we weren't able to aquire in the span of Phase 1. But hoping to deal with it as we progress.

## Accomplishments that we're proud of
- Learning about Cosmos techstack in less than a week and interacting with multiple Osmosis tooling/frameworks
- Very comprehensive initial research that allowed us to think of different mechanisms
- v1 mechanism documentation and a semi-functional PoC

## What we learned
ALMOST EVERYTHING, haha.
- Cosmos techstack
- Working of traditional insurance
- DeFi Risk Management models
- Research Documentation - drafting IRD, BRD

## What's next for PeerShield
- Implementation of end-to-end PoC with basic functionalities
- Hashing out the maths for the protocol
- Deploying on the Osmosis testnet
- Experimenting with interchain services such as ICA 

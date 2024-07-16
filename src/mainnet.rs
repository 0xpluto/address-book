use alloy_primitives::{Address, FixedBytes};

/// 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2
pub const WETH: Address = Address(FixedBytes([
    192, 42, 170, 57, 178, 35, 254, 141, 10, 14, 92, 79, 39, 234, 217, 8, 60, 117, 108, 194,
]));

/// 0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599
pub const WBTC: Address = Address(FixedBytes([
    34, 96, 250, 197, 229, 84, 42, 119, 58, 164, 79, 188, 254, 223, 124, 25, 59, 194, 197, 153,
]));

/// 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
pub const USDC: Address = Address(FixedBytes([
    160, 184, 105, 145, 198, 33, 139, 54, 193, 209, 157, 74, 46, 158, 176, 206, 54, 6, 235, 72,
]));

/// 0xdAC17F958D2ee523a2206206994597C13D831ec7
pub const USDT: Address = Address(FixedBytes([
    218, 193, 127, 149, 141, 46, 229, 35, 162, 32, 98, 6, 153, 69, 151, 193, 61, 131, 30, 199,
]));

/// 0x6B175474E89094C44Da98b954EedeAC495271d0F
pub const DAI: Address = Address(FixedBytes([
    107, 23, 84, 116, 232, 144, 148, 196, 77, 169, 139, 149, 78, 237, 234, 196, 149, 39, 29, 15,
]));  

/// 0x00008110c8c2a752903249537ff625a489c25000
pub const SPRM: Address = Address(FixedBytes([
    0, 0, 8, 17, 12, 140, 42, 117, 41, 3, 36, 149, 55, 255, 98, 90, 72, 156, 37, 0,
]));

/// 0x7a250d5630b4cf539739df2c5dacb4c659f2488d
pub const UNISWAP_V2_ROUTER: Address = Address(FixedBytes([
    122, 37, 13, 86, 48, 180, 207, 83, 151, 57, 223, 44, 93, 172, 180, 198, 89, 242, 72, 141,
]));

/// 0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f
pub const UNISWAP_V2_FACTORY: Address = Address(FixedBytes([
    92, 105, 190, 231, 1, 239, 129, 74, 43, 106, 62, 221, 75, 22, 82, 203, 156, 197, 170, 111,
]));

pub const SUSHISWAP_V2_FACTORY: Address = Address(FixedBytes([
    192, 174, 228, 120, 227, 101, 142, 38, 16, 197, 247, 164, 162, 225, 119, 124, 233, 228, 242,
    172,
]));


/// 0xe592427a0aece92de3edee1f18e0157c05861564
pub const UNISWAP_V3_ROUTER: Address = Address(FixedBytes([
    229, 146, 66, 122, 10, 236, 233, 45, 227, 237, 238, 31, 24, 224, 21, 124, 5, 134, 21, 100,
]));

/// 0x1f98431c8ad98523631ae4a59f267346ea31f984
pub const UNISWAP_V3_FACTORY: Address = Address(FixedBytes([
    31, 152, 67, 28, 138, 217, 133, 35, 99, 26, 228, 165, 159, 38, 115, 70, 234, 49, 249, 132,
]));

/// 0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc
pub const WETH_USDC_UNI_V2: Address = Address(FixedBytes([
    180, 225, 109, 1, 104, 229, 45, 53, 202, 205, 44, 97, 133, 180, 66, 129, 236, 40, 201, 220,
]));

/// 0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852
pub const WETH_USDT_UNI_V2: Address = Address(FixedBytes([
    13, 74, 17, 213, 238, 170, 194, 142, 195, 246, 29, 16, 13, 175, 77, 64, 71, 31, 24, 82,
]));

/// 0x853d955aCEf822Db058eb8505911ED77F175b99e
pub const FRAX: Address = Address(FixedBytes([
    133, 61, 149, 90, 206, 248, 34, 219, 5, 142, 184, 80, 89, 17, 237, 119, 241, 117, 185, 158,
]));

/// 0x0C10bF8FcB7Bf5412187A595ab97a3609160b5c6
pub const USDD: Address = Address(FixedBytes([
    12, 16, 191, 143, 203, 123, 245, 65, 33, 135, 165, 149, 171, 151, 163, 96, 91, 96, 181, 198,
]));

/// 0x57Ab1ec28D129707052df4dF418D58a2D46d5f51
pub const SUSD: Address = Address(FixedBytes([
    87, 171, 30, 194, 141, 18, 151, 7, 5, 45, 244, 223, 65, 141, 88, 162, 212, 109, 95, 81,
]));

/// 0x5f98805A4E8be255a32880FDeC7F6728C6568bA0
pub const LUSD: Address = Address(FixedBytes([
    95, 152, 128, 90, 78, 139, 226, 85, 163, 40, 128, 253, 236, 127, 103, 40, 198, 86, 139, 160,
]));

/// 0x8D6CeBD76f18E1558D4DB88138e2DeFB3909fAD6
pub const MAI: Address = Address(FixedBytes([
    141, 108, 235, 215, 111, 24, 225, 85, 141, 77, 184, 129, 56, 226, 222, 251, 57, 9, 250, 214,
]));

/// 0x9E32b13ce7f2E80A01932B42553652E053D6ed8e
pub const METIS: Address = Address(FixedBytes([
    158, 50, 177, 60, 231, 242, 232, 10, 1, 147, 43, 66, 85, 54, 82, 224, 83, 214, 237, 142,
]));
<a name="v0.7.0"></a>
## v0.7.0 (2019-12-01)


#### Breaking Changes

*   Rename space to line and newline to hardline ([a011c1b0](https://github.com/Marwes/pretty.rs/commit/a011c1b05d26257c5b529eaa973073d94522225c), breaks [#](https://github.com/Marwes/pretty.rs/issues/))

#### Features

*   Add RcDoc ([f7c675fc](https://github.com/Marwes/pretty.rs/commit/f7c675fc9d6eb2142f71033cffa4bfac5749b1bc))
*   Add convenience combinators for enclosing documents ([5358b6ed](https://github.com/Marwes/pretty.rs/commit/5358b6edd562b082ec9f1f89503f2eea7f2f8aaa))
*   Add softline ([3802a856](https://github.com/Marwes/pretty.rs/commit/3802a8566c51ec2819ab485b194d0e14f6ccc1c0))
*   Rename space to line and newline to hardline ([a011c1b0](https://github.com/Marwes/pretty.rs/commit/a011c1b05d26257c5b529eaa973073d94522225c), breaks [#](https://github.com/Marwes/pretty.rs/issues/))
*   Add nesting and align ([713f5a98](https://github.com/Marwes/pretty.rs/commit/713f5a984d8bea0afa0f82af92b0b5148f853b4b))
*   Add the width document ([927583e9](https://github.com/Marwes/pretty.rs/commit/927583e948f979966626d0819acffbba1522acda))
*   Introduce the Column document ([f78cd2ea](https://github.com/Marwes/pretty.rs/commit/f78cd2ea9b366a9d2d1ae1a2fd93adf1807ff20b))

#### Bug Fixes

*   Allow usize::max_value as a width ([340f6685](https://github.com/Marwes/pretty.rs/commit/340f6685ca08cd7adaee1599efaec8b4b403c137), closes [#53](https://github.com/Marwes/pretty.rs/issues/53))



<a name="v0.5.0"></a>
## v0.5.0 (2018-06-16)


#### Breaking Changes

*   Change the type parameter order so attributes can be defaulted ([ba08cedc](https://github.com/Marwes/pretty.rs/commit/ba08cedcdfe2ce117d757ab5bc0fcfb4d2a7a6b6), breaks [#](https://github.com/Marwes/pretty.rs/issues/))

#### Features

*   Allow custom attributes to be rendered ([07c8ac03](https://github.com/Marwes/pretty.rs/commit/07c8ac03178c00a3d28a02b7395701b59d6abe4d))
*   Permit newlines in text documents ([d11ad4be](https://github.com/Marwes/pretty.rs/commit/d11ad4bee656f67fba42fcc50988d7aa7a271a7e))

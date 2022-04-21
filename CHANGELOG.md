<a name="v0.11.3"></a>
### v0.11.3 (2022-04-21)




<a name="v0.11.2"></a>
### v0.11.2 (2021-12-15)


#### Features

*   Implement Add/AddAssign on DocBuilder ([45e085a4](https://github.com/Marwes/pretty.rs/commit/45e085a4bd5737c66be8654ab30dae2fd6aa5a08))



<a name="v0.11.1"></a>
### v0.11.1 (2021-12-10)


#### Bug Fixes

*   Allow &String in the docs! macro again ([e02bdc8f](https://github.com/Marwes/pretty.rs/commit/e02bdc8f24f251a496529fc79fcb243da816ddee))



<a name="v0.11.0"></a>
## v0.11.0 (2021-12-10)


#### Features

*   Use display width during rendering ([b88f123a](https://github.com/Marwes/pretty.rs/commit/b88f123a3dba6f2f269d4c9f38961765238983a8), closes [#67](https://github.com/Marwes/pretty.rs/issues/67))
*   Introduce the Pretty trait ([84a41d3d](https://github.com/Marwes/pretty.rs/commit/84a41d3daf2adfb6d3d8aa5806e12ff26bd4e3b7))
*   Implement Deref for DocBuilder ([97602f36](https://github.com/Marwes/pretty.rs/commit/97602f36aff8c0a3a5895a9d6348fea424a5fb2c))
*   Implement Debug on DocBuilder ([05bc8b76](https://github.com/Marwes/pretty.rs/commit/05bc8b76e05fc8dd0fbd425987c5597dab8a860c))
*   Debug print Line documents ([063052d5](https://github.com/Marwes/pretty.rs/commit/063052d5ac4ef484ab3988547fa373824ed975fd))
*   Debug print sotfline_ docs in a shortform ([b64b8b37](https://github.com/Marwes/pretty.rs/commit/b64b8b37f5cb23830ce37ce0ee475af5a28998ed))
*   Debug print sotfline docs in a shortform ([86915fea](https://github.com/Marwes/pretty.rs/commit/86915fea79ddcb02f1cc32f6f23229a3dbd9fee5))

#### Performance

*   convert empty text to Nil docs ([91931342](https://github.com/Marwes/pretty.rs/commit/91931342315c0500f93cfbf3f4f97f774d2192ae))
*   No need to group on individual text components ([9a67247b](https://github.com/Marwes/pretty.rs/commit/9a67247b725380607d315e8117cf757eca5f0b82))



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

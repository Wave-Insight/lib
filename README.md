# wave-viewer

**[Documentation](https://docs.rs/wave-viewer)** | **[Changelog](https://github.com/zao111222333/wave-viewer/releases)**

<!-- [![Build Status](https://travis-ci.org/kevinmehall/rust-vcd.wave-viewer?branch=master)](https://travis-ci.org/kevinmehall/rust-vcd) -->

This crate parsers [VCD (Value Change Dump)][wp] files, a common format used with logic analyzers, HDL simulators, and other EDA tools.

[wp]: https://en.wikipedia.org/wiki/Value_change_dump

## TODO
- [x] Define `structure::Structure`
- [x] Implement `Deserialize`&`Serialize` for `structure::wire::Wire`&`structure::module::Module`
- [ ] Parser `.vcd` files
- [ ] `Websocket`, `wasm`
- [ ] Canvas line-chart
- [ ] Data Format: `TureFalse`, `Binary`, `Octonary`, `Hexadecimal`, 
                   `Decimal`, `Signed Decimal`, 
                   `FP16`, `FP32`, `FP64`,
                   `BF16`, `TF32`,
                   `RISCV-32i`, `ARM32/64`,
                   `RGB`, `AXI`, `etc.`

        1. 可以编辑(文本框更改)多bit信号线的取值范围, 将一个signal拆成多个操作(单独设置Data Format)
        2. 数据显示成简单形式, 鼠标在某个数据停留时会显示复杂模式(像vscode会显示小框), eg. RGB格式缩写:#a23d4d, 复杂模式显示RGB(0-255)和调色盘中位置, 浮点数可以显示sign/exp/mant分别是多少
        3. 多根信号线bunch的定义, 多信号mapping多信号(bit-bit映射), bunch显示为1个单独的有明确含义的信号线并可以展开看每一个元素的情况, bunch可以继续组bunch
        4. 用户可以自定义数据格式

- [ ] 实现一个类shell的交互环境, 可以使用指令`cd`, `ls`, `add xx(signal/module)`, `set_format xx(signal)`, `help`, `etc.`, 可以使用`-f file.wish` (Wave Insight Shell)指令预加载, 可以将
,类似`PyMOL`的`script`https://pymolwiki.org/index.php/Simple_Scripting
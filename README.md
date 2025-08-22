# EasyDB APP

EasyDB 是一个由 Rust 编写的 SQL 助手，旨在简化文本文件查询过程。通过 EasyDB，你可以将多个文本文件视为一个数据库，并使用 SQL 进行查询。它支持多种文件格式，包括 CSV、NdJson、JSON、xlsx 和 Parquet 文件，无需进行文件转换，开箱即用。

之前的 [EasyDB](https://github.com/shencangsheng/easy_db) 更多用在 Linux 服务器中来提供大规模文本文件查询服务，虽然提供 Docker 部署，但使用起来还是不够方便。现在提供 MacOS/Windows 平台下的客户端版本，方便个人使用。

为此更好的区分两个项目，将此项目命名为 EasyDB APP 由 Tauri 构建，而 [EasyDB](https://github.com/shencangsheng/easy_db) 命名为 EasyDB Server。

与 Server 端不同，EasyDB APP 客户端在查询引擎上由 `Datafusion` 切换为 `pola-rs/polars`。`polars` 具备更高的轻量性和流式计算能力，显著降低了内存占用，更加适合个人电脑。

## 📖 功能

- 支持 CSV、NdJson、JSON、xlsx 和 Parquet 文件
- 使用标准 SQL 语句对文件数据进行查询

## 👍 依赖库

这些开源库用于创建本项目。

- [pola-rs/polars](https://github.com/pola-rs/polars)

## 📝 许可证

A short snippet describing the license (MIT)

MIT © Cangsheng Shen

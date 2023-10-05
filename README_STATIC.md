# OpenDigger CLI

本项目为 OpenSODA 挑战赛参赛项目。 旨在为 [OpenDigger](https://github.com/X-lab2017/open-digger) 提供的各类统计型和网络型的指标提供统一的 Terminal User Interface (TUI)。

**该文档为静态演示版本，为了更好的阅读体验，请访问 [README.md](./README.md)**。

## **本项目的主要特性**

* 🚀 基于 ChatGPT 的 **自然语言化查询** 功能
* 🚀 针对不同类型指标的 **SQL 和 Cypher** 查询操作
* 🚀 丰富的 **交互式 UI** 展示功能
* 🚀 基于 Rust 开发，**支持单文件分发，无需安装依赖**

## **安装**

本项目在 Github Release 提供在 Linux 平台上的二进制文件 [[🚀链接]](https://github.com/DarkHighness/opendigger-cli/releases/tag/v0.0.1-beta)。

### **功能特性**

部分演示提供了基于 [asciinema](https://asciinema.org/) 转换得到的GIF 演示。 若图像无法正常加载，请尝试

#### **基于 ChatGPT 的自然语言化查询**

本工具为底层的统计型和网络型的指标提供了统一的 SQL 和 Cypher 查询表示。 SQL 和 Cypher 为后续的查询和处理操作提供了极大的灵活性，但对于非专业人士来说，SQL 和 Cypher 查询语言的学习成本较高。 **本工具提供了基于 ChatGPT 的自然语言化查询功能，用户可以通过自然语言的方式查询指标数据。**

> 注：
>   使用该功能需要手动配置 OPENAI_API_KEY 环境变量，具体配置方法请参考 [OpenAI API](https://platform.openai.com/docs/quickstart)。
> 

可能的示例如下:

1. 对比 `X-lab2017/open-digger` 和 `vuejs/core` 仓库的 star 数量变化

```bash
$ opendigger-cli chat "仓库‘X-lab2017/open-digger’,'vuejs/core' stars 数量的变化, 保留仓库名称" --ui
```

![chat query 1](https://github.com/DarkHighness/opendigger-cli/blob/main/doc/assets/chat_query_1.png?raw=true)

> 注：
>   本工具支持中文输入，但 asciinema 中录制中文字符存在乱码，故演示中使用英文输入。

2. `X-lab2017/open-digger` 在哪个月获得了最多的 star？

```bash
$ opendigger-cli chat "仓库‘X-lab2017/open-digger’ 在哪一个月获得了最多的 star" --ui
```

![chat query 2](https://github.com/DarkHighness/opendigger-cli/blob/main/doc/assets/chat_query_2.png?raw=true)

#### **SQL 和 Cypher 查询**

本工具提供了基于 SQL 和 Cypher 的查询功能，用户可以通过 SQL 和 Cypher 查询指标数据。 查询结果支持多种展示方式，并支持管道操作符。 

支持的 SQL 操作包括：`SELECT`, `JOIN`, `WHERE`, `GROUPBY`, `ORDERBY`, `LIMIT`, `OFFSET` 等。 不支持任何形式的 DML 或 DDL 操作。 

支持切换图表类型： `Treading`(历史数据的前缀和) 和 `Value`（原始数据）

一些可能的示例如下:

**SQL 查询**

1. 查询所有可用的关系表

```bash
$ opendigger-cli sql "SHOW TABLES" --ui
```

![show tables](https://github.com/DarkHighness/opendigger-cli/blob/main/doc/assets/sql_query_show_tables.png?raw=true)

2. 查询 `Issues` 表中的所有列

```bash
$ opendigger-cli sql "SHOW COLUMNS FROM Issues"
```

![show columns](https://github.com/DarkHighness/opendigger-cli/blob/main/doc/assets/sql_query_show_columns.png?raw=true)

3. 查询 `X-lab2017/open-digger` 仓库在 `Openrank` 指标上的数据

```bash
$ opendigger-cli sql "SELECT * FROM Openrank WHERE name = 'X-lab2017/open-digger'" --ui
```

![basic query 1](https://github.com/DarkHighness/opendigger-cli/blob/main/doc/assets/basic_query_1.png?raw=true)

4. 查询 `X-lab2017/open-digger` 仓库在 `Openrank` 指标上的数据，按 `OpenRank` 值排序后，并将结果写入文件

```bash
$ opendigger-cli sql "SELECT * FROM Openrank WHERE name = 'X-lab2017/open-digger'" -o openrank.csv > /dev/null
```

![basic query 2](https://github.com/DarkHighness/opendigger-cli/blob/main/doc/assets/basic_query_2.png?raw=true)

5. 查询 `X-lab2017/open-digger` 和 `vuejs/core` 仓库在 `Stars` 指标上的数据, 根据 `month` 进行连接，筛选出 star 之和大于 100 的月份

```bash
$ opendigger-cli sql "SELECT * FROM Stars a LEFT JOIN Stars b ON a.month = b.month WHERE a.name = 'X-lab2017/open-digger' AND b.name = 'vuejs/core' AND a.value + b.value > 100" --ui
```

![basic query 3](https://github.com/DarkHighness/opendigger-cli/blob/main/doc/assets/basic_query_3.png?raw=true)

6. 查询 `X-lab2017/open-digger`, `facebook/react` 和 `vuejs/core` 仓库在 `Stars` 指标上的数据, 交互式 UI 展示

```bash
$ opendigger-cli sql "SELECT * FROM Stars WHERE name = 'X-lab2017/open-digger' OR name = 'facebook/react' OR name = 'vuejs/core'" --ui
```

![basic query 4](https://github.com/DarkHighness/opendigger-cli/blob/main/doc/assets/basic_query_4.png?raw=true)

**Cypher 查询**

1. 图查询： 查询用户'will-ww'在repo='X-lab2017/open-digger'上的'Developer network'指标数据，使用TUI展示数据值。

```bash
$ opendigger-cli cypher "MATCH (n:Node {value: 'will-ww'}) WHERE n.owner = 'X-lab2017/open-digger' AND n.metric = 'Developer network' RETURN n"
```

![cypher query 1](https://github.com/DarkHighness/opendigger-cli/blob/main/doc/assets/cypher_1.png?raw=true)

2. 图查询：查询用户'Zzzzzhuzhiwei'在repo='X-lab2017/open-digger'上的'Developer network'上的所有有合作关系或者是有相关性的用户。

```bash
$ opendigger-cli cypher "MATCH (n:Node {value: 'Zzzzzhuzhiwei'})-[]-(neighbor) where n.owner='X-lab2017/open-digger' and n.metric='Developer network' RETURN neighbor"
```

![cypher query 2](https://github.com/DarkHighness/opendigger-cli/blob/main/doc/assets/cypher_2.png?raw=true)

3. 图查询： 查询用户'Zzzzzhuzhiwei'在repo='X-lab2017/open-digger'上的'Developer network'上的合作最紧密的用户。

```bash
$ opendigger-cli cypher "MATCH (n:Node {value: 'Zzzzzhuzhiwei'})-[]->(neighbor) WHERE n.owner='X-lab2017/open-digger' and n.metric='Developer network' WITH neighbor ORDER BY neighbor.metric DESC LIMIT 1 RETURN neighbor"
```

![cypher query 3](https://github.com/DarkHighness/opendigger-cli/blob/main/doc/assets/cypher_2.png?raw=true)

#### **原始数据下载**

本工具提供了原始数据下载功能，用户可以通过本工具下载指标数据，并保存为 json 或 csv 格式的文件。

一种可能的示例如下:

下载 `X-lab2017/open-digger` 仓库在 `Openrank` 指标上的数据

```bash
$ opendigger-cli download "X-lab2017/open-digger" "openrank" -o openrank.json
```

![download](https://github.com/DarkHighness/opendigger-cli/blob/main/doc/assets/download_command.png?raw=true)

#### **整体报告**

本工具提供了整体报告功能，用户可以通过本工具查询仓库的整体报告。 报告将导出为单独的 HTML 文件。

一种可能的示例如下：

整体报告：查询仓库'X-lab2017/open-digger'的整体报告。

```bash
$ opendigger-cli report "X-lab2017/open-digger"
```

![report](https://github.com/DarkHighness/opendigger-cli/blob/main/doc/assets/report.png?raw=true)


**其他** 

其余特性请参考帮助命令：

```bash
$ opendigger-cli help

Usage: opendigger-cli <COMMAND>

Commands:
  download  Download data from the API
  sql       Query data with sql
  cypher    Query data with cypher
  chat      ChatGPT yes!
  report    Generate a report
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```


1. 基本帮助命令

```bash
$ opendigger-cli help
```

![help_command](https://raw.githubusercontent.com/DarkHighness/opendigger-cli/c1d560f41e9d71ad0af31f92b3c8ef5648a2330f/doc/assets/help_command.png)

2. 下载 `X-lab2017/open-digger` 仓库在 `Openrank` 指标上的数据

```bash
$ opendigger-cli download "X-lab2017/open-digger" "openrank" -o openrank.json
```

![download](https://raw.githubusercontent.com/DarkHighness/opendigger-cli/c1d560f41e9d71ad0af31f92b3c8ef5648a2330f/doc/assets/download_command.png)

3. 查询所有可用的关系表

```bash
$ opendigger-cli sql "SHOW TABLES"
```

![show_tables](https://raw.githubusercontent.com/DarkHighness/opendigger-cli/c1d560f41e9d71ad0af31f92b3c8ef5648a2330f/doc/assets/sql_query_show_tables.png)

4. 查询 `Issues` 表中的所有列

```bash
$ opendigger-cli sql "SHOW COLUMNS FROM Issues"
```

![show_columns](https://raw.githubusercontent.com/DarkHighness/opendigger-cli/c1d560f41e9d71ad0af31f92b3c8ef5648a2330f/doc/assets/sql_query_show_columns.png)

5. 查询 `X-lab2017/open-digger` 仓库在 `Openrank` 指标上的数据

```bash
$ opendigger-cli sql "SELECT * FROM Openrank WHERE name = 'X-lab2017/open-digger'"
```

![basic_query_1](https://raw.githubusercontent.com/DarkHighness/opendigger-cli/c1d560f41e9d71ad0af31f92b3c8ef5648a2330f/doc/assets/basic_query_1.png)

6. 查询 `X-lab2017/open-digger` 仓库在 `Openrank` 指标上的数据，并将结果写入文件

```bash
$ opendigger-cli sql "SELECT * FROM Openrank WHERE name = 'X-lab2017/open-digger'" -o openrank.csv > /dev/null
```

![basic_query_2](https://raw.githubusercontent.com/DarkHighness/opendigger-cli/c1d560f41e9d71ad0af31f92b3c8ef5648a2330f/doc/assets/basic_query_2.png)

7. 查询 `X-lab2017/open-digger` 和 `vuejs/core` 仓库在 `Stars` 指标上的数据, 根据 `month` 进行连接

```bash
$ opendigger-cli sql "SELECT * FROM Stars a LEFT JOIN Stars b ON a.month = b.month WHERE a.name = 'X-lab2017/open-digger' AND b.name = 'vuejs/core'"
```

![basic_query_3](https://raw.githubusercontent.com/DarkHighness/opendigger-cli/c1d560f41e9d71ad0af31f92b3c8ef5648a2330f/doc/assets/basic_query_3.png)

8. 查询 `X-lab2017/open-digger`, `facebook/react' 和 `vuejs/core` 仓库在 `Stars` 指标上的数据, 交互式 UI 展示

```bash
$ opendigger-cli sql "SELECT * FROM Stars WHERE name = 'X-lab2017/open-digger' OR name = 'facebook/react' OR name = 'vuejs/core'" --ui
```

>(支持切换图表类型： `Treading`(历史数据的前缀和) 和 `Value`（原始数据）)

![basic_query_4](https://raw.githubusercontent.com/DarkHighness/opendigger-cli/c1d560f41e9d71ad0af31f92b3c8ef5648a2330f/doc/assets/basic_query_4.png)

9. 图查询： 查询用户'will-ww'在repo='X-lab2017/open-digger'上的'Developer network'指标数据，使用TUI展示数据值。

```bash
$ opendigger-cli cypher "MATCH (n:Node {value: 'will-ww'}) WHERE n.owner = 'X-lab2017/open-digger' AND n.metric = 'Developer network' RETURN n"
```

![cypher_3](https://raw.githubusercontent.com/DarkHighness/opendigger-cli/c1d560f41e9d71ad0af31f92b3c8ef5648a2330f/doc/assets/cypher_3.png)

10. 图查询：查询用户'Zzzzzhuzhiwei'在repo='X-lab2017/open-digger'上的'Developer network'上的所有有合作关系或者是有相关性的用户。

```bash
$ opendigger-cli cypher "MATCH (n:Node {value: 'Zzzzzhuzhiwei'})-[]-(neighbor) where n.owner='X-lab2017/open-digger' and n.metric='Developer network' RETURN neighbor"
```

![cypher_1](https://raw.githubusercontent.com/DarkHighness/opendigger-cli/c1d560f41e9d71ad0af31f92b3c8ef5648a2330f/doc/assets/cypher_1.png)

11. 图查询： 查询用户'Zzzzzhuzhiwei'在repo='X-lab2017/open-digger'上的'Developer network'上的合作最紧密的用户。

```bash
$ opendigger-cli cypher "MATCH (n:Node {value: 'Zzzzzhuzhiwei'})-[]->(neighbor) WHERE n.owner='X-lab2017/open-digger' and n.metric='Developer network' WITH neighbor ORDER BY neighbor.metric DESC LIMIT 1 RETURN neighbor"
```

![cypher_2](https://raw.githubusercontent.com/DarkHighness/opendigger-cli/c1d560f41e9d71ad0af31f92b3c8ef5648a2330f/doc/assets/cypher_2.png)


12. ChatGPT 查询

```bash
$ opendigger-cli chat "仓库‘X-lab2017/open-digger’,'vuejs/core' stars 数量的变化, 包含仓库名称" --ui
```

>(支持切换图表类型： `Treading`(历史数据的前缀和) 和 `Value`（原始数据）)

![chat_query_1](https://raw.githubusercontent.com/DarkHighness/opendigger-cli/c1d560f41e9d71ad0af31f92b3c8ef5648a2330f/doc/assets/chat_query_1.png)

13. 整体报告：查询仓库'X-lab2017/open-digger'的整体报告。

```bash
$ opendigger-cli report "X-lab2017/open-digger"
```

![image-20230715143743718](https://raw.githubusercontent.com/DarkHighness/opendigger-cli/main/doc/assets/report.png)

## 

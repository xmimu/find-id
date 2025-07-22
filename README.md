## 📘 `find-id`

`find-id` 是一个命令行工具，用于在 Wwise `.wwu` 文件中查找包含指定 ID 的节点，支持按 `MediaID`、`GUID`、`ShortID` 三种模式搜索。

---

### 🛠️ 功能特点

* ✅ 并行处理：使用 [Rayon](https://docs.rs/rayon/) 加速多文件遍历和查找
* ✅ 多种搜索模式：支持 `MediaID`、`ID (GUID)`、`ShortID`
* ✅ 自动验证目录结构（需包含 `.wproj` 文件）
* ✅ 中文注释 + 用户友好输出格式

---

### 📦 下载

[下载地址]()

---

### ▶️ 使用方法

```bash
find-id <ID> <FOLDER_PATH> [OPTIONS]
```

#### ✅ 参数说明

| 参数              | 说明                      |
| --------------- | ----------------------- |
| `<ID>`          | 要查找的字符串，可部分匹配，不区分大小写    |
| `<FOLDER_PATH>` | 项目文件夹路径，需包含 `.wproj` 文件 |

#### ⚙️ 可选项（搜索模式）

| 选项               | 简写 | 说明                   |
| ---------------- | -- | -------------------- |
| `--mode guid`    | 无  | 按照节点属性 `ID` 进行查找（默认） |
| `--mode media-id` | 无  | 查找 `<MediaID>` 节点    |
| `--mode short-id` | 无  | 按照节点属性 `ShortID` 查找  |

---

### 🧪 示例

```bash
# 查找所有 MediaID 中包含 "abc" 的节点
find-id.exe abc ./MyProject --mode media-id

# 查找包含 ShortID 为 123456 的节点
find-id.exe 123456 ./MyProject -m short-id

# 查找所有 ID（GUID）中包含 def 的节点（默认）
find-id.exe def ./MyProject
```

---

### 🔧 输出样式示例

#### `--mode media-id`

```txt
MediaID: 2309123 | Tag: Sound | Name: Footstep | ID: {guid} | Language: SFX | AudioFile: footstep.wav
...
共匹配到 3 条结果
```

#### `--mode guid`

```txt
Tag: Event | Name: Explosion | ID: {ABCDEF-GUID} | ShortID: 234123
...
共匹配到 1 条结果
```

---

### 📚 依赖库

* [clap](https://docs.rs/clap/) - 命令行参数解析
* [glob](https://docs.rs/glob/) - 通配符文件匹配
* [rayon](https://docs.rs/rayon/) - 数据并行处理
* [roxmltree](https://docs.rs/roxmltree/) - XML 文档解析

---

### 📜 License

本项目采用 MIT 协议，欢迎自由使用与修改。

---
